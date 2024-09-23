use crate::disk::files::{get_file_metadata, read_file_range};
use axum::{body::Body, extract::Query, http::HeaderMap, response::IntoResponse};
use hyper::StatusCode;
use serde::Deserialize;
use std::{os::windows::fs::MetadataExt, path::PathBuf};

#[derive(Deserialize)]
pub struct Request {
    path: Option<String>,
}

pub async fn read_file_controller(
    request: Query<Request>,
    request_headers: HeaderMap,
) -> impl IntoResponse {
    let mut header = HeaderMap::new();

    const BASE_SIZE: u64 = 128_000;

    let start_range: u64 = match request_headers.get("Range") {
        Some(val) => {
            let st = String::from(val.to_str().unwrap()).split_off(6);
            let vals = st.split_once("-").unwrap();
            vals.0.parse().unwrap()
        }
        None => 0,
    };

    let path = PathBuf::from(
        String::from("src/root_files_folder/") + &request.path.clone().unwrap_or(String::from("/")),
    );

    let metadata = match get_file_metadata(path.clone()).await {
        Ok(x) => {
            x
        },
        Err(err) => return Err((StatusCode::NOT_FOUND, err)),
    };

    let max_size = metadata.file_size();

    let end_range = u64::min(start_range + BASE_SIZE, max_size);

    let file = match read_file_range(path.clone(), start_range, end_range).await {
        Ok(x) => {
            x
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    };

    header.insert("Content-Type", "video/x-matroska".parse().unwrap());
    header.insert("Accept-Ranges", "bytes".parse().unwrap());
    header.insert(
        "Content-Range",
        format!("bytes {}-{}/{}", start_range, end_range, max_size)
            .parse()
            .unwrap(),
    );
    header.insert("Connection", "keep-alive".parse().unwrap());
    header.insert("Keep-Alive", "timeout=5, max=100".parse().unwrap());
    
    let stream = Body::from_stream(file);

    return Ok((StatusCode::PARTIAL_CONTENT, header, stream));
}
