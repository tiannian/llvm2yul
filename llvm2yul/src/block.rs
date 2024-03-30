use anyhow::{anyhow, Result};
use llvm_ir::{
    instruction::{Alloca, Call, ExtractValue, InsertValue, IntToPtr, Phi, Select},
    BasicBlock, Instruction, Operand, Type, TypeRef,
};
use yuler::{FunctionCall, Ident, Literal, Statement, VariableDeclare};

use crate::{utils, CallCompiler, SelectCompiler};

pub struct BlockCompiler<'a> {
    bb: &'a BasicBlock,
}

impl<'a> BlockCompiler<'a> {
    pub fn new(bb: &'a BasicBlock) -> Self {
        Self { bb }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        let mut stmts = Vec::new();

        for inst in &self.bb.instrs {
            let mut i = self.compile_inst(inst)?;
            stmts.append(&mut i);
        }

        Ok(stmts)
    }

    pub fn compile_inst(&self, inst: &Instruction) -> Result<Vec<Statement>> {
        let res = match inst {
            Instruction::Phi(i) => vec![self.compile_phi(i)?],
            Instruction::Call(i) => vec![self.compile_call(i)?],
            Instruction::Alloca(i) => vec![self.compile_alloca(i)?],
            Instruction::Select(i) => self.compile_select(i)?,
            Instruction::ExtractValue(i) => vec![self.compile_extract_value(i)?],
            Instruction::InsertValue(i) => vec![self.compile_insert_value(i)?],
            Instruction::IntToPtr(i) => vec![self.compile_int2ptr(i)?],
            _ => return Err(anyhow!("Unsupported instruction: {}", inst)),
        };

        Ok(res)
    }

    fn compile_phi(&self, _inst: &Phi) -> Result<Statement> {
        Ok(Statement::Break)
    }

    fn compile_call(&self, call: &Call) -> Result<Statement> {
        let compiler = CallCompiler::new(call);

        compiler.compile_call()
    }

    fn compile_alloca(&self, inst: &Alloca) -> Result<Statement> {
        let num = if let Type::ArrayType {
            element_type,
            num_elements,
        } = inst.allocated_type.as_ref()
        {
            if let Type::IntegerType { bits } = element_type.as_ref() {
                (bits / 8) * (*num_elements as u32)
            } else {
                return Err(anyhow!("Unsupported alloc type: {inst}"));
            }
        } else {
            return Err(anyhow!("Unsupported alloc type: {inst}"));
        };

        let dest = utils::yul_ident_name(&inst.dest);
        let value = FunctionCall {
            name: Ident::new("__yul_allocate")?,
            args: vec![Literal::int_number(format!("{}", num))?.into()],
        }
        .into();

        Ok(VariableDeclare {
            names: vec![Ident::new(dest)?],
            value,
        }
        .into())
    }

    fn compile_select(&self, inst: &Select) -> Result<Vec<Statement>> {
        let select = SelectCompiler::new(inst);

        select.compile()
    }

    fn compile_extract_value(&self, inst: &ExtractValue) -> Result<Statement> {
        // TODO: Add flat deep struct

        if let Operand::LocalOperand { name, ty: _ } = &inst.aggregate {
            if inst.indices.len() == 1 {
                let name = utils::yul_ident_name(name);

                let index = inst.indices[0];

                let name = if index == 0 {
                    name
                } else {
                    format!("{}_{}", name, index)
                };

                let dest = utils::yul_ident_name(&inst.dest);

                Ok(VariableDeclare {
                    names: vec![Ident::new(dest)?],
                    value: Ident::new(name)?.into(),
                }
                .into())
            } else {
                Err(anyhow!("Unsupported multi indices for extract value"))
            }
        } else {
            Err(anyhow!("Unsupported struct type on extract value."))
        }
    }

    fn compile_insert_value(&self, _inst: &InsertValue) -> Result<Statement> {
        Ok(Statement::Break)
    }

    fn compile_int2ptr(&self, _inst: &IntToPtr) -> Result<Statement> {
        Ok(Statement::Break)
    }
}
