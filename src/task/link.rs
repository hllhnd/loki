use std::path::PathBuf;
use std::process::ExitStatus;

use tokio::io::Error;
use tokio::process::Command;

/// Use Clang to link objects to an executable
pub async fn link_objects_to_executable(objects: &[PathBuf], executable: PathBuf) -> Result<ExitStatus, Error> {
    let mut linker_command = Command::new("clang");
    linker_command.arg("-o");
    linker_command.arg(executable);
    linker_command.args(objects);

    linker_command.spawn()?.wait().await
}
