use std::io::Write;

use anyhow::Result;

use crate::{Ident, Literal};

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    FunctionCall(FunctionCall),
}

impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl From<FunctionCall> for Value {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}

impl Value {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        match self {
            Self::Literal(v) => v.write(w),
            Self::FunctionCall(v) => v.write(w),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: Ident,
    pub args: Vec<Value>,
}

impl FunctionCall {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        self.name.write(w)?;
        w.write_all(b"(")?;

        let skip = self.args.len();

        for (i, v) in self.args.iter().enumerate() {
            v.write(w)?;

            if skip == i + 1 {
                break;
            }

            w.write_all(b", ")?;
        }

        w.write_all(b")")?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod value_tests {
    use anyhow::Result;

    use crate::{FunctionCall, Ident, Literal, Value};

    pub(crate) fn build_fc() -> Result<FunctionCall> {
        let v0 = Value::Literal(Literal::hex_number("0x1").unwrap());
        let v1 = Value::Literal(Literal::hex_number("0x2").unwrap());
        let v2 = Value::Literal(Literal::hex_number("0x3").unwrap());
        let v3 = Value::Literal(Literal::hex_number("0x4").unwrap());

        let fc = FunctionCall {
            name: Ident::new("_test").unwrap(),
            args: vec![v0.clone(), v1, v2, v3],
        };

        Ok(fc)
    }

    #[test]
    fn test_function_call() {
        let v0 = Value::Literal(Literal::hex_number("0x1").unwrap());
        let v1 = Value::Literal(Literal::hex_number("0x2").unwrap());
        let v2 = Value::Literal(Literal::hex_number("0x3").unwrap());
        let v3 = Value::Literal(Literal::hex_number("0x4").unwrap());

        let fc = FunctionCall {
            name: Ident::new("_test").unwrap(),
            args: vec![v0.clone(), v1, v2, v3],
        };

        let mut res = Vec::new();

        let fc = FunctionCall {
            name: Ident::new("_test").unwrap(),
            args: vec![v0, Value::FunctionCall(fc)],
        };

        fc.write(&mut res).unwrap();

        assert_eq!(res, b"_test(0x1, _test(0x1, 0x2, 0x3, 0x4))")
    }
}
