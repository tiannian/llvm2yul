use std::collections::BTreeSet;

use anyhow::{anyhow, Result};
use llvm_ir::{
    instruction::{Alloca, Call, ExtractValue, InsertValue, IntToPtr, Phi, PtrToInt, Select},
    types::Types,
    BasicBlock, Instruction,
};
use yuler::Statement;

use crate::{
    AllocaCompiler, CallCompiler, Config, ExtractValueCompiler, PtrIntCompiler, SelectCompiler,
};

pub struct BlockCompiler<'a> {
    bb: &'a BasicBlock,
    llvm_types: &'a Types,
    config: &'a Config,
    pub(crate) objects: BTreeSet<String>,
}

impl<'a> BlockCompiler<'a> {
    pub fn new(bb: &'a BasicBlock, llvm_types: &'a Types, config: &'a Config) -> Self {
        Self {
            bb,
            llvm_types,
            config,

            objects: Default::default(),
        }
    }

    pub fn compile(&mut self) -> Result<Vec<Statement>> {
        let mut stmts = Vec::new();

        for inst in &self.bb.instrs {
            let mut i = self.compile_inst(inst)?;
            stmts.append(&mut i);
        }

        Ok(stmts)
    }

    pub fn compile_inst(&mut self, inst: &Instruction) -> Result<Vec<Statement>> {
        let res = match inst {
            Instruction::ExtractValue(i) => self.compile_extract_value(i)?,
            Instruction::InsertValue(i) => self.compile_insert_value(i)?,
            Instruction::Alloca(i) => self.compile_alloca(i)?,
            Instruction::Phi(i) => self.compile_phi(i)?,
            Instruction::Call(i) => self.compile_call(i)?,
            Instruction::Select(i) => self.compile_select(i)?,
            Instruction::IntToPtr(i) => self.compile_int2ptr(i)?,
            Instruction::PtrToInt(i) => self.compile_ptr2int(i)?,
            _ => return Err(anyhow!("Unsupported instruction: {}", inst)),
        };

        Ok(res)
    }

    fn compile_phi(&self, _inst: &Phi) -> Result<Vec<Statement>> {
        Ok(vec![])
    }

    fn compile_call(&mut self, call: &Call) -> Result<Vec<Statement>> {
        let mut compiler = CallCompiler::new(call, self.llvm_types, self.config);

        let stmt = compiler.compile_call()?;

        if let Some(object) = compiler.object {
            self.objects.insert(object);
        }

        Ok(vec![stmt])
    }

    fn compile_alloca(&self, inst: &Alloca) -> Result<Vec<Statement>> {
        let compiler = AllocaCompiler::new(inst, self.llvm_types, self.config);

        compiler.compile()
    }

    fn compile_select(&self, inst: &Select) -> Result<Vec<Statement>> {
        let select = SelectCompiler::new(inst);

        select.compile()
    }

    fn compile_extract_value(&self, inst: &ExtractValue) -> Result<Vec<Statement>> {
        let compiler = ExtractValueCompiler::new(inst);

        compiler.compile()
    }

    fn compile_insert_value(&self, _inst: &InsertValue) -> Result<Vec<Statement>> {
        Ok(vec![])
    }

    fn compile_int2ptr(&self, inst: &IntToPtr) -> Result<Vec<Statement>> {
        let compiler = PtrIntCompiler::new(&inst.operand, &inst.dest);

        compiler.compile()
    }

    fn compile_ptr2int(&self, inst: &PtrToInt) -> Result<Vec<Statement>> {
        let compiler = PtrIntCompiler::new(&inst.operand, &inst.dest);

        compiler.compile()
    }
}
