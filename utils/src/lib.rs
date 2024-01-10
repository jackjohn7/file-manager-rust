use std::{path::{PathBuf, Path}, fs, time::SystemTime};

pub struct Metadata {
    pub path: PathBuf,
    pub name_str: String,
    pub created_date: SystemTime,
    pub is_symlink: bool
}

pub enum FolderItem {
    File(Metadata),
    Directory(Metadata)
}

/// lists files in the given directory
pub fn files_in_dir(path: &Path) -> Vec<FolderItem> {
    fs::read_dir(path)
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<fs::DirEntry>>()
        .iter()
        .map(|entry| {
            let meta = entry.metadata().unwrap();
            match meta.file_type().is_dir() {
                true => FolderItem::Directory(Metadata {
                    path: entry.path(),
                    name_str: entry.file_name().to_str().unwrap().to_owned(),
                    created_date: meta.created().unwrap(),
                    is_symlink: meta.is_symlink(),
                }),
                false => FolderItem::File(Metadata {
                    path: entry.path(),
                    name_str: entry.file_name().to_str().unwrap().to_owned(),
                    created_date: meta.created().unwrap(),
                    is_symlink: meta.is_symlink(),
                })
            }
        })
        .collect::<Vec<FolderItem>>()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use super::*;

    #[test]
    fn test_files_in_dir() {
        let x = files_in_dir(current_dir().unwrap().as_path());
        for item in x {
            match item {
                FolderItem::Directory(metadata) => {
                    println!("Directory: jname: {}, loc: {:?}", metadata.name_str, metadata.path);
                },
                FolderItem::File(metadata) => {
                    println!("File: name: {}, loc: {:?}", metadata.name_str, metadata.path);
                }
            }
        }
        panic!();
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
