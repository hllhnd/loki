mod task;

use std::env::current_dir;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

use color_eyre::Result;
use futures::future::join_all;
use task::compile::compile_source_to_object;
use task::link::link_objects_to_executable;
use task::object::resolve_object;
use tokio::fs::create_dir_all;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let target_dir = current_dir()?.join("target");
    create_dir_all(&target_dir).await?;

    let objects = join_all(
        current_dir()?
            .join("src")
            .read_dir()?
            .filter_map(Result::ok)
            .map(|de| de.path())
            .filter(|pb| pb.is_file())
            .filter(|pb| pb.extension().is_some_and(|ex| ex == "c"))
            .map(|source| async {
                let object = resolve_object(&source, &target_dir).await?;

                assert_eq!(compile_source_to_object(source, &object).await?.code().unwrap(), 0);

                Ok::<PathBuf, Error>(object)
            }),
    )
    .await;

    link_objects_to_executable(
        objects
            .iter()
            .map(|o| o.as_ref().cloned().unwrap())
            .collect::<Vec<_>>()
            .as_slice(),
        PathBuf::from("target/executable"),
    )
    .await?;

    Ok(())
}
