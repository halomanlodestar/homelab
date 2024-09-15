use std::path::PathBuf;

use axum::{body::Body, extract::Query, http::HeaderMap, response::IntoResponse};
use serde::Deserialize;

use crate::files::read_file;

#[derive(Deserialize)]
pub struct Request {
    path: Option<String>,
    // start: Option<usize>,
    // end: Option<usize>,
}

pub async fn read_file_controller(
    request: Query<Request>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut header = HeaderMap::new();

    let file = match read_file(PathBuf::from(
        String::from("src/root_files_folder/") + &request.path.clone().unwrap_or(String::from("/")),
    ))
    .await
    {
        Ok(x) => x,
        Err(_) => return Err(""),
    };

    // println!("{:?}", f);

    header.insert("Content-Type", "video/mp4".parse().unwrap());

    return Ok((header, Body::from_stream(file)));
}
