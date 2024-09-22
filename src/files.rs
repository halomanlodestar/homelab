use axum::body::Bytes;
use serde::Serialize;
use std::fmt::Display;
use std::io::Error;
use std::os::windows::fs::MetadataExt;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use tokio::fs::File;
use tokio::io::{Result as TokioResult, AsyncReadExt, SeekFrom, AsyncSeekExt};
use futures::stream::{self, Stream};

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

pub fn get_files(path: &Path) -> std::result::Result<Vec<FileData>, Error> {
    let dir_content = match fs::read_dir(path) {
        std::result::Result::Ok(x) => x,
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

pub async fn read_file(path: PathBuf) -> Result<Bytes, String> {
    return match File::open(&path).await {
        tokio::io::Result::Ok(_) => {
            let stream = match fs::read(path) {
                Ok(file) => file,
                Err(_) => todo!(),
            };

            return Ok(Bytes::from(stream));
        }
        Err(_) => Err(String::from("Unable to read the specified file")),
    };
}


pub async fn read_file_range(path: PathBuf, start: u64, end: u64) -> TokioResult<impl Stream<Item = TokioResult<Vec<u8>>>> {

    let mut file = match File::open(path).await {
        tokio::io::Result::Ok(x) => x,
        Err(_) => todo!()
        // Err(_) => return Err(String::from("No such file found")),
    };

    match file.seek(SeekFrom::Start(start)).await {
        Ok(x) => x,
        Err(_) => todo!(),
    };

    let chunk_size = ((end - start) + 1) as usize;

    let file_stream = stream::unfold((file, chunk_size), move |(mut file, chunk_size)| async move {
        let mut buffer = vec![0; chunk_size];

        match file.read(&mut buffer).await {
            tokio::io::Result::Ok(0) => None,
            tokio::io::Result::Ok(n) => {
                Some((Ok(buffer[..n].to_vec()), (file, chunk_size)))
            },
            tokio::io::Result::Err(e) => Some((tokio::io::Result::Err(e), (file, chunk_size))),
        }
    });

    return Ok(file_stream);
}