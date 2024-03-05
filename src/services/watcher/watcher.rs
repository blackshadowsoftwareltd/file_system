use anyhow::Result;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::path::PathBuf;
pub async fn watcher(path: PathBuf) -> Result<()> {
    let mut watcher = recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    loop {
        watcher.watch(&path, RecursiveMode::Recursive)?;
    }
}
