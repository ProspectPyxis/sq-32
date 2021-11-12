use crate::error::HubError;
use std::iter::Peekable;
use std::str::Chars;

pub struct HubPair {
    pub key: String,
    pub val: Option<String>,
}

pub struct Scanner<'a>(Peekable<Chars<'a>>);

impl HubPair {
    pub fn unwrap_val(self) -> Result<String, HubError> {
        self.val.ok_or(HubError::NoValue)
    }
}

impl Scanner<'_> {
    pub fn new(line: &str) -> Scanner {
        Scanner(line.chars().peekable())
    }

    pub fn get_pair(&mut self) -> Result<HubPair, HubError> {
        let key = self.get_key()?;

        self.skip_whitespaces();

        let val = if let Some('=') = self.0.peek() {
            self.0.next();
            Some(self.get_val()?)
        } else {
            None
        };

        Ok(HubPair { key, val })
    }

    pub fn get_key(&mut self) -> Result<String, HubError> {
        self.skip_whitespaces();

        let mut key = String::new();

        for c in &mut self.0 {
            if is_divider(c) {
                break;
            } else {
                key.push(c);
            }
        }

        if key.is_empty() {
            Err(HubError::NoNextToken)
        } else {
            Ok(key)
        }
    }

    pub fn get_val(&mut self) -> Result<String, HubError> {
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
                    None => return Err(HubError::UnclosedQuote),
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

    pub fn is_done(&mut self) -> bool {
        self.0.peek().is_none()
    }
}

fn is_divider(c: char) -> bool {
    c.is_whitespace() || c == '=' || c == '"'
}
