// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'status.rs'
// Author: jcjuarez
// *******************************************************

pub type StatusCode = u16;

pub struct Status;

impl Status {
    // Determines if an operation failed.
    pub fn failed(status_code: StatusCode) -> bool {
        return status_code != Status::SUCCESS;
    }

    // Operation succeeded.
    pub const SUCCESS: StatusCode = 0;

    // Invalid HTTP method provided for an API endpoint.
    pub const INVALID_HTTP_METHOD: StatusCode = 9001;

    // Byte stream conversion failed.
    pub const BYTE_STREAM_CONVERSION_FAILED: StatusCode = 9002;

    // String conversion failed.
    pub const STRING_CONVERSION_FAILED: StatusCode = 9003;

    // Deserialization proces failed.
    pub const DESERIALIZATION_FAILED: StatusCode = 9004;

    // Invalid or missing expected fields in request.
    pub const INVALID_OR_MISSING_EXPECTED_FIELDS: StatusCode = 9005;

    // File already exists in the server filesystem.
    pub const FILE_ALREADY_EXISTS: StatusCode = 9006;

    // Failed to create the specified file in the server filesystem.
    pub const FILE_CREATION_FAILED: StatusCode = 9007;

    // Directory already exists in the server filesystem.
    pub const DIRECTORY_ALREADY_EXISTS: StatusCode = 9008;

    // Failed to create the specified directory in the server filesystem.
    pub const DIRECTORY_CREATION_FAILED: StatusCode = 9009;

    // HTTP protocol error.
    pub const HTTP_ERROR: StatusCode = 9010;
}