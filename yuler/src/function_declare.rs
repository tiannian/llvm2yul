use std::io::Write;

use anyhow::Result;

use crate::{Block, Ident, Writer};

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub rets: Vec<Ident>,
    pub block: Block,
}

impl FunctionDefinition {
    pub fn new(name: Ident) -> Self {
        Self {
            name,
            args: Default::default(),
            rets: Default::default(),
            block: Default::default(),
        }
    }

    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        w.write_str("function ")?;
        self.name.write(w)?;
        w.write_str("(")?;

        let skip = self.args.len();
        for (i, v) in self.args.iter().enumerate() {
            v.write(w)?;

            if skip == i + 1 {
                break;
            }

            w.write_str(", ")?;
        }
        w.write_str(") ")?;

        // Returns
        if !self.rets.is_empty() {
            w.write_str("->")?;

            let skip = self.rets.len();
            for (i, v) in self.rets.iter().enumerate() {
                v.write(w)?;

                if skip == i + 1 {
                    break;
                }

                w.write_str(", ")?;
            }

            w.write_str(" ")?;
        }

        self.block.write(w)?;
        w.write_end()?;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod function_defination_tests {
    use anyhow::Result;

    use crate::{stmt_tests::build_block, FunctionDefinition, Ident};

    pub fn build_function_defination() -> Result<FunctionDefinition> {
        let name = Ident::new("test")?;
        let a = Ident::new("a")?;

        let args = vec![a.clone(), a.clone(), a.clone()];
        let rets = vec![a.clone(), a.clone(), a];
        let block = build_block()?;

        let r = FunctionDefinition {
            args,
            name,
            rets,
            block,
        };

        Ok(r)
    }
}
