mod http_server;
mod request;
mod response;

use http_server::HttpServer;
use request::Request;
use response::Response;

fn add(req: &Request) -> Response {
    let mut response = Response::new();

    response.body = req.body.clone();

    response
}

fn main() {
    let mut router = HttpServer::new();

    router.get("/", add);
    router.post("/", |req| {
        let mut response = Response::new();
        response
            .headers
            .insert(String::from("x-my-header"), String::from("nope"));
        response.body = req.body.clone();

        response
    });

    router.serve();
}
