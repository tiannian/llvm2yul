use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Call, Constant, Operand, Type};
use yuler::{Ident, Literal, Value};

use super::{build_list_by_type, yul_ident_name};

pub fn build_call_function_name(call: &Call) -> Result<Ident> {
    let operand = call
        .function
        .as_ref()
        .right()
        .ok_or(anyhow!("function must be operand"))?;

    let constant = operand
        .as_constant()
        .ok_or(anyhow!("called function must be constant"))?;

    if let Constant::GlobalReference { name, ty: _ } = constant {
        Ident::new(yul_ident_name(name))
    } else {
        Err(anyhow!("call global function only"))
    }
}

pub fn build_call_function_args(call: &Call) -> Result<Vec<Value>> {
    let mut res = Vec::new();

    for (arg, _) in &call.arguments {
        match arg {
            Operand::LocalOperand { name, ty } => {
                let args = build_list_by_type(Some(name), ty, false)?;

                for arg in args {
                    res.push(arg.into());
                }
            }
            Operand::ConstantOperand(c) => {
                let constant = c.as_ref();

                // log::warn!("{:?}", call);

                match constant {
                    Constant::Int { bits: _, value } => {
                        let arg = Literal::int_number(format!("{value}"))?.into();
                        res.push(arg)
                    }
                    Constant::GlobalReference { name: _, ty } => {
                        // TODO: use generic type handle method.
                        match ty.as_ref() {
                            Type::StructType {
                                element_types,
                                is_packed: _,
                            } => {
                                if !element_types.is_empty() {
                                    return Err(anyhow!("No supported struct"));
                                }
                                res.push(Literal::int_number("0")?.into())
                            }
                            _ => return Err(anyhow!("Unknown global reference type")),
                        }
                    }
                    Constant::Null(_) => res.push(Literal::int_number("0")?.into()),
                    _ => return Err(anyhow!("Unsupport value")),
                }
            }
            _ => return Err(anyhow!("Unsupport argument")),
        }
    }

    Ok(res)
}
