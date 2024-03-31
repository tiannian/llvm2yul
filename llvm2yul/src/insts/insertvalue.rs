use anyhow::Result;
use llvm_ir::instruction::InsertValue;
use yuler::Statement;

pub struct InsertValueCompiler<'a> {
    inst: &'a InsertValue,
}

impl<'a> InsertValueCompiler<'a> {
    pub fn new(inst: &'a InsertValue) -> Self {
        Self { inst }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        Ok(vec![])
    }
}
