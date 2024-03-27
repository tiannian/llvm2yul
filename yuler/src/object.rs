use std::io::Write;

use anyhow::Result;

use crate::{Block, HexLiteral, Ident, Writer};

#[derive(Debug)]
pub struct Object {
    pub name: Ident,
    pub code: Block,
    pub data: Vec<Data>,
    pub objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Data {
    pub name: Ident,
    pub data: HexLiteral,
}

impl Object {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            code: Block(Vec::new()),
            data: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("object ")?;
        self.name.write_qoute(w)?;
        w.write_str(" {")?;
        w.write_end()?;

        w.enter_block();

        w.write_str("code ")?;
        self.code.write(w)?;

        for v in &self.data {
            w.write_str("data ")?;
            v.name.write_qoute(w)?;
            w.write_str(" ")?;
            v.data.write(&mut w.w)?;
            w.write_end()?;
        }
        w.write_end()?;

        w.leave_block();
        w.write_str("}")?;

        Ok(())
    }
}
