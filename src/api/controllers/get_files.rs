use crate::files::{get_files, FileData, FileType, IsValid};
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize)]
pub struct Request {
    path: Option<String>,
    file_type: Option<String>,
    search: Option<String>,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    files: Vec<FileData>,
}

#[derive(Serialize)]
pub struct ApiError {
    message: String,
    status: u16,
}

pub async fn get_files_controller(
    request: Query<Request>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let path = request.path.clone().unwrap_or(String::from("/"));
    let final_path = String::from("src/root_files_folder/") + &path;

    let mut list: Vec<FileData> = match get_files(Path::new(&final_path)) {
        Ok(val) => val,
        Err(_) => {
            return Err((
                StatusCode::NOT_FOUND,
                String::from("No such directory found"),
            ));
        }
    };

    match &request.file_type {
        Some(file_type) => {
            if !FileType::is_valid(&file_type) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    String::from(format!(
                        "{} is invalid argument. Either use Dir or File",
                        file_type
                    )),
                ));
            }

            list = list
                .into_iter()
                .filter(|file| *file_type == file.file_type.to_string())
                .collect();
        }
        None => {}
    }

    match &request.search {
        Some(name) => {
            list = list
                .into_iter()
                .filter(|file| file.name.contains(name))
                .collect();
        }
        None => {}
    }

    return Ok(Json(SuccessResponse { files: list }));
}
