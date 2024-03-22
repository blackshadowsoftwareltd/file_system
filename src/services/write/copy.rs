use anyhow::Result;
use std::path::PathBuf;

pub async fn copy_file(from: PathBuf, to: PathBuf) -> Result<()> {
    let to = to.join(from.file_name().unwrap());
    tokio::fs::copy(from, to).await?;
    Ok(())
}
