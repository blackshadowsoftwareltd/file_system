use std::path::PathBuf;

use super::read::read::read;

pub async fn run() {
    let path = PathBuf::from("src");
    match read(path).await {
        Ok(e) => {
            for x in e {
                println!("{:?}", x.entry);
                println!("{:?}", x.children)
            }
        }
        Err(e) => println!("error: {:?}", e),
    }

    // match watcher(path).await {
    //     Ok(_) => println!("watcher started successfully!"),
    //     Err(e) => println!("error: {:?}", e),
    // }
}
