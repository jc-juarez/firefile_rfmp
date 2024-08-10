// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'fcreate.rs'
// Author: jcjuarez
// *******************************************************

use serde_json::Value;
use std::fs::{self, File};
use crate::status::Status;
use std::convert::Infallible;
use hyper::{Body, Request, Response};
use crate::response::create_response;
use crate::request::deserialize_request_body;

const FCREATE_FILE_PATH_KEY: &str = "filePath";
pub const FCREATE_ENDPOINT_ROUTE: &str = "/fcreate";

pub async fn fcreate_endpoint(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Get the request body.
    let request_body = match deserialize_request_body(request).await {
        Ok(body) => body,
        Err(status) => return Ok(create_response("Failed to deserialize the request body.", status))
    };

    // Check if the object has the expected field.
    let file_path = match request_body.get(FCREATE_FILE_PATH_KEY) {
        Some(Value::String(file)) => file.clone(),
        _ => {
            return Ok(create_response("Invalid or missing expected fields.", Status::INVALID_OR_MISSING_EXPECTED_FIELDS))
        }
    };

    // Check if the file already exists in the filesystem.
    if fs::metadata(&file_path).is_ok() {
        return Ok(create_response("File already exists.", Status::FILE_ALREADY_EXISTS))
    }

    // If it does not exist, create it.
    let _ = match File::create(file_path) {
        Ok(_) => {},
        Err(_) => {
            return Ok(create_response("File creation failed.", Status::FILE_CREATION_FAILED))
        }
    };

    Ok(create_response("File has been created.", Status::SUCCESS))
}