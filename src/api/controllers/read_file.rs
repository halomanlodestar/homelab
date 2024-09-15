use std::path::PathBuf;

use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::files::read_file;

#[derive(Deserialize)]
pub struct Request {
    path: Option<String>,
    start: Option<usize>,
    end: Option<usize>,
}

pub async fn read_file_controller(
    request: Query<Request>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut header = HeaderMap::new();

    let file = match read_file(PathBuf::from(
        String::from("src/root_files_folder/") + &request.path.clone().unwrap_or(String::from("/")),
    )) {
        Ok(x) => x,
        Err(err) => return Err((StatusCode::NOT_FOUND, String::from(err))),
    };

    let max_len = file.len();

    let start = request.start.unwrap_or(0);
    let end = request.end.unwrap_or(max_len);

    let file_slice = &file[start..end];

    let status = if start == 0 && end == max_len {
        StatusCode::OK
    } else {
        StatusCode::PARTIAL_CONTENT
    };

    header.insert(
        "Content-Range",
        format!("bytes {}-{}/{}", start, end, max_len)
            .parse()
            .unwrap(),
    );
    // header.insert("Transfer-Encoding", "".parse().unwrap());
    header.insert("Content-Type", "video/mp4".parse().unwrap());
    // header.insert("Content-Length", format!("{}",file_slice.len()).parse().unwrap());

    return Ok((status, header, file_slice.to_owned()));
}
