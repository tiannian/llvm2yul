use std::io::Write;

use anyhow::Result;

use crate::{Statement, Writer};

#[derive(Debug, Clone, Default)]
pub struct Block(pub Vec<Statement>);

impl From<Vec<Statement>> for Block {
    fn from(value: Vec<Statement>) -> Self {
        Self(value)
    }
}

impl Block {
    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("{")?;
        w.enter_block();
        w.write_end()?;

        for v in &self.0 {
            v.write(w)?;

            w.write_end()?;
        }

        w.leave_block();
        w.write_str("}")?;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct InlineBlock(pub Vec<Statement>);

impl InlineBlock {
    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("{ ")?;

        for v in &self.0 {
            v.write(w)?;
        }

        w.write_str(" }")?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod block_tests {
    use anyhow::Result;

    use crate::{Ident, InlineBlock, Literal, VariableDeclare};

    pub(crate) fn build_inline_block() -> Result<InlineBlock> {
        let st = VariableDeclare {
            names: vec![Ident::new("i")?],
            value: Literal::int_number("0")?.into(),
        }
        .into();

        let iblk = InlineBlock(vec![st]);

        Ok(iblk)
    }
}
