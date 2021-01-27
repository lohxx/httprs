use std::fmt::Debug;
use std::fmt::Display;
use std::convert::{From};
use std::collections::HashMap;


#[derive(Debug)]
pub struct Header<'a> {
    pub name: &'a str,
    pub value: &'a str
}


impl <'a>From<&'a str> for Header<'a> {
    fn from(value: &'a str) -> Self {
        let values: Vec<&str> = value.split(":").collect();

        Self {
            name: values[0],
            value: values[1].trim()
        }
    }
}

impl <'a>From<(&'a str, &'a str)> for Header<'a> {
    fn from(header: (&'a str, &'a str)) -> Self {
        Self {
            name: header.0,
            value: header.1
        }
    }
}


impl Display for Header<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(format!("{}: {}\n", self.name, self.value).as_str())
    }
}
