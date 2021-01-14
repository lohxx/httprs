use std::collections::HashMap;
use std::fmt::{Display, Debug};

#[derive(Debug)]
pub struct Response {
    status_code: u8,
    body: Option<String>,
    headers: HashMap<String, String>,
    response_text: String
}


impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}", self.response_text
        )
    }
}


impl Response {
    pub fn parse(response_text: String) -> Self {
        Self {
            body: None,
            status_code: 200,
            headers: HashMap::new(),
            response_text: response_text
        }
    }
}