use anyhow::Result;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::models::content::Content;

pub async fn read(path: PathBuf) -> Result<Vec<Content>> {
    let mut contents = vec![];
    // let dirs = nested(path)?;

    // for x in dirs {
    //     contents.push(manege(x)?)
    // }
    Ok(contents)
}

// fn manege(e: DirEntry) -> Result<Content> {
//     // println!("{:?}", e);
//     let children = nested(e.path().to_path_buf())?;
//     let content = Content {
//         entry: Some(e.clone()),
//         children: Some(children),
//     };
//     Ok(content)
// }

// fn nested(content: Content) -> Result<Vec<Content>> {
//     let path = content.entry.clone().unwrap().path().to_path_buf();
//     let dirs = WalkDir::new(path)
//         .max_depth(1)
//         .into_iter()
//         .filter_map(|e| e.ok())
//         .filter(|e| {
//             path.clone() != e.path().to_path_buf()
//                 && (e.file_type().is_file() || e.file_type().is_dir())
//         })
//         .collect::<Vec<DirEntry>>();
//     // let mut contents = vec![];
//     // for x in &dirs {
//     //     contents.push(manege(x.clone())?)
//     // }
//     Ok(Content {
//         entry: Some(content.entry.unwrap()),
//         children: Some(dirs),
//     })
// }
