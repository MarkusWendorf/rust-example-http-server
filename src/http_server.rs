use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread, vec,
};

use crate::*;

pub struct HttpServer {
    routes: Vec<RouteHandler>,
}

struct RouteHandler {
    path: String,
    method: String,
    handler: Box<dyn Fn(&Request) -> Response + Send + Sync>,
}

impl HttpServer {
    pub fn new() -> Self {
        HttpServer { routes: vec![] }
    }

    pub fn get(
        &mut self,
        path: &str,
        handler: impl Fn(&Request) -> Response + Send + Sync + 'static,
    ) {
        self.routes.push(RouteHandler {
            path: path.to_owned(),
            method: "GET".to_owned(),
            handler: Box::new(handler),
        });
    }

    pub fn post(
        &mut self,
        path: &str,
        handler: impl Fn(&Request) -> Response + Send + Sync + 'static,
    ) {
        self.routes.push(RouteHandler {
            path: path.to_owned(),
            method: "POST".to_owned(),
            handler: Box::new(handler),
        });
    }

    pub fn serve(self) {
        let listener = TcpListener::bind("127.0.0.1:3000").expect("could not bind to port 3000");
        let arc = Arc::new(self);

        for stream in listener.incoming() {
            let server = arc.clone();
            thread::spawn(move || server.handle_request(stream));
        }
    }

    fn handle_request(&self, stream: Result<TcpStream, std::io::Error>) {
        match stream {
            Ok(mut stream) => {
                let request = parse_request_header(&stream);

                for route in self.routes.iter() {
                    if request.uri == route.path && request.method == route.method {
                        let response = (*route.handler)(&request);
                        send_response(&mut stream, response).expect("error sending response");
                        return;
                    }
                }

                stream
                    .write("HTTP/1.1 404 Not Found\n\n".as_bytes())
                    .expect("error sending 404");
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}

fn parse_request_header(stream: &TcpStream) -> Request {
    let mut request = Request::new();
    let mut reader = BufReader::new(stream);

    for line in reader.by_ref().lines().enumerate() {
        match line {
            (0, Ok(line)) => {
                parse_request_line(&mut request, &line);
            }
            (_, Ok(line)) => {
                if let Some((header, value)) = line.split_once(":") {
                    let header = header.trim().to_owned();
                    let value = value.trim().to_owned();
                    request.headers.insert(header, value);
                }

                // line == "" means the reader encountered \n\n
                // the request header ends here
                if line == "" {
                    break;
                }
            }
            (_, Err(err)) => {
                println!("err {:?}", err)
            }
        }
    }

    let content_length = match request.headers.get("Content-Length") {
        Some(length) => length.parse::<usize>().unwrap_or_default(),
        None => 0,
    };

    let mut buffer = vec![0u8; content_length];
    match reader.read_exact(&mut buffer) {
        Ok(_) => request.body = Some(buffer),
        Err(_) => {
            // todo err handling
        }
    }

    return request;
}

fn parse_request_line(request: &mut Request, line: &String) {
    for part in line.split_whitespace().enumerate() {
        match part {
            (0, method) => request.method = method.to_owned(),
            (1, request_uri) => request.uri = request_uri.to_owned(),
            (2, http_version) => request.http_version = http_version.to_owned(),
            (_, _) => {}
        }
    }
}

fn send_response(stream: &mut TcpStream, response: Response) -> Result<usize, std::io::Error> {
    let mut writer = BufWriter::new(stream);

    let headers = response
        .headers
        .iter()
        .fold(String::new(), |acc, (header, value)| {
            acc + header + ":" + value + "\n"
        });

    let status_line_and_headers = format!(
        "{} {} {}\n{}\n",
        response.http_version, response.status, response.status_text, headers
    );

    writer.write(status_line_and_headers.as_bytes())?;

    if let Some(body) = response.body {
        writer.write(&body)
    } else {
        Ok(0)
    }
}
