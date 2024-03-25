use std::io::Write;

use anyhow::Result;

use crate::{Ident, Value, Writer};

#[derive(Debug, Clone)]
pub struct VariableDeclare {
    pub names: Vec<Ident>,
    pub value: Value,
}

impl VariableDeclare {
    pub fn write<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        writer.write_str("let ")?;
        let skip = self.names.len();

        for (i, v) in self.names.iter().enumerate() {
            v.write(writer)?;

            if skip == i + 1 {
                break;
            }

            writer.write_str(", ")?;
        }

        writer.write_str(" := ")?;
        self.value.write(writer)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub names: Vec<Ident>,
    pub value: Value,
}

impl Assignment {
    pub fn write<W>(&self, writer: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        let skip = self.names.len();

        for (i, v) in self.names.iter().enumerate() {
            v.write(writer)?;

            if skip == i + 1 {
                break;
            }

            writer.write_str(", ")?;
        }

        writer.write_str(" := ")?;
        self.value.write(writer)?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod variable_tests {
    use anyhow::Result;

    use crate::{Assignment, FunctionCall, Ident, VariableDeclare, Writer};

    pub(crate) fn build_vd() -> Result<VariableDeclare> {
        let ident = Ident::new("a")?;

        let value = FunctionCall {
            name: ident.clone(),
            args: Vec::new(),
        };

        Ok(VariableDeclare {
            names: vec![ident.clone(), ident.clone()],
            value: value.into(),
        })
    }

    #[test]
    fn test_vd() {
        let vd = build_vd().unwrap();

        let mut res = Writer::new(Vec::new(), "    ");
        vd.write(&mut res).unwrap();

        assert_eq!(res.w, b"let a, a := a()")
    }

    pub(crate) fn build_as() -> Result<Assignment> {
        let ident = Ident::new("a").unwrap();

        let value = FunctionCall {
            name: ident.clone(),
            args: Vec::new(),
        };

        let vd = Assignment {
            names: vec![ident.clone(), ident.clone()],
            value: value.into(),
        };

        Ok(vd)
    }

    #[test]
    fn test_as() {
        let vd = build_as().unwrap();

        let mut res = Writer::new(Vec::new(), "    ");
        vd.write(&mut res).unwrap();

        assert_eq!(res.w, b"a, a := a()")
    }
}
