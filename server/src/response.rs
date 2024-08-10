// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'response.rs'
// Author: jcjuarez
// *******************************************************

use crate::status;
use serde_json::json;
use hyper::{Body, Response, StatusCode};

const MESSAGE_KEY: &str = "message";
const STATUS_CODE_KEY: &str = "statusCode";

pub fn create_response(message: &str, status_code: status::StatusCode) -> Response<Body> {
    let response = json!({
        MESSAGE_KEY: message,
        STATUS_CODE_KEY: status_code
    });

    let server_http_status_code = if status::Status::failed(status_code) {
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::OK
    };
    
    Response::builder()
        .status(server_http_status_code)
        .header("Content-Type", "application/json")
        .body(Body::from(response.to_string()))
        .unwrap()
}