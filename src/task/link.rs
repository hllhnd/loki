use std::path::Path;
use std::process::ExitStatus;

use tokio::io::Error;
use tokio::process::Command;

/// Use Clang to link objects to an executable
pub async fn link_objects_to_executable(
    objects: &[impl AsRef<Path>],
    executable: impl AsRef<Path>,
) -> Result<ExitStatus, Error> {
    let mut linker_command = Command::new("clang");
    linker_command.arg("-o");
    linker_command.arg(executable.as_ref());
    linker_command.args(objects.iter().map(|path| path.as_ref()));

    linker_command.spawn()?.wait().await
}
