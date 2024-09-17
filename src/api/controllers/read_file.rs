use crate::files::read_file;
use axum::{body::Body, extract::Query, http::HeaderMap, response::IntoResponse};
use hyper::StatusCode;
use serde::Deserialize;
use std::path::PathBuf;

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

    let path = PathBuf::from(
        String::from("src/root_files_folder/") + &request.path.clone().unwrap_or(String::from("/")),
    );

    let file = match read_file(path).await {
        Ok(x) => x,
        Err(err) => return Err((StatusCode::NOT_FOUND, err)),
    };

    header.insert("Content-Type", "video/mp4".parse().unwrap());

    return Ok((StatusCode::OK, header, Body::from_stream(file)));
}
