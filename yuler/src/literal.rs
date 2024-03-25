use std::io::Write;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEX_LITERAL: Regex = Regex::new(r"^[0-9A-Fa-f]+$").unwrap();
    static ref INT_LITERAL: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
    static ref HEX_NUMBER_LITERAL: Regex = Regex::new(r"^0x[0-9A-Fa-f]+$").unwrap();
}

pub enum Literal {
    Number(NumberLiteral),
    ASCII(ASCIILiteral),
    Hex(HexLiteral),
}

impl Literal {
    pub fn int_number(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        if INT_LITERAL.is_match(&s) {
            Ok(Self::Number(NumberLiteral(s)))
        } else {
            Err(anyhow!("Wrong format of number"))
        }
    }

    pub fn hex_number(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        if HEX_NUMBER_LITERAL.is_match(&s) {
            Ok(Self::Number(NumberLiteral(s)))
        } else {
            Err(anyhow!("Wrong format of number"))
        }
    }

    pub fn ascii(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        Ok(Self::ASCII(ASCIILiteral(s)))
    }

    pub fn hex_data(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        if HEX_LITERAL.is_match(&s) {
            Ok(Self::Hex(HexLiteral(s)))
        } else {
            Err(anyhow!("Wrong format of number"))
        }
    }

    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        match self {
            Self::Number(v) => v.write(w),
            Self::Hex(v) => v.write(w),
            Self::ASCII(v) => v.write(w),
        }
    }
}

pub struct NumberLiteral(pub(crate) String);

impl NumberLiteral {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        Ok(Self(s.into()))
    }

    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(self.0.as_bytes())?;
        Ok(())
    }
}

pub struct ASCIILiteral(pub(crate) String);

impl ASCIILiteral {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        Ok(Self(s.into()))
    }

    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        w.write_fmt(format_args!("\"{}\"", self.0))?;
        Ok(())
    }
}

pub struct HexLiteral(pub(crate) String);

impl HexLiteral {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        if s.len() % 2 == 0 && HEX_LITERAL.is_match(&s) {
            Ok(Self(s))
        } else {
            Err(anyhow!("Wrong format of hex string"))
        }
    }

    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        w.write_fmt(format_args!("hex\"{}\"", self.0))?;
        Ok(())
    }
}

pub struct Ident(pub String);

impl Ident {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        Ok(Self(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Literal;

    #[test]
    fn test_number() {
        let number = Literal::int_number("123").unwrap();

        let mut res = Vec::new();

        number.write(&mut res).unwrap();

        assert_eq!(res, b"123");
    }

    #[test]
    fn test_int_number() {
        let number = Literal::hex_number("0x123").unwrap();

        let mut res = Vec::new();

        number.write(&mut res).unwrap();

        assert_eq!(res, b"0x123");
    }

    #[test]
    fn test_ascii() {
        let number = Literal::ascii("abcd").unwrap();

        let mut res = Vec::new();

        number.write(&mut res).unwrap();

        assert_eq!(res, b"\"abcd\"");
    }

    #[test]
    fn test_hex() {
        let number = Literal::hex_data("abcd").unwrap();

        let mut res = Vec::new();

        number.write(&mut res).unwrap();

        assert_eq!(res, b"hex\"abcd\"");
    }
}
