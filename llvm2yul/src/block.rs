use anyhow::{anyhow, Result};
use llvm_ir::{
    instruction::{Alloca, Call, ExtractValue, InsertValue, IntToPtr, Phi, PtrToInt, Select},
    BasicBlock, Constant, Instruction, Operand, Type,
};
use yuler::{FunctionCall, Ident, Literal, Statement, Value, VariableDeclare};

use crate::{utils, CallCompiler, ExtractValueCompiler, SelectCompiler};

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

    fn compile_call(&self, _call: &Call) -> Result<Vec<Statement>> {
        // let compiler = CallCompiler::new(call);
        //
        // Ok(vec![compiler.compile_call()?])
        Ok(vec![])
    }

    fn compile_alloca(&self, inst: &Alloca) -> Result<Vec<Statement>> {
        // let num = if let Type::ArrayType {
        //     element_type,
        //     num_elements,
        // } = inst.allocated_type.as_ref()
        // {
        //     if let Type::IntegerType { bits } = element_type.as_ref() {
        //         (bits / 8) * (*num_elements as u32)
        //     } else {
        //         return Err(anyhow!("Unsupported alloc type: {inst}"));
        //     }
        // } else {
        //     return Err(anyhow!("Unsupported alloc type: {inst}"));
        // };
        //
        // let dest = utils::yul_ident_name(&inst.dest);
        // let value = FunctionCall {
        //     name: Ident::new("__yul_allocate")?,
        //     args: vec![Literal::int_number(format!("{}", num))?.into()],
        // }
        // .into();
        //
        // Ok(VariableDeclare {
        //     names: vec![Ident::new(dest)?],
        //     value,
        // }
        // .into())
        Ok(vec![])
    }

    fn compile_select(&self, inst: &Select) -> Result<Vec<Statement>> {
        // let select = SelectCompiler::new(inst);

        // select.compile()
        Ok(vec![])
    }

    fn compile_extract_value(&self, inst: &ExtractValue) -> Result<Vec<Statement>> {
        let compiler = ExtractValueCompiler::new(inst);

        compiler.compile()
    }

    fn compile_insert_value(&self, inst: &InsertValue) -> Result<Vec<Statement>> {
        Ok(vec![])
    }

    fn compile_int2ptr(&self, inst: &IntToPtr) -> Result<Vec<Statement>> {
        // let dest = Ident::new(utils::yul_ident_name(&inst.dest))?;
        //
        // let value: Value = match &inst.operand {
        //     Operand::LocalOperand { name, ty: _ } => {
        //         let name = Ident::new(utils::yul_ident_name(name))?;
        //
        //         name.into()
        //     }
        //     Operand::ConstantOperand(constant) => match constant.as_ref() {
        //         Constant::Int { bits: _, value } => Literal::int_number(format!("{value}"))?.into(),
        //         Constant::Null(_) => Literal::int_number("0")?.into(),
        //         _ => return Err(anyhow!("Unsupported constant type")),
        //     },
        //     _ => return Err(anyhow!("Unsupported operand for select")),
        // };
        //
        // Ok(vec![VariableDeclare {
        //     names: vec![dest],
        //     value,
        // }
        // .into()])
        Ok(vec![])
    }

    fn compile_ptr2int(&self, _inst: &PtrToInt) -> Result<Vec<Statement>> {
        Ok(vec![])
    }
}
