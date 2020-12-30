use std::fmt::Debug;


#[derive(Debug)]
pub struct URL<'u> {
    pub path: &'u str,
    pub port: &'u str,
    pub hostname: &'u str
}

impl <'u>URL<'u> {
    pub fn parse(uri: &'u str) -> Self {
        let (host, port) = URL::scheme_port(&uri);

        let (hostname_without_path, path) = match host.find('/') {
            Some(byte_index) => (&host[..byte_index], &host[byte_index..]),
            None => (host, "/")
        };

        Self {
            port,
            path,
            hostname: hostname_without_path,
        }
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.hostname, self.port)
    }

    fn scheme_port(hostname: &str) -> (&str, &str) {
        match &hostname[..5] {
            "http:" => (&hostname[7..], "80"),
            "https" => (&hostname[8..], "443"),
            _ => (&hostname, "80")
        }
    }
}
