// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'router.rs'
// Author: jcjuarez
// *******************************************************

use hyper::Body;
use routerify::Router;
use std::convert::Infallible;
use crate::actions::fcreate::fcreate_endpoint;
use crate::actions::dcreate::dcreate_endpoint;
use crate::actions::fcreate::FCREATE_ENDPOINT_ROUTE;
use crate::actions::dcreate::DCREATE_ENDPOINT_ROUTE;

const BASE_RFMP_ENDPOINT_ROUTE: &str = "/rfmp";

pub fn router() -> Router<Body, Infallible> {
    Router::builder()
        .put(BASE_RFMP_ENDPOINT_ROUTE.to_string() + FCREATE_ENDPOINT_ROUTE, fcreate_endpoint)
        .put(BASE_RFMP_ENDPOINT_ROUTE.to_string() + DCREATE_ENDPOINT_ROUTE, dcreate_endpoint)
        .build()
        .unwrap()
}