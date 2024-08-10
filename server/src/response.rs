// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'response.rs'
// Author: jcjuarez
// *******************************************************

use serde_json::json;
use std::convert::Infallible;
use crate::status::{self, Status};
use hyper::{Body, Request, Response, StatusCode};

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

pub async fn not_found_endpoint(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = json!({
        MESSAGE_KEY: "Endpoint not found.",
        STATUS_CODE_KEY: Status::HTTP_ERROR
    });

    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "application/json")
        .body(Body::from(response.to_string()))
        .unwrap())
}

pub async fn probe_endpoint(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = json!({
        MESSAGE_KEY: "FireFile RFMP server is running.",
        STATUS_CODE_KEY: Status::SUCCESS
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(response.to_string()))
        .unwrap())
}