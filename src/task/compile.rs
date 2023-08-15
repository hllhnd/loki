use std::path::PathBuf;
use std::process::ExitStatus;

use tokio::io::Error;
use tokio::process::Command;

/// Invoke a Clang compilation from a source and object path
pub async fn compile_source_to_object(source: PathBuf, object: PathBuf) -> Result<ExitStatus, Error> {
    let mut compiler_command = Command::new("clang");
    compiler_command.arg("-o");
    compiler_command.arg(object);
    compiler_command.arg(source);

    compiler_command.spawn()?.wait().await
}
