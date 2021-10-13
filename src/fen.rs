use crate::error::FenValidationError;
use std::iter::Peekable;
use std::str::Chars;

pub struct FenProcessor<'a> {
    fen: String,
    iter: Peekable<Chars<'a>>,
}

impl FenProcessor<'_> {
    pub fn new(fen: &str) -> FenProcessor {
        FenProcessor {
            fen: fen.to_uppercase(),
            iter: fen.chars().peekable(),
        }
    }

    pub fn validate(&mut self) -> Result<&str, FenValidationError> {
        if !self.fen.is_ascii() {
            return Err(FenValidationError::NotAscii);
        }

        let fen_fields = self.fen.split(':').count();
        if fen_fields != 5 {
            return Err(FenValidationError::IncorrectFieldCount(fen_fields));
        }

        self.match_first(0, vec!['W', 'B'])?;

        if self.next_or_err()? != ':' {
            return Err(FenValidationError::InvalidFieldLength {
                expected: 1,
                found: self
                    .fen
                    .split(':')
                    .next()
                    .expect("should never fail here")
                    .len(),
            });
        }

        let mut board: u32 = 0;

        self.match_first(1, vec!['W'])?;
        self.check_piece_list(&mut board)?;
        self.match_first(2, vec!['B'])?;
        self.check_piece_list(&mut board)?;

        self.match_first(3, vec!['H'])?;
        self.parse_clock()?;
        self.match_first(4, vec!['F'])?;
        self.parse_clock()?;

        Ok(self.fen.as_str())
    }

    fn next_or_err(&mut self) -> Result<char, FenValidationError> {
        self.iter.next().ok_or(FenValidationError::TerminatedEarly)
    }

    fn match_first(&mut self, field: usize, expected: Vec<char>) -> Result<(), FenValidationError> {
        let found = self.next_or_err()?;
        if !expected.contains(&found) {
            Err(FenValidationError::InvalidFieldStart {
                field,
                expected,
                found,
            })
        } else {
            Ok(())
        }
    }

    fn parse_or_err(intstring: String) -> Result<usize, FenValidationError> {
        match intstring.parse::<usize>() {
            Ok(num) => Ok(num),
            Err(error) => Err(FenValidationError::ParseIntError { intstring, error }),
        }
    }

    fn check_piece_list(&mut self, b: &mut u32) -> Result<(), FenValidationError> {
        if self.iter.peek().is_some() && *self.iter.peek().unwrap() != ':' {
            let mut intstring = String::new();
            loop {
                let c = self.next_or_err()?;
                if c == ',' || c == ':' {
                    let val = FenProcessor::parse_or_err(intstring)?;

                    if !(1..=32).contains(&val) {
                        return Err(FenValidationError::PosOutOfBounds { pos: val, max: 32 });
                    }
                    if *b & (1 << (val - 1)) != 0 {
                        return Err(FenValidationError::InvalidSquare(val));
                    }
                    *b |= 1 << (val - 1);
                    intstring = String::new();

                    if c == ':' {
                        break;
                    }
                } else if !(c == 'K' && intstring.is_empty()) {
                    intstring.push(c);
                }
            }
        }

        Ok(())
    }

    fn parse_clock(&mut self) -> Result<(), FenValidationError> {
        let mut intstring = String::new();
        loop {
            let c = self.iter.next();
            if let None | Some(':') = c {
                FenProcessor::parse_or_err(intstring)?;
                break;
            } else {
                intstring.push(c.unwrap());
            }
        }
        Ok(())
    }
}
