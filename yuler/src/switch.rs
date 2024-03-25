use std::io::Write;

use anyhow::Result;

use crate::{Block, Literal, Value, Writer};

#[derive(Debug, Clone)]
pub struct Switch {
    pub cond: Value,
    pub cases: Vec<CaseBlock>,
    pub default: Option<Block>,
}

impl Switch {
    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("switch ")?;
        self.cond.write(w)?;
        w.write_end()?;

        for v in &self.cases {
            v.write(w)?;
        }

        if let Some(v) = &self.default {
            v.write(w)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CaseBlock {
    pub cond: Literal,
    pub block: Block,
}

impl CaseBlock {
    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("case ")?;
        self.cond.write(&mut w.w)?;
        w.write_str(" ")?;
        self.block.write(w)?;

        Ok(())
    }
}
