use anyhow::{anyhow, Result};
use llvm_ir::{instruction::ExtractValue, Name, Operand, Type};
use yuler::{Ident, Statement, VariableDeclare};

use crate::{error, utils};

pub struct ExtractValueCompiler<'a> {
    inst: &'a ExtractValue,
}

impl<'a> ExtractValueCompiler<'a> {
    pub fn new(inst: &'a ExtractValue) -> Self {
        Self { inst }
    }

    pub fn compile(&self) -> Result<Vec<Statement>> {
        let name = check_arg(&self.inst.aggregate)?;

        let mut res = Vec::new();

        let from = build_from_name(&name, &self.inst.indices)?;
        let to = utils::yul_ident_name(&self.inst.dest);
        let to = Ident::new(utils::tidy_name(&to))?;

        res.push(
            VariableDeclare {
                names: vec![to],
                value: from.into(),
            }
            .into(),
        );

        Ok(res)
    }
}

fn build_from_name(name: &Name, indices: &[u32]) -> Result<Ident> {
    let name = utils::yul_ident_name(name);
    let name = utils::tidy_name(&name);

    let indices: Vec<String> = indices.iter().map(|e| format!("{e}")).collect();

    let res = indices.join("_");

    Ident::new(format!("{name}_{res}"))
}

fn check_arg(ty: &Operand) -> Result<Name> {
    match ty {
        Operand::LocalOperand { name, ty } => match ty.as_ref() {
            Type::StructType {
                element_types: _,
                is_packed: _,
            }
            | Type::ArrayType {
                element_type: _,
                num_elements: _,
            } => Ok(name.clone()),
            _ => Err(anyhow!("{} extractvalue: {}", error::WRONG_ARG, ty)),
        },
        _ => Err(anyhow!("{} extractvalue: {}", error::WRONG_ARG, ty)),
    }
}
