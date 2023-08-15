use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;
use tokio::io::Error;
use xxhash_rust::xxh3::xxh3_64;

/// Map a source file to its object file using a hashing algorithm
pub async fn resolve_object(source: PathBuf, object_directory: PathBuf) -> Result<PathBuf, Error> {
    let file_contents = {
        let mut buf = Vec::new();
        let file = File::open(source).await?;
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buf).await?;
        buf
    };

    let hash = xxh3_64(&file_contents);
    let object = object_directory.join(format!("{:x}.o", hash));

    Ok(object)
}
