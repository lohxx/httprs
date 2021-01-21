use std::fmt::Debug;

use regex::Regex;


#[derive(Debug)]
pub struct URL<'u> {
    pub path: &'u str,
    pub port: String,
    pub scheme: &'u str,
    pub hostname: &'u str
}

impl <'u>URL<'u> {
    pub fn parse(uri: &'u str) -> Self {
        let (host, port, scheme) = URL::scheme_port(&uri);

        let (mut hostname_without_path, path) = match host.find('/') {
            Some(byte_index) => (&host[..byte_index], &host[byte_index..]),
            None => (host, "/")
        };

        hostname_without_path = match hostname_without_path.find(port.as_str()) {
            Some(index) => &hostname_without_path[..index-1],
            None => hostname_without_path
        };

        Self {
            path,
            port: port,
            scheme: scheme,
            hostname: hostname_without_path,
        }
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    fn get_port_number(hostname: &str) -> String {
        let port = Regex::new(r"(?P<port>:\d+)")
            .unwrap()
            .captures(hostname);

        match port {
            Some(p) => p["port"][1..].to_string(),
            None => String::from("")
        }

    }

    fn scheme_port(hostname: &str) -> (&str, String, &str) {
        let port = URL::get_port_number(hostname);

        let (uri, default_port, scheme) = match &hostname[..5] {
            "http:" => (&hostname[7..], "80", "http"),
            "https" => (&hostname[8..], "443", "https"),
            _ => (hostname, "80", "http")
        };

        if port.len() >= 1 {
            return (uri, String::from(port), scheme)
        }

        (uri, String::from(default_port), scheme)
    }
}
