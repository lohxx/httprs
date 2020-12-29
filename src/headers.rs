use std::fmt::Debug;
use std::fmt::Display;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Headers {
    headers: HashMap<String, String>
}

impl Display for Headers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let mut headers_as_string = String::from("");

        for (k, v) in self.headers.iter() {
            headers_as_string = format!("{}{}: {}\r\n", headers_as_string, k, v);
        }

        f.write_str(headers_as_string.as_str())
    }
}

impl Headers {
    pub fn new(items: Vec<(&str, &str)>) -> Self {
       let mut headers = Self {
           headers: HashMap::new()
       };

       for (key, value) in items {
           headers.headers.insert(key.to_string(), value.to_string());
       }

       headers
    }

    pub fn insert(&mut self, value: (&str, &str)) {
        let (k, v) = value;
        self.headers.insert(k.to_string(), v.to_string());
    }
}