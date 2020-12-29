use std::fmt::Debug;
use std::fmt::Display;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Headers<'a> {
    headers: HashMap<&'a str, &'a str>
}

impl Display for Headers<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let mut headers_as_string = String::from("");

        for (k, v) in self.headers.iter() {
            headers_as_string = format!("{}{}: {}\r\n", headers_as_string, k, v);
        }

        f.write_str(headers_as_string.as_str())
    }
}

impl <'a>Headers<'a> {
    /// Adiciona multiplos headers ao mapeamento.
    pub fn new(items: Vec<(&'a str, &'a str)>) -> Self {
       let mut headers = Self {
           headers: HashMap::new()
       };

       for (key, value) in items {
           headers.headers.insert(key, value);
       }

       headers
    }

    /// Adiciona um novo header
    pub fn insert(&mut self, value: (&'a str, &'a str)) {
        self.headers.insert(value.0, value.1);
    }
}