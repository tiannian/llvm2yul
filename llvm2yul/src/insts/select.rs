use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Select, Constant, Operand, Type};
use yuler::{Assignment, Ident, If, Literal, Statement, Value, VariableDeclare};

use crate::utils::{self, yul_ident_name};

pub struct SelectCompiler<'a> {
    select: &'a Select,
}

impl<'a> SelectCompiler<'a> {
    pub fn new(select: &'a Select) -> Self {
        Self { select }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        let mut res = Vec::new();

        let dest = Ident::new(utils::yul_ident_name(&self.select.dest))?;

        let name = if let Operand::LocalOperand { name, ty } = &self.select.condition {
            if let Type::IntegerType { bits: 1 } = ty.as_ref() {
                Ident::new(utils::yul_ident_name(name))?
            } else {
                return Err(anyhow!(
                    "Fatal Error, condition of select must be i1, {}",
                    self.select.condition
                ));
            }
        } else {
            return Err(anyhow!(
                "Fatal Error, condition of select must be i1, {}",
                self.select.condition
            ));
        };

        res.push(
            VariableDeclare {
                names: vec![dest.clone()],
                value: build_operand_value_for_select(&self.select.false_value)?,
            }
            .into(),
        );

        res.push(
            If {
                cond: name.into(),
                block: vec![Assignment {
                    names: vec![dest],
                    value: build_operand_value_for_select(&self.select.true_value)?,
                }
                .into()]
                .into(),
            }
            .into(),
        );

        Ok(res)
    }
}

fn build_operand_value_for_select(operand: &Operand) -> Result<Value> {
    match operand {
        // TODO: flatten operand
        Operand::LocalOperand { name, ty } => match ty.as_ref() {
            Type::IntegerType { bits: _ } => {
                let name = Ident::new(yul_ident_name(name))?;

                Ok(name.into())
            }
            Type::PointerType { addr_space: _ } => {
                let name = Ident::new(yul_ident_name(name))?;

                Ok(name.into())
            }
            _ => Err(anyhow!("Unsupported local type: {operand}")),
        },
        // TODO: flatten constant
        Operand::ConstantOperand(constant) => match constant.as_ref() {
            Constant::Int { bits: _, value } => Ok(Literal::int_number(format!("{value}"))?.into()),
            _ => Err(anyhow!("Unsupported constant type: {constant}")),
        },
        _ => Err(anyhow!("Unsupported operand for select: {operand}")),
    }
}
