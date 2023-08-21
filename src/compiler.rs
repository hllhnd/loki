use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use xxhash_rust::xxh3::xxh3_64;

use crate::config::Configuration;
use crate::config::Lto;
use crate::config::Optimization;
use crate::config::Standard;
use crate::executable::Executable;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CSourceToObject {
    pub configuration:    Configuration,
    pub input:            PathBuf,
    pub object_directory: PathBuf,
}

impl Executable for CSourceToObject {
    fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        let mut buf = Vec::new();
        BufReader::new(File::open(&self.input)?).read_to_end(&mut buf)?;

        let hash = format!("{:x}", xxh3_64(&buf));
        let output = self.object_directory.join(format!("{}.o", hash));

        let mut command = Command::new(&self.configuration.compiler);

        command.arg(match self.configuration.standard {
            Standard::C89 => "-std=c89",
            Standard::C99 => "-std=c99",
            Standard::C11 => "-std=c11",
            Standard::C17 => "-std=c17",
            Standard::C23 => "-std=c2x",
            Standard::Gnu89 => "-std=gnu89",
            Standard::Gnu99 => "-std=gnu99",
            Standard::Gnu11 => "-std=gnu11",
            Standard::Gnu17 => "-std=gnu17",
            Standard::Gnu23 => "-std=gnu2x",
        });

        command.arg("-Wall").arg("-Wextra").arg("-Wpedantic");

        if output.exists() {
            command.arg("-fsyntax-only");
        } else {
            command.arg("-fwrapv");

            match self.configuration.optimization.lto {
                Some(Lto::Full) => _ = command.arg("-flto=full"),
                Some(Lto::Thin) => _ = command.arg("-flto=thin"),
                _ => (),
            };

            command.arg("-c").arg("-o").arg(&output);
        }

        command.arg(&self.input);

        println!(
            "{}",
            [&[command.get_program()], &command.get_args().collect_vec()[..]]
                .concat()
                .iter()
                .map(|f| f.to_str().unwrap())
                .join(" ")
        );

        command.execute()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LinkObjectsToBinary {
    pub optimization: Optimization,
    pub inputs:       Vec<PathBuf>,
    pub output:       PathBuf,
}

impl Executable for LinkObjectsToBinary {
    fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        let mut command = Command::new("clang");

        command.arg("-fuse-ld=lld");

        match self.optimization.lto {
            Some(Lto::Full) => _ = command.arg("-flto=full"),
            Some(Lto::Thin) => _ = command.arg("-flto=thin"),
            _ => (),
        };

        command.arg("-o");
        command.arg(&self.output);
        command.args(&self.inputs);

        println!(
            "{}",
            [&[command.get_program()], &command.get_args().collect_vec()[..]]
                .concat()
                .iter()
                .map(|f| f.to_str().unwrap())
                .join(" ")
        );

        command.execute()
    }
}
