// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'dcreate.rs'
// Author: jcjuarez
// *******************************************************

use serde_json::Value;
use crate::status::Status;
use std::convert::Infallible;
use std::fs::{self, create_dir_all};
use hyper::{Body, Request, Response};
use crate::response::create_response;
use crate::request::deserialize_request_body;

const DCREATE_FILE_PATH_KEY: &str = "directoryPath";
pub const DCREATE_ENDPOINT_ROUTE: &str = "/dcreate";

pub async fn dcreate_endpoint(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Get the request body.
    let request_body = match deserialize_request_body(request).await {
        Ok(body) => body,
        Err(status) => return Ok(create_response("Failed to deserialize the request body.", status))
    };

    // Check if the object has the expected field.
    let directory_path = match request_body.get(DCREATE_FILE_PATH_KEY) {
        Some(Value::String(directory)) => directory.clone(),
        _ => {
            return Ok(create_response("Invalid or missing expected fields.", Status::INVALID_OR_MISSING_EXPECTED_FIELDS))
        }
    };

    // Check if the directory already exists in the filesystem.
    if fs::metadata(&directory_path).is_ok() {
        return Ok(create_response("Directory already exists.", Status::DIRECTORY_ALREADY_EXISTS))
    }

    // If it does not exist, create it.
    let _ = match create_dir_all(directory_path) {
        Ok(_) => {},
        Err(_) => {
            return Ok(create_response("Directory creation failed.", Status::DIURECTORY_CREATION_FAILED))
        }
    };

    Ok(create_response("Directory has been created.", Status::SUCCESS))
}