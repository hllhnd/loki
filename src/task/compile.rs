use std::path::Path;
use std::process::ExitStatus;

use tokio::io::Error;
use tokio::process::Command;

/// Invoke a Clang compilation from a source and object path
pub async fn compile_source_to_object(source: impl AsRef<Path>, object: impl AsRef<Path>) -> Result<ExitStatus, Error> {
    let mut compiler_command = Command::new("clang");
    compiler_command.arg("-c");
    compiler_command.arg("-o");
    compiler_command.arg(object.as_ref());
    compiler_command.arg(source.as_ref());

    compiler_command.spawn()?.wait().await
}
