use std::collections::HashMap;
use std::fmt::{Display, Debug};

#[derive(Debug)]
pub struct Response<'_> {
    status_code: &'_ str,
    body: Option<String>,
    //headers: HashMap<String, String>,
    response_text: String
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "{}", self
        )
    }
}

fn extract_headers(txt: &String) -> Vec<&str> {
    let mut response_lines = txt.lines();
    
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

    status_line
}


impl Response {
    pub fn parse(response_text: String) -> Self { 
        let status_line = extract_headers(&response_text);

        Self {
            body: Some(response_text),
            status_code: status_line[1],
            //headers: HashMap::new(),
            response_text: response_text
        }
    }
}
