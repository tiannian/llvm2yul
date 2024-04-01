use std::io::Write;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use literals::{ASCIILiteral, NumberLiteral};
use regex::Regex;

use self::literals::HexNumberLiteral;

lazy_static! {
    static ref HEX_LITERAL: Regex = Regex::new(r"^[0-9A-Fa-f]+$").unwrap();
    static ref INT_LITERAL: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
    static ref HEX_NUMBER_LITERAL: Regex = Regex::new(r"^0x[0-9A-Fa-f]+$").unwrap();
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(NumberLiteral),
    HexNumber(HexNumberLiteral),
    ASCII(ASCIILiteral),
}

impl Literal {
    pub fn int_number(s: u64) -> Result<Self> {
        Ok(Self::Number(NumberLiteral(s)))
    }

    pub fn hex_number(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        if HEX_NUMBER_LITERAL.is_match(&s) {
            Ok(Self::HexNumber(HexNumberLiteral(s)))
        } else {
            Err(anyhow!("Wrong format of number"))
        }
    }

    pub fn ascii(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        Ok(Self::ASCII(ASCIILiteral(s)))
    }

    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        match self {
            Self::Number(v) => v.write(w),
            Self::HexNumber(v) => v.write(w),
            Self::ASCII(v) => v.write(w),
        }
    }

    pub fn as_number(&self) -> Option<u64> {
        match self {
            Literal::Number(n) => Some(n.0),
            _ => None,
        }
    }

    pub fn as_ascii(&self) -> Option<&str> {
        match self {
            Literal::ASCII(n) => Some(&n.0),
            _ => None,
        }
    }
}

pub mod literals {
    use std::io::Write;

    use anyhow::Result;

    #[derive(Debug, Clone)]
    pub struct NumberLiteral(pub(crate) u64);

    impl NumberLiteral {
        pub fn write(&self, w: &mut impl Write) -> Result<()> {
            w.write_all(format!("{}", self.0).as_bytes())?;
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub struct HexNumberLiteral(pub(crate) String);

    impl HexNumberLiteral {
        pub fn write(&self, w: &mut impl Write) -> Result<()> {
            w.write_all(self.0.to_string().as_bytes())?;
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    pub struct ASCIILiteral(pub(crate) String);

    impl ASCIILiteral {
        pub fn write(&self, w: &mut impl Write) -> Result<()> {
            w.write_fmt(format_args!("\"{}\"", self.0))?;
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct HexLiteral(pub(crate) String);

impl HexLiteral {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        let s = s.into();

        if HEX_LITERAL.is_match(&s) {
            Ok(HexLiteral(s))
        } else {
            Err(anyhow!("Wrong format of number"))
        }
    }

    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        w.write_fmt(format_args!("hex\"{}\"", self.0))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{HexLiteral, Literal};

    #[test]
    fn test_number() {
        let number = Literal::int_number(123).unwrap();

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
        let number = HexLiteral::new("abcd").unwrap();

        let mut res = Vec::new();

        number.write(&mut res).unwrap();

        assert_eq!(res, b"hex\"abcd\"");
    }
}
