use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Alloca, types::Types, Constant, Operand, Type};
use yuler::Statement;

use crate::error;

pub struct AllocaCompiler<'a> {
    inst: &'a Alloca,
    types: &'a Types,
}

impl<'a> AllocaCompiler<'a> {
    pub fn new(inst: &'a Alloca, types: &'a Types) -> Self {
        Self { inst, types }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        log::debug!("{:#?}", self.inst);

        let mut res = Vec::new();

        Ok(res)
    }
}

fn compute_size(ty: &Type, types: &Types) -> Result<u64> {
    Ok(0)
}

fn build_num_elements(num: &Operand) -> Result<u64> {
    match num {
        Operand::LocalOperand { name: _, ty: _ } => Err(anyhow!(
            "{} alloca num_elements: {}",
            error::UNSUPPERTED_OPERAND,
            num
        )),
        Operand::ConstantOperand(r) => {
            if let Constant::Int { bits: _, value } = r.as_ref() {
                Ok(*value)
            } else {
                Err(anyhow!("{} alloca num_elements: {}", error::WRONG_ARG, num))
            }
        }
        _ => Err(anyhow!("{} alloca num_elements: {}", error::WRONG_ARG, num)),
    }
}
