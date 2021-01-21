use std::collections::HashMap;
use std::fmt::{Display, Debug};

use super::headers::Header;

#[derive(Debug)]
pub struct Response<'a> {
    pub status_code: String,
    pub phrase: Option<String>,
    pub cookies: Option<String>,

    body: Option<String>,
    headers: Option<Vec<Header<'a>>>,
    response_text: String
}

impl Display for Response<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}", self
        )
    }
}

impl Response<'_> {
    pub fn parse(response_text: String) -> Self {
        let mut headers = vec![];
        let mut body: Option<&str> = None;
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

        body = response_lines.next();

        dbg!(headers);

        Self {
            body: Some(response_text.clone()),
            status_code: status_line[1].to_string(),
            headers: None,
            phrase: None,
            cookies: None,
            response_text: response_text
        }
    }
}
