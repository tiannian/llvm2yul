use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Call, types::Types, Constant, Operand, Type};
use yuler::{FunctionCall, Ident, Literal, Statement, Value, VariableDeclare};

use crate::{utils, Config, TypeFlatter};

pub struct CallCompiler<'a> {
    call: &'a Call,
    types: &'a Types,
    config: &'a Config,
}

impl<'a> CallCompiler<'a> {
    pub fn new(call: &'a Call, types: &'a Types, config: &'a Config) -> Self {
        Self {
            call,
            config,
            types,
        }
    }

    pub fn compile_call(&self) -> Result<Statement> {
        let (call_name, rets) = self.build_call_function_name_and_rets()?;

        // build function call name
        let mut func_call = FunctionCall::new(Ident::new(&call_name)?);

        // build function call paramters
        func_call.args = if utils::is_builtin(&call_name) {
            // builtin functions don't have any struct parameters. It just have plain type.
            self.build_call_function_parameters_directly()?
        } else {
            // For common function call may have struct, we can build function call parameter based
            // on map of args
            self.build_call_function_parameters()?
        };

        Ok(if rets.is_empty() {
            func_call.into()
        } else {
            VariableDeclare {
                names: rets,
                value: func_call.into(),
            }
            .into()
        })
    }

    fn build_call_function_name_and_rets(&self) -> Result<(String, Vec<Ident>)> {
        let operand = self
            .call
            .function
            .as_ref()
            .right()
            .ok_or(anyhow!("function must be operand"))?;

        let constant = operand
            .as_constant()
            .ok_or(anyhow!("called function must be constant"))?;

        if let Constant::GlobalReference { name, ty } = constant {
            let name = utils::yul_ident_name(name);

            if let Some(dest) = &self.call.dest {
                if let Type::FuncType {
                    result_type,
                    param_types: _,
                    is_var_arg: _,
                } = ty.as_ref()
                {
                    let flatter = TypeFlatter::new(self.types, self.config);
                    let names = flatter.flatten_parameter(dest, result_type)?;
                    Ok((name, names))
                } else {
                    Err(anyhow!("must call function"))
                }
            } else {
                Ok((name, vec![]))
            }
        } else {
            Err(anyhow!("call global function only"))
        }
    }

    /// Build builtin function call without type flatten
    pub(crate) fn build_call_function_parameters_directly(&self) -> Result<Vec<Value>> {
        let mut res = Vec::new();

        for (arg, _) in &self.call.arguments {
            res.push(build_function_arg_from_llvm_directly(arg)?);
        }

        Ok(res)
    }

    pub fn build_call_function_parameters(&self) -> Result<Vec<Value>> {
        let mut res = Vec::new();

        for (parameter, _) in &self.call.arguments {
            match parameter {
                Operand::LocalOperand { name, ty } => {
                    let flatter = TypeFlatter::new(self.types, self.config);
                    let names = flatter.flatten_parameter(name, ty)?;

                    for n in names {
                        res.push(n.into())
                    }
                }
                Operand::ConstantOperand(constant) => {
                    res.push(build_function_arg_from_constant(constant)?);
                }
                _ => return Err(anyhow!("Unsupported function parameter for function call")),
            }
        }

        Ok(res)
    }
}

fn build_function_arg_from_llvm_directly(arg: &Operand) -> Result<Value> {
    match arg {
        Operand::LocalOperand { name, ty: _ } => {
            Ok(Ident::new(utils::yul_ident_name(name))?.into())
        }
        Operand::ConstantOperand(constant) => build_function_arg_from_constant(constant),
        _ => Err(anyhow!(
            "Unsupported function paramter for builtin function"
        )),
    }
}

fn build_function_arg_from_constant(constant: &Constant) -> Result<Value> {
    let arg = match constant {
        Constant::Int { bits: _, value } => Literal::int_number(format!("{value}"))?.into(),
        Constant::GlobalReference { name, ty } => match ty.as_ref() {
            Type::StructType {
                element_types,
                is_packed: _,
            } => {
                if !element_types.is_empty() {
                    return Err(anyhow!("No supported struct"));
                }
                Literal::int_number("0")?.into()
            }
            Type::FuncType {
                result_type: _,
                param_types: _,
                is_var_arg: _,
            } => Literal::ascii(utils::yul_ident_name(name))?.into(),
            _ => return Err(anyhow!("Unknown global reference type: {constant:?}")),
        },
        Constant::Null(_) => Literal::int_number("0")?.into(),
        Constant::IntToPtr(i) => build_function_arg_from_constant(&i.operand)?,
        _ => return Err(anyhow!("Unsupported constant: {constant:?}")),
    };

    Ok(arg)
}
