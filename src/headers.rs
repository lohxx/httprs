use std::fmt::Debug;
use std::fmt::Display;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Headers<'a> {
    name: &'a str,
    value: &'a str
}

impl Display for Headers<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(format!("{}: {}\n", self.name, self.value).as_str())
    }
}

impl <'a>Headers<'a> {
    /// Cria um objeto do tipo header.
    pub fn new(items: (&'a str, &'a str)) -> Self {
        Headers {
            name: items.0,
            value: items.1
        }
    }
}
