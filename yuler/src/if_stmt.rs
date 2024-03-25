use std::io::Write;

use anyhow::Result;

use crate::{Block, Value, Writer};

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Value,
    pub block: Block,
}

impl If {
    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("if ")?;
        self.cond.write(w)?;
        w.write_str(" ")?;
        self.block.write(w)?;

        Ok(())
    }
}
