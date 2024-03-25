use std::io::Write;

use anyhow::Result;

use crate::{Ident, Value};

#[derive(Debug, Clone)]
pub struct VariableDeclare {
    pub names: Vec<Ident>,
    pub value: Value,
}

impl VariableDeclare {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(b"let ")?;
        let skip = self.names.len();

        for (i, v) in self.names.iter().enumerate() {
            v.write(w)?;

            if skip == i + 1 {
                break;
            }

            w.write_all(b", ")?;
        }

        w.write_all(b" := ")?;
        self.value.write(w)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub names: Vec<Ident>,
    pub value: Value,
}

impl Assignment {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        let skip = self.names.len();

        for (i, v) in self.names.iter().enumerate() {
            v.write(w)?;

            if skip == i + 1 {
                break;
            }

            w.write_all(b", ")?;
        }

        w.write_all(b" := ")?;
        self.value.write(w)?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod variable_tests {
    use anyhow::Result;

    use crate::{Assignment, FunctionCall, Ident, VariableDeclare};

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

        let mut res = Vec::new();
        vd.write(&mut res).unwrap();

        assert_eq!(res, b"let a, a := a()")
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

        let mut res = Vec::new();
        vd.write(&mut res).unwrap();

        assert_eq!(res, b"a, a := a()")
    }
}
