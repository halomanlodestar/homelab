use std::path::PathBuf;

use axum::{extract::Query, http::HeaderMap, response::IntoResponse};
use serde::Deserialize;

use crate::files::read_file;

#[derive(Deserialize)]
pub struct Request {
    path: Option<String>,
    // start: Option<usize>,
    // end: Option<usize>
}

pub async fn read_file_controller(request: Query<Request>) -> impl IntoResponse {
    // println!("{}", request.path.clone().unwrap());

    let mut header = HeaderMap::new();

    header.insert("Content-Type", "video/mp4".parse().unwrap());

    let file = read_file(PathBuf::from(
        String::from("src/root_files_folder/") + &request.path.clone().unwrap_or(String::from("/")),
    ))
    .unwrap();

    return (header, file);
}
