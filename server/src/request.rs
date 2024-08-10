// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'request.rs'
// Author: jcjuarez
// *******************************************************

use serde_json::Value;
use crate::status::Status;
use crate::status::StatusCode;
use hyper::{body, Body, Request};

pub async fn deserialize_request_body(request: Request<Body>) -> Result<Value, StatusCode> {
    let request_body_bytes = match body::to_bytes(request.into_body()).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err(Status::BYTE_STREAM_CONVERSION_FAILED)
        }
    };

    let request_body = match String::from_utf8(request_body_bytes.to_vec()) {
        Ok(body) => body,
        Err(_) => {
            return Err(Status::STRING_CONVERSION_FAILED)
        }
    };

    match serde_json::from_str(&request_body) {
        Ok(json) => return Ok(json),
        Err(_) => {
            return Err(Status::DESERIALIZATION_FAILED)
        }
    };
}