use crate::files::read_file;
use axum::{extract::Query, http::HeaderMap, response::IntoResponse};
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
    request_headers: HeaderMap,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut header = HeaderMap::new();

    const BASE_SIZE: usize = 1_000;

    let start_range: usize = match request_headers.get("Range") {
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

    let file = match read_file(path).await {
        Ok(x) => x,
        Err(err) => return Err((StatusCode::NOT_FOUND, err)),
    };

    let end_range = usize::max(start_range + BASE_SIZE, file.len());

    let partial_file = file.slice(start_range..end_range);

    header.insert("Content-Type", "video/mp4".parse().unwrap());
    header.insert("Accept-Ranges", "bytes".parse().unwrap());
    header.insert(
        "Content-Length",
        format!("{}", partial_file.len()).parse().unwrap(),
    );
    header.insert(
        "Content-Range",
        format!(
            "bytes {}-{}/{}",
            start_range,
            start_range + BASE_SIZE,
            file.len()
        )
        .parse()
        .unwrap(),
    );

    if start_range == file.len() {
        return Ok((StatusCode::OK, header, partial_file));
    }

    return Ok((StatusCode::PARTIAL_CONTENT, header, partial_file));
}
