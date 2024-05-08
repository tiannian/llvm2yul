use anyhow::Result;
use llvm_ir::instruction::GetElementPtr;
use yuler::Statement;

pub struct GetElementPtrCompiler<'a> {
    inst: &'a GetElementPtr,
}

impl<'a> GetElementPtrCompiler<'a> {
    pub fn new(inst: &'a GetElementPtr) -> Self {
        Self { inst }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        println!("{:#?}", self.inst);

        Ok(vec![])
    }
}
