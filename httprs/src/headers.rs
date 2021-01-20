use std::fmt::Debug;
use std::fmt::Display;
use std::convert::TryFrom;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Header<'a> {
    name: &'a str,
    value: &'a str
}

impl TryFrom<&str> for Header<'_> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values: Vec<&str> = value.split(":").collect();
        dbg!(values);
        if !values.len() > 1 {
            return Err("Header invalido")
        }

        Ok(Self {
            name: values[0],
            value: values[1]
        })
    }
}

impl Display for Header<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(format!("{}: {}\n", self.name, self.value).as_str())
    }
}

impl <'a>Header<'a> {
    /// Cria um objeto do tipo header.
    pub fn new(items: (&'a str, &'a str)) -> Self {
        Header {
            name: items.0,
            value: items.1
        }
    }
}
