use std::collections::HashMap;
use std::fmt::{Display, Debug};

use super::headers::Header;

use chrono::prelude::*;


#[derive(Debug)]
pub struct Response {
    pub phrase: String,
    pub status_code: u16,
    pub http_version: String,
    pub body: Option<String>,
    pub cookies: Option<Cookies>,
    pub headers: HashMap<String, String>,
}

// Encontrar o cookie pelo nome e acessar seus atributos, como valor, data de expiração, dominio e httpOnly

#[derive(Debug)]
pub struct Cookie {
    name: String,
    value: String,
    expires: usize, // Date????
    path: String,
    domain: String,
    http_only: bool,
    same_site: String
}

#[derive(Debug)]
pub struct Cookies {
    cookie: Vec<Cookie>
}


impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Response {
    pub fn parse(mut response_text: String) -> Self {
        let mut body = None;
        let mut headers = HashMap::new();
        let mut concat_body = String::from("");
        let mut response_lines = response_text.lines();

    
        // A primeira linha de uma resposta é sempre a de status.
        let mut status_line = vec![];
        
        match &response_lines.next() {
            Some(data) => {
                let mut split = data.split_ascii_whitespace();
                status_line.push(split.next().unwrap());
                status_line.push(split.next().unwrap());
                status_line.push(split.next().unwrap());

                dbg!(data);
            },
            _ => ()
        };

        while let Some(header_line) = response_lines.next() {
            if header_line.len() == 0 {
                // começa o body, testar tbm com chunked body
                while let Some(content) = response_lines.next() {
                    concat_body += content;
                };
                break;
            }

            if header_line.contains("Set-Cookie") {
                // separar os cookies aqui
                let cookie_attributes: Vec<&str> = header_line.split(";").collect();
                let cookie_name_value: Vec<&str> = cookie_attributes[0].split("=").collect();

                for (pos, attr) in cookie_attributes.iter().enumerate() {
                    dbg!(pos, attr);
                }

                // cookies.push(
                //     Cookie {
                //         name: cookie_name_value[0].to_string(),
                //         value: cookie_name_value[1].to_string(),
                //         path: host
                //     });
            }

            let values: Vec<&str> = header_line.split(":").collect();

            headers.insert(values[0].to_string(), values[1].to_string());
        };

        let mut version = status_line[0].split("/");

        // descarta o nome do protocolo.
        version.next();

        let version = version.next().unwrap();
        let status_code = status_line[1].to_string();

        if concat_body.len() > 0 {
            body = Some(concat_body);
        }

        dbg!(&status_line);

        Self {
            body: body,
            cookies: None,
            headers: headers,
            http_version: version.to_string(),
            phrase: status_line[2].to_string(),
            status_code: status_code.parse::<u16>().unwrap(),
        }
    }
}