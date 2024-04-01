use anyhow::{anyhow, Result};
use llvm_ir::{Constant, Name, Operand};
use yuler::{Ident, Literal, Statement, Value, VariableDeclare};

use crate::utils;

pub struct PtrIntCompiler<'a> {
    operand: &'a Operand,
    dest: &'a Name,
}

impl<'a> PtrIntCompiler<'a> {
    pub fn new(operand: &'a Operand, dest: &'a Name) -> Self {
        Self { operand, dest }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        let dest = Ident::new(utils::yul_ident_name(self.dest))?;

        let value: Value = match &self.operand {
            Operand::LocalOperand { name, ty: _ } => {
                let name = Ident::new(utils::yul_ident_name(name))?;

                name.into()
            }
            Operand::ConstantOperand(constant) => match constant.as_ref() {
                Constant::Int { bits: _, value } => Literal::int_number(format!("{value}"))?.into(),
                Constant::Null(_) => Literal::int_number("0")?.into(),
                _ => return Err(anyhow!("Unsupported constant type")),
            },
            _ => return Err(anyhow!("Unsupported operand for select")),
        };

        Ok(vec![VariableDeclare {
            names: vec![dest],
            value,
        }
        .into()])
    }
}
