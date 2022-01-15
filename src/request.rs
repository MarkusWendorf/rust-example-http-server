use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: "method".to_owned(),
            uri: "/".to_owned(),
            http_version: "HTTP/1.1".to_owned(),
            headers: HashMap::new(),
            body: None,
        }
    }
}
