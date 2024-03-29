use anyhow::{anyhow, Result};
use llvm_ir::{
    instruction::{Alloca, Call, ExtractValue, InsertValue, IntToPtr, Load, Phi, Select, Store},
    BasicBlock, Instruction,
};
use yuler::Statement;

use crate::{CallCompiler, ExtendedArgsMap};

pub struct BlockCompiler<'a> {
    bb: &'a BasicBlock,
    extended_args: &'a ExtendedArgsMap,
}

impl<'a> BlockCompiler<'a> {
    pub fn new(bb: &'a BasicBlock, extended_args: &'a ExtendedArgsMap) -> Self {
        Self { bb, extended_args }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        let mut stmts = Vec::new();

        for inst in &self.bb.instrs {
            let i = self.compile_inst(inst)?;
            stmts.push(i);
        }

        Ok(stmts)
    }

    pub fn compile_inst(&self, inst: &Instruction) -> Result<Statement> {
        match inst {
            Instruction::Phi(i) => self.compile_phi(i),
            Instruction::Call(i) => self.compile_call(i),
            Instruction::Alloca(i) => self.compile_alloca(i),
            Instruction::Load(i) => self.compile_load(i),
            Instruction::Store(i) => self.compile_store(i),
            Instruction::Select(i) => self.compile_select(i),
            Instruction::ExtractValue(i) => self.compile_extract_value(i),
            Instruction::InsertValue(i) => self.compile_insert_value(i),
            Instruction::IntToPtr(i) => self.compile_int2ptr(i),
            _ => Err(anyhow!("Unsupported instruction: {}", inst)),
        }
    }

    fn compile_phi(&self, _inst: &Phi) -> Result<Statement> {
        Ok(Statement::Break)
    }

    fn compile_call(&self, call: &Call) -> Result<Statement> {
        let compiler = CallCompiler::new(call, self.extended_args);

        compiler.compile_call()
    }
    fn compile_alloca(&self, _inst: &Alloca) -> Result<Statement> {
        Ok(Statement::Break)
    }
    fn compile_load(&self, _inst: &Load) -> Result<Statement> {
        Ok(Statement::Break)
    }
    fn compile_store(&self, _inst: &Store) -> Result<Statement> {
        Ok(Statement::Break)
    }
    fn compile_select(&self, _inst: &Select) -> Result<Statement> {
        Ok(Statement::Break)
    }
    fn compile_extract_value(&self, _inst: &ExtractValue) -> Result<Statement> {
        Ok(Statement::Break)
    }

    fn compile_insert_value(&self, _inst: &InsertValue) -> Result<Statement> {
        Ok(Statement::Break)
    }

    fn compile_int2ptr(&self, _inst: &IntToPtr) -> Result<Statement> {
        Ok(Statement::Break)
    }
}
