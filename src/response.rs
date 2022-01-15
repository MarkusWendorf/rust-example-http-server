use std::collections::HashMap;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: 400,
            status_text: "Bad Request".to_owned(),
            headers: HashMap::new(),
            http_version: "HTTP/1.1".to_owned(),
            body: None,
        }
    }
}
