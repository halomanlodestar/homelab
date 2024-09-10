use std::path::Path;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::files::{get_files, FileData};

#[derive(Deserialize)]
pub struct Request {
  path: String
}


#[derive(Serialize)]
pub struct SuccessResponse {
  files: Vec<FileData>
}

#[derive(Serialize)]
pub struct ApiError {
  message: String,
  status: u16
}

pub async fn get_files_from(file_path: Query<Request>) -> Result<impl IntoResponse, impl IntoResponse> {

  let path = &file_path.path;
  let final_path = String::from("src/root_files_folder/") + path;

  let list: Vec<FileData> = match get_files(Path::new(&final_path)) {
    Ok(val) => val,
    Err(_) => {
      // return Err(ApiError {status: StatusCode::NOT_FOUND.as_u16(), message: String::from("value")});
      // return Err(StatusCode::NOT_FOUND);
      return Err((StatusCode::NOT_FOUND,  String::from("No such directory found")));
    },
  };

  return Ok(Json(SuccessResponse { files: list }));
}