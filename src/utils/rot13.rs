use std::convert::From;
use std::fmt::{Formatter, Display, Result as FmtResult};
#[derive(Debug)]
pub struct Rot13(Vec<u8>);

impl Rot13 {
    // https://github.com/marekventur/rust-rot13/blob/master/src/lib.rs
    fn decode(&self) -> String{
        let decoded = (*self.0).into_iter().map(|c| match *c as char {
            'A' ..= 'M' | 'a' ..= 'm' => (*c as u8) + 13,
            'N' ..= 'Z' | 'n' ..= 'z' => (*c as u8) - 13,
            _ => *c
        }).collect::<Vec<u8>>();
        format!("{}",String::from_utf8_lossy(&decoded)).to_owned()
    }
}

impl From<String> for Rot13 {
    fn from(rot13_str: String) -> Self {
        Self(rot13_str.into_bytes())
    }
}

impl From<&str> for Rot13 {
    fn from(rot13_str: &str) -> Self {
        Rot13::from(rot13_str.to_string())
    }
}

impl Display for Rot13 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.decode())
    }
}