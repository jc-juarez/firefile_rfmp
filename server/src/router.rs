// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'router.rs'
// Author: jcjuarez
// *******************************************************

use hyper::Body;
use routerify::Router;
use std::convert::Infallible;
use crate::response::probe_endpoint;
use crate::response::not_found_endpoint;
use crate::actions::fcreate::fcreate_endpoint;
use crate::actions::dcreate::dcreate_endpoint;
use crate::actions::fcreate::FCREATE_ENDPOINT_ROUTE;
use crate::actions::dcreate::DCREATE_ENDPOINT_ROUTE;

const BASE_RFMP_ENDPOINT_ROUTE: &str = "/rfmp";

pub fn router() -> Router<Body, Infallible> {
    Router::builder()
        .get(BASE_RFMP_ENDPOINT_ROUTE, probe_endpoint)
        .put(BASE_RFMP_ENDPOINT_ROUTE.to_string() + FCREATE_ENDPOINT_ROUTE, fcreate_endpoint)
        .put(BASE_RFMP_ENDPOINT_ROUTE.to_string() + DCREATE_ENDPOINT_ROUTE, dcreate_endpoint)
        .any(not_found_endpoint)
        .build()
        .unwrap()
}