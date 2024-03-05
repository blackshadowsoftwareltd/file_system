use walkdir::DirEntry;

#[derive(Debug)]
pub struct Content {
    pub entry: Option<DirEntry>,
    pub children: Option<Vec<DirEntry>>,
}
