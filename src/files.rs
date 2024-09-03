use std::{fs::{self}, os::windows::fs::MetadataExt, path::{Path, PathBuf}};
use std::{fmt::Display, io::Error};


#[derive(Debug)]
#[derive(serde::Serialize)]
enum FileType {
    File = 1,
    Dir = 0
}

#[derive(serde::Serialize)]
pub struct FileData {
    name: String,
    path: PathBuf,
    size: u64,
    file_type: FileType,
    extension: String
}

impl Display for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ name:{:?}, path:{:?}, size:{}, file_type:{:?}. extension:{:?} }}", self.name, self.path, self.size, self.file_type, self.extension)
    }
}

pub fn get_files(path: &Path) -> Result<Vec<FileData>, Error> {

    let dirs = match fs::read_dir(path) {
        Ok(x) => x,
        Err(err) => { return Err(err); },
    };

    let list = Vec::from_iter(dirs.into_iter().map(|file| {

        let file = file.unwrap();
        let metadata = file.metadata().unwrap();

        let name  = String::from(file.file_name().to_str().unwrap());
        let path = file.path();
        let size = metadata.file_size();
        let file_type;
        
        if metadata.file_type().is_file() {
            file_type = FileType::File;
        } else {
            file_type = FileType::Dir;
        };

        
        let extension =
            if let Some(x) = Path::new(path.as_path()).extension() { String::from(x.to_str().unwrap()) }
            else {
                match file_type {
                    FileType::File => String::from("None"),
                    FileType::Dir => String::from("Dir"),
                }
            };
        
        return FileData { name, path, size, file_type, extension };

    }));

    return Ok(list);
}
