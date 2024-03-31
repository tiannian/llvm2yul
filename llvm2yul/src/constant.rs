use anyhow::{anyhow, Result};
use llvm_ir::Constant;
use yuler::{Literal, Value};

use crate::error;

#[derive(Default)]
pub struct ConstantFlatter {}

impl ConstantFlatter {
    pub fn flatten(constant: &Constant) -> Vec<Value> {
        vec![]
    }
}

fn _flatten(values: &mut Vec<Value>, constant: &Constant) -> Result<()> {
    match constant {
        Constant::Int { bits: _, value } => {
            values.push(Literal::int_number(format!("{value}"))?.into())
        }
        Constant::Null(_) => values.push(Literal::int_number("0")?.into()),
        Constant::AggregateZero(ty) => {
            // TODO: flatten zero here.
        }
        Constant::Struct {
            name: _,
            values,
            is_packed: _,
        } => {
            // TODO: flatten zero here.
        }
        Constant::Array {
            element_type: _,
            elements,
        } => {
            // TODO: flatten zero here.
        }
        Constant::Undef(_) => values.push(Literal::int_number("0")?.into()),
        Constant::Poison(_) => values.push(Literal::int_number("0")?.into()),
        _ => {
            return Err(anyhow!(
                "{} constant flatten: {}",
                error::UNSUPPERTED_OPERAND,
                constant
            ))
        }
    }

    Ok(())
}
