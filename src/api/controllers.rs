use std::path::Path;
use axum::{extract::Query, Json};
use serde::Deserialize;
use crate::files::{get_files, FileData};

#[derive(Deserialize)]
pub struct FilePath {
  path: String
}

pub async fn get_files_from(file_path: Query<FilePath>) -> Json<Vec<FileData>> {

  let path = &file_path.path;

  let final_path = String::from("src/root_files_folder/") + path;

  let list: Vec<FileData> = get_files(Path::new(&final_path));

  return Json(list);
}