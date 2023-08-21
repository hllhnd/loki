use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use color_eyre::Report;

use crate::config::Project;

const HELLO_WORLD: &str = r#"#include <stdio.h>

int main() {
   printf("Hello, World!\n");
   return 0;
}
"#;

pub fn create_project<T, N>(project_dir: T, name: N) -> Result<(), Report>
where
    PathBuf: From<T>,
    N: ToString,
{
    fn create<A>(path: &Path, addition: A) -> io::Result<()>
    where
        A: AsRef<Path>,
    {
        let mut path = path.to_owned();
        path.push(addition);
        fs::create_dir(path)?;
        Ok(())
    }

    fn create_main_file<A>(path: &Path, addition: A) -> io::Result<()>
    where
        A: AsRef<Path>,
    {
        let mut path = path.to_owned();
        path.push(addition);
        let mut file = File::create(path)?;
        file.write_all(HELLO_WORLD.as_bytes())?;
        Ok(())
    }

    let mut path = PathBuf::from(project_dir);
    if path.exists() && path.is_dir() {
        path.push(name.to_string());
        create(&path, "")?;
        create_loki_toml(name.to_string(), &path)?;
        create(&path, "src")?;
        create_main_file(&path, "src/main.c")?;
    }
    println!("Creating dirs as: {:?}", path);
    Ok(())
}

pub fn create_loki_toml(name: String, path: &Path) -> Result<(), Report> {
    let mut path = path.to_owned();
    path.push("loki.toml");
    let project = Project::with_name(name);
    let toml_string = toml::to_string(&project)?;
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
