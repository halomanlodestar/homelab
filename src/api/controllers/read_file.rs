use std::path::PathBuf;

use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::files::read_file;

#[derive(Deserialize)]
pub struct Request {
    path: Option<String>,
}

pub async fn read_file_controller(request: Query<Request>) -> impl IntoResponse {
    // println!("{}", request.path.clone().unwrap());
    return read_file(PathBuf::from(
        String::from("src/root_files_folder/") + &request.path.clone().unwrap_or(String::from("/")),
    ));
}
