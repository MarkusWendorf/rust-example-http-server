mod http_server;
mod request;
mod response;

use http_server::HttpServer;
use request::Request;
use response::Response;

fn main() {
    let mut server = HttpServer::new();

    server.post("/", |req| {
        let mut response = Response::new();
        response
            .headers
            .insert(String::from("x-my-header"), String::from("nope"));

        response.body = req.body.clone();

        response
    });

    server.serve();
}
