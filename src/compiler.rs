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

use crate::executable::Executable;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CSourceToObject {
	pub input:            PathBuf,
	pub object_directory: PathBuf,
}

impl Executable for CSourceToObject {
	fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>> {
		let mut buf = Vec::new();
		BufReader::new(File::open(&self.input)?).read_to_end(&mut buf)?;

		let hash = format!("{:x}", xxh3_64(&buf));
		let output = self.object_directory.join(format!("{hash}.o"));

		let mut command = Command::new("clang");

		command.arg("-std=c11");
		command.arg("-Wall").arg("-Wextra").arg("-Wpedantic");

		if output.exists() {
			command.arg("-fsyntax-only");
		} else {
			command.arg("-fwrapv");
			command.arg("-c").arg("-o").arg(&output);
		}

		command.arg(&self.input);

		println!(
			"{} {}",
			command.get_program().to_string_lossy(),
			command.get_args().map(|arg| arg.to_string_lossy()).join(" ")
		);

		command.execute()
	}
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LinkObjectsToBinary {
	pub inputs: Vec<PathBuf>,
	pub output: PathBuf,
}

impl Executable for LinkObjectsToBinary {
	fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>> {
		let mut command = Command::new("clang");

		command.arg("-fuse-ld=lld");
		command.arg("-o");
		command.arg(&self.output);
		command.args(&self.inputs);

		println!(
			"{} {}",
			command.get_program().to_string_lossy(),
			command.get_args().map(|arg| arg.to_string_lossy()).join(" ")
		);

		command.execute()
	}
}
