use std::io::Write;

use anyhow::Result;

use crate::Writer;

#[derive(Debug, Clone)]
pub struct Ident(pub String);

impl Ident {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        Ok(Self(s.into()))
    }

    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str(&self.0)?;
        Ok(())
    }

    pub fn write_qoute<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("\"")?;
        w.write_str(&self.0)?;
        w.write_str("\"")?;
        Ok(())
    }
}
