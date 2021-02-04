use std::collections::HashMap;
use std::fmt::{Display, Debug};

use super::headers::Header;

#[derive(Debug)]
pub struct Response {
    pub phrase: String,
    pub status_code: u8,
    pub http_version: String,
    pub body: Option<String>,
    pub cookies: Option<String>,
    pub headers: HashMap<String, String>,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Response {
    pub fn parse(mut response_text: String) -> Self {
        let mut headers = HashMap::new();
        let mut response_lines = response_text.lines();
    
        // A primeira linha de uma resposta é sempre a de status.
        let mut status_line = vec![];
        
        match &response_lines.next() {
            Some(data) => {
                let mut split = data.split_ascii_whitespace();
                status_line.push(split.next().unwrap());
                status_line.push(split.next().unwrap());
                status_line.push(split.next().unwrap());
            },
            _ => ()
        };
        
        while let Some(header_line) = response_lines.next() {
            if header_line.len() == 0 {
                // começa o body, testar tbm com chunked body
                break;
            }

            let values: Vec<&str> = header_line.split(":").collect();

            headers.insert(values[0].to_string(), values[1].to_string());
        };

        let body = match response_lines.next() {
            Some(data) => Some(data.to_string()),
            None => None
        };

        let mut version = status_line[0].split("/");

        // descarta o nome do protocolo.
        version.next();

        let version = version.next().unwrap();
        let status_code = status_line[1].to_string();
        
        Self {
            body,
            cookies: None,
            headers: headers,
            http_version: version.to_string(),
            phrase: status_line[2].to_string(),
            status_code: status_code.parse::<u8>().unwrap(),
        }
    }

    fn parse_body(body: String, chunked_body: bool) -> String {
        unimplemented!()
    }
}