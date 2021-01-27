use std::collections::HashMap;
use std::fmt::{Display, Debug};

use super::headers::Header;

#[derive(Debug)]
pub struct Response<'a> {
    pub body: String,
    pub phrase: String,
    pub status_code: String,
    pub cookies: Option<String>,
    pub headers: Vec<Header<'a>>,
    pub http_version: String,
}

impl Display for Response<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Response<'_> {
    pub fn parse(response_text: String) -> Self {
        let mut headers = vec![];
        let mut response_lines = response_text.lines();
    
        // A primeira linha de uma resposta é sempre a de status.
        let mut status_line = vec![];

        match response_lines.next() {
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

            headers.push(Header::from(header_line));
        };

        let body = response_lines.next().unwrap();

        Self {
            cookies: None,
            headers: headers,
            body: body.to_string(),
            phrase: status_line[2].to_string(),
            status_code: status_line[1].to_string(),
            http_version: status_line[0].to_string()
        }
    }

    fn parse_body(body: String, chunked_body: bool) -> String {
        unimplemented!()
    }
}