use std::path::Path;
use axum::{extract::Query, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::files::{get_files, FileData};

#[derive(Deserialize)]
pub struct FilePath {
  path: String
}

#[derive(Serialize)]
pub struct Response {
  files: Vec<FileData>
}

pub async fn get_files_from(file_path: Query<FilePath>) -> Result<Json<Response>, StatusCode> {

  let path = &file_path.path;
  let final_path = String::from("src/root_files_folder/") + path;

  let list: Vec<FileData> = match get_files(Path::new(&final_path)) {
    Ok(val) => val,
    Err(_) => {
      return Err(StatusCode::NOT_FOUND);
    },
  };

  return Ok(Json(Response { files: list }));
}