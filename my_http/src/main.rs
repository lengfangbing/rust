use std::convert::Infallible;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use http::{Request as Req, Response as Res};
use futures::future::{Future};
mod router;

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

// a service
async fn service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // println!("{:?}", req);
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("get '/'");
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    }

    Ok(response)
}

#[tokio::main]
async fn start_server() {
    // bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // create service
    let make_svc = make_service_fn(|_conn| async {
        // service_fn convert our function into a Service
        Ok::<_, Infallible>(service_fn(service))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn start_http_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn main() {
    router::router::test();
    start_server();
}
