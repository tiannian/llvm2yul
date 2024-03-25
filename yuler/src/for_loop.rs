use std::io::Write;

use anyhow::Result;

use crate::{Block, InlineBlock, Value, Writer};

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub init: InlineBlock,
    pub cond: Value,
    pub incr: InlineBlock,
    pub block: Block,
}

impl ForLoop {
    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("for ")?;
        self.init.write(w)?;
        w.write_str(" ")?;
        self.cond.write(w)?;
        w.write_str(" ")?;
        self.incr.write(w)?;
        w.write_str(" ")?;
        self.block.write(w)?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod for_loop_tests {
    use anyhow::Result;

    use crate::{
        block_tests::build_inline_block, stmt_tests::build_block, value_tests::build_fc, ForLoop,
    };

    pub(crate) fn build_for_loop() -> Result<ForLoop> {
        let ib = build_inline_block()?;
        let cond = build_fc()?.into();
        let block = build_block()?;

        let fl = ForLoop {
            cond,
            init: ib.clone(),
            incr: ib.clone(),
            block,
        };

        Ok(fl)
    }
}
