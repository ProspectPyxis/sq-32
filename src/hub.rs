use std::io::ErrorKind;
use std::iter::Peekable;
use std::str::Chars;

pub struct HubPair {
    pub key: String,
    pub val: String,
}

pub struct Scanner<'a>(Peekable<Chars<'a>>);

impl Scanner<'_> {
    pub fn new(line: &str) -> Scanner {
        Scanner(line.chars().peekable())
    }

    pub fn get_pair(&mut self) -> Result<HubPair, ErrorKind> {
        let key = self.get_key()?;

        self.skip_whitespaces();

        let val = if let Some('=') = self.0.peek() {
            self.0.next();
            self.get_val()?
        } else {
            return Err(ErrorKind::InvalidData);
        };

        Ok(HubPair { key, val })
    }

    pub fn get_key(&mut self) -> Result<String, ErrorKind> {
        self.skip_whitespaces();

        let mut key = String::new();

        for c in &mut self.0 {
            if Self::is_divider(c) {
                break;
            } else {
                key.push(c);
            }
        }

        if key.is_empty() {
            Err(ErrorKind::InvalidData)
        } else {
            Ok(key)
        }
    }

    pub fn get_val(&mut self) -> Result<String, ErrorKind> {
        self.skip_whitespaces();

        let mut val = String::new();

        if let Some('"') = self.0.peek() {
            self.0.next();
            loop {
                if let Some('"') = self.0.peek() {
                    break;
                }
                match self.0.next() {
                    Some(c) => val.push(c),
                    None => return Err(ErrorKind::InvalidData),
                }
            }
            self.0.next();
        } else {
            val = self.get_key()?;
        }

        Ok(val)
    }

    pub fn skip_whitespaces(&mut self) {
        while let Some(c) = self.0.peek() {
            if c.is_whitespace() {
                self.0.next();
            } else {
                break;
            }
        }
    }

    pub fn is_divider(c: char) -> bool {
        c.is_whitespace() || c == '=' || c == '"'
    }
}