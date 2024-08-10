// *******************************************************
// FireFile - RFMP (Remote Filesystem Management Protocol)
// Server
// 'main.rs'
// Author: jcjuarez
// *******************************************************

pub mod router;
pub mod status;
pub mod actions;
pub mod request;
pub mod response;

use std::env;
use hyper::Server;
use std::net::SocketAddr;
use crate::router::router;
use routerify::RouterService;

const DEFAULT_RFMP_SERVER_PORT: &str = "9090";

#[tokio::main]
async fn main() {
    // Set the specified port provided as parameter, if any.
    let port: u16 = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_RFMP_SERVER_PORT.to_string())
        .parse()
        .expect("Invalid port provided for the FireFile RFMP server");

    // Bind socket to any network interface the server
    // is reachable at, as per the established behavior of RFMP.
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let router = router();
    let service = RouterService::new(router).unwrap();
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

