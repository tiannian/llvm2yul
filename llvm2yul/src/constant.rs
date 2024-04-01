use anyhow::{anyhow, Result};
use llvm_ir::{types::Types, Constant, Type};
use yuler::{Literal, Value};

use crate::{error, utils, Config, TypeFlatter};

pub struct ConstantFlatter<'a> {
    types: &'a Types,
    config: &'a Config,
}

impl<'a> ConstantFlatter<'a> {
    pub fn new(types: &'a Types, config: &'a Config) -> Self {
        Self { types, config }
    }

    pub fn flatten(&self, constant: &Constant) -> Result<Vec<Value>> {
        let mut res = Vec::new();

        self._flatten(&mut res, constant)?;

        Ok(res)
    }

    fn _flatten(&self, values: &mut Vec<Value>, constant: &Constant) -> Result<()> {
        match constant {
            Constant::Int { bits: _, value } => {
                values.push(Literal::int_number(format!("{value}"))?.into())
            }
            Constant::Null(_) => values.push(Literal::int_number("0")?.into()),
            Constant::AggregateZero(ty) => self._build_type_values(values, ty)?,
            Constant::Struct {
                name: _,
                values: consts,
                is_packed: _,
            } => {
                for c in consts {
                    self._flatten(values, c)?
                }
            }
            Constant::Array {
                element_type: _,
                elements,
            } => {
                for c in elements {
                    self._flatten(values, c)?
                }
            }
            Constant::Undef(ty) => self._build_type_values(values, ty)?,
            Constant::Poison(ty) => self._build_type_values(values, ty)?,
            Constant::PtrToInt(i) => self._flatten(values, &i.operand)?,
            Constant::IntToPtr(i) => self._flatten(values, &i.operand)?,
            Constant::GlobalReference { name, ty: _ } => {
                values.push(Literal::ascii(utils::yul_ident_name(name))?.into())
            }
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

    fn _build_type_values(&self, values: &mut Vec<Value>, ty: &Type) -> Result<()> {
        let flatter = TypeFlatter::new(self.types, self.config);

        let res = flatter.flatten_return_type(ty)?;

        for _ in 0..res.len() {
            values.push(Literal::int_number("0")?.into())
        }

        Ok(())
    }
}
