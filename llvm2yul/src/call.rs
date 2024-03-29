use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Call, Constant, Operand, Type};
use yuler::{FunctionCall, Ident, Literal, Statement, Value};

use crate::{utils, ExtendedArgsMap};

pub struct CallCompiler<'a> {
    call: &'a Call,
    extended_args: &'a ExtendedArgsMap,
}

impl<'a> CallCompiler<'a> {
    pub fn new(call: &'a Call, extended_args: &'a ExtendedArgsMap) -> Self {
        Self {
            call,
            extended_args,
        }
    }

    pub fn compile_call(&self) -> Result<Statement> {
        let call_name = self.build_call_function_name()?;

        // build function call name
        let mut func_call = FunctionCall::new(Ident::new(&call_name)?);

        // build function call paramters
        func_call.args = if utils::is_builtin(&call_name) {
            // builtin functions don't have any struct parameters. It just have plain type.
            self.build_call_function_parameters_directly()?
        } else {
            // For common function call may have struct, we can build function call parameter from
            self.build_call_function_parameters(&call_name)?
        };

        let res = if let Some(dest) = &self.call.dest {
            func_call.into()
            // Assignment{names: vec![]}.into()
        } else {
            func_call.into()
        };

        Ok(res)
    }

    pub fn build_call_function_name(&self) -> Result<String> {
        let operand = self
            .call
            .function
            .as_ref()
            .right()
            .ok_or(anyhow!("function must be operand"))?;

        let constant = operand
            .as_constant()
            .ok_or(anyhow!("called function must be constant"))?;

        if let Constant::GlobalReference { name, ty: _ } = constant {
            let name = utils::yul_ident_name(name);
            if name == "_ZN11patine_core5alloc8allocate17h72f78d4cb2bfc710E" {
                log::debug!("Build function call to {name}, {:#?}", self.call);
            }
            Ok(name)
        } else {
            Err(anyhow!("call global function only"))
        }
    }

    pub fn build_call_function_parameters_directly(&self) -> Result<Vec<Value>> {
        let mut res = Vec::new();

        for (arg, _) in &self.call.arguments {
            res.push(build_function_arg_from_llvm_directly(arg)?);
        }

        Ok(res)
    }

    pub fn build_call_function_parameters(&self, call_name: &str) -> Result<Vec<Value>> {
        let mut res = Vec::new();

        for (parameter, _) in &self.call.arguments {
            if let Some(r) = self.extended_args.get(call_name) {
                match parameter {
                    Operand::LocalOperand { name, ty: _ } => {
                        // TODO: Fix need. Use position
                        let arg_name = utils::yul_ident_name(name);

                        if let Some(args) = r.get(&Some(arg_name.clone())) {
                            for arg in args {
                                res.push(arg.clone().into());
                            }
                        } else {
                            res.push(Ident::new(arg_name)?.into());
                        }
                    }
                    Operand::ConstantOperand(constant) => {
                        res.push(build_function_arg_from_constant(constant)?);
                    }
                    _ => return Err(anyhow!("Unsupported function parameter for function call")),
                }
            } else {
                res.push(build_function_arg_from_llvm_directly(parameter)?);
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
