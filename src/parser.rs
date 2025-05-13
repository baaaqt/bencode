use std::collections::HashMap;

use crate::error::Error;
use crate::value::BencodeValue;

pub struct Parser {
    data: Vec<u8>,
    position: usize,
}

type Result<T> = std::result::Result<T, Error>;

impl Parser {
    pub fn new(data: Vec<u8>) -> Self {
        Parser { data, position: 0 }
    }

    pub fn parse(&mut self) -> Result<BencodeValue> {
        let c = self.data[self.position];
        match c {
            b'i' => self.parse_int(),
            b'l' => self.parse_list(),
            b'd' => self.parse_dictionary(),
            b'0'..=b'9' => self.parse_string(),
            _ => {
                return Err(Error::new(
                    crate::error::ErrorCode::UnexpectedCharacter,
                    self.position,
                    format!("Unexpected character '{}' at {}", c, self.position),
                ));
            }
        }
    }

    fn parse_int(&mut self) -> Result<BencodeValue> {
        self.position += 1;
        let parsed = self
            .parse_int_string_until(b'e')?
            .parse::<i64>()
            .map_err(|err| {
                Error::new(
                    crate::error::ErrorCode::IntegerParseError,
                    self.position,
                    format!("Can't parse integer at {}: {}", self.position, err),
                )
            })?;
        Ok(BencodeValue::Integer(parsed))
    }

    fn parse_int_string_until(&mut self, until: u8) -> Result<String> {
        let end_of_int = self.data[self.position..]
            .iter()
            .position(|&x| x == until)
            .ok_or_else(|| {
                Error::new(
                    crate::error::ErrorCode::IntegerParseError,
                    self.position,
                    format!("Missing '{}' after integer", until as char),
                )
            })?;
        let res = String::from_utf8(self.data[self.position..self.position + end_of_int].to_vec())
            .map_err(|err| {
                Error::new(
                    crate::error::ErrorCode::IntegerParseError,
                    self.position,
                    format!("Can't parse integer at {}: {}", self.position, err),
                )
            })?;
        self.position += end_of_int + 1;
        Ok(res)
    }

    fn parse_string(&mut self) -> Result<BencodeValue> {
        let start_pos = self.position;
        let length = self
            .parse_int_string_until(b':')?
            .parse::<usize>()
            .map_err(|err| {
                Error::new(
                    crate::error::ErrorCode::StringParseError,
                    start_pos,
                    format!("Can't parse string length at {}: {}", start_pos, err),
                )
            })?;

        let parsed = self.data[self.position..self.position + length].to_vec();
        self.position += length;
        Ok(BencodeValue::ByteString(parsed))
    }

    fn parse_dictionary(&mut self) -> Result<BencodeValue> {
        self.position += 1;
        let mut obj = HashMap::new();
        while self.data[self.position] != b'e' {
            if let BencodeValue::ByteString(key) = self.parse_string()? {
                let value = self.parse();
                if value.is_err() {
                    return Err(Error::new(
                        crate::error::ErrorCode::InvalidDictionary,
                        self.position,
                        format!("Can't parse dictionary value at {}", self.position),
                    ));
                }

                obj.insert(key, value.unwrap());
            } else {
                return Err(Error::new(
                    crate::error::ErrorCode::InvalidDictionary,
                    self.position,
                    format!("Invalid dictionary key at {}", self.position),
                ));
            }
        }
        self.position += 1;
        return Ok(BencodeValue::Dict(obj));
    }

    fn parse_list(&mut self) -> Result<BencodeValue> {
        self.position += 1;
        let mut obj = Vec::new();
        while self.data[self.position] != b'e' {
            let value = self.parse()?;
            obj.push(value);
        }
        self.position += 1;
        return Ok(BencodeValue::List(obj));
    }
}
