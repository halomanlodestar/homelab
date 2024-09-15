use serde::Serialize;
use std::{fmt::Display, io::Error};
use std::{
    fs::{self},
    os::windows::fs::MetadataExt,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize)]
pub enum FileType {
    File = 1,
    Dir = 0,
}

pub trait IsValid<T> {
    fn is_valid(val: &T) -> bool;
}

impl IsValid<String> for FileType {
    fn is_valid(val: &String) -> bool {
        match val.as_str() {
            "File" => true,
            "Dir" => true,
            _ => false,
        }
    }
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            FileType::File => String::from("File"),
            FileType::Dir => String::from("Dir"),
        }
    }
}

// #[derive(Serialize)]
// pub struct Metadata {
//     pub size: usize,
// }

#[derive(Serialize)]
pub struct FileData {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub file_type: FileType,
    pub extension: String,
}

impl Display for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ name:{:?}, path:{:?}, size:{}, file_type:{:?}. extension:{:?} }}",
            self.name, self.path, self.size, self.file_type, self.extension
        )
    }
}

pub fn get_files(path: &Path) -> Result<Vec<FileData>, Error> {
    let dir_content = match fs::read_dir(path) {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };

    let list = Vec::from_iter(dir_content.into_iter().map(|file| {
        let file = match file {
            Ok(entry) => entry,
            Err(_) => panic!("No such entry found"),
        };
        let metadata = match file.metadata() {
            Ok(x) => x,
            Err(_) => panic!("No Meta data found for specified file"),
        };

        let name = String::from(file.file_name().to_str().unwrap_or("unnamed"));
        let path = file.path();
        let size = metadata.file_size();
        let file_type = match metadata.file_type().is_file() {
            true => FileType::File,
            false => FileType::Dir,
        };

        let extension = if let Some(x) = Path::new(path.as_path()).extension() {
            String::from(x.to_str().unwrap())
        } else {
            match file_type {
                FileType::File => String::from("None"),
                FileType::Dir => String::from("Dir"),
            }
        };

        return FileData {
            name,
            path,
            size,
            file_type,
            extension,
        };
    }));

    return Ok(list);
}
