use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Alloca, types::Types, Constant, Operand};
use yuler::{FunctionCall, Ident, Literal, Statement, VariableDeclare};

use crate::{error, utils, Config, TypeFlatter};

pub struct AllocaCompiler<'a> {
    inst: &'a Alloca,
    types: &'a Types,
    config: &'a Config,
}

impl<'a> AllocaCompiler<'a> {
    pub fn new(inst: &'a Alloca, types: &'a Types, config: &'a Config) -> Self {
        Self {
            inst,
            types,
            config,
        }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        let flatter = TypeFlatter::new(self.types, self.config);
        let size = flatter.compute_size(&self.inst.allocated_type)?;

        let num_elem = build_num_elements(&self.inst.num_elements)?;

        let total_size = size * num_elem;

        let dest = utils::yul_ident_name(&self.inst.dest);
        let dest = Ident::new(utils::tidy_name(&dest))?;

        let value = FunctionCall {
            name: Ident::new("__yul_allocate")?,
            args: vec![Literal::int_number(format!("{}", total_size))?.into()],
        }
        .into();

        Ok(vec![VariableDeclare {
            names: vec![dest],
            value,
        }
        .into()])
    }
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
