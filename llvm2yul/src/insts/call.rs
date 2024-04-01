use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Call, types::Types, Constant, Operand, Type};
use primitive_types::U256;
use yuler::{FunctionCall, Ident, Literal, Statement, Value, VariableDeclare};

use crate::{error, utils, Config, ConstantFlatter, TypeFlatter};

pub struct CallCompiler<'a> {
    call: &'a Call,
    types: &'a Types,
    config: &'a Config,
    pub(crate) object: Option<String>,
}

impl<'a> CallCompiler<'a> {
    pub fn new(call: &'a Call, types: &'a Types, config: &'a Config) -> Self {
        Self {
            call,
            config,
            types,

            object: None,
        }
    }

    pub fn compile_call(&mut self) -> Result<Statement> {
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

        self.object = build_object_entry(&func_call)?;

        if let Some(res) = convert_literal(&func_call)? {
            return Ok(VariableDeclare {
                names: rets,
                value: res.into(),
            }
            .into());
        }

        convert_builtin(&mut func_call)?;

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
            res.push(self.build_function_arg_from_llvm_directly(arg)?);
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
                    res.append(&mut self.build_function_arg_from_constant(constant)?);
                }
                _ => return Err(anyhow!("Unsupported function parameter for function call")),
            }
        }

        Ok(res)
    }

    fn build_function_arg_from_llvm_directly(&self, arg: &Operand) -> Result<Value> {
        match arg {
            Operand::LocalOperand { name, ty: _ } => {
                Ok(Ident::new(utils::yul_ident_name(name))?.into())
            }
            Operand::ConstantOperand(constant) => {
                let v = self.build_function_arg_from_constant(constant)?.remove(0);
                Ok(v)
            }
            _ => Err(anyhow!(
                "Unsupported function paramter for builtin function"
            )),
        }
    }

    fn build_function_arg_from_constant(&self, constant: &Constant) -> Result<Vec<Value>> {
        let flatter = ConstantFlatter::new(self.types, self.config);

        flatter.flatten(constant)
    }
}

fn build_object_entry(function_call: &FunctionCall) -> Result<Option<String>> {
    let s = function_call.name.0.as_str();

    if s == "__yul_datasize" || s == "__yul_dataoffset" {
        let arg = &function_call.args[0];

        let r = arg
            .as_literal()
            .expect("Wrong argument type")
            .as_ascii()
            .expect("Wrong argument type");

        Ok(Some(r.into()))
    } else {
        Ok(None)
    }
}

fn convert_literal(function_call: &FunctionCall) -> Result<Option<Literal>> {
    if function_call.name.0.as_str() == "__yul__ext_literal" {
        let r0 = function_call.args[0]
            .as_literal()
            .expect("Wrong argument type")
            .as_number()
            .expect("Wrong argument type");

        let r1 = function_call.args[1]
            .as_literal()
            .expect("Wrong argument type")
            .as_number()
            .expect("Wrong argument type");

        let r2 = function_call.args[2]
            .as_literal()
            .expect("Wrong argument type")
            .as_number()
            .expect("Wrong argument type");

        let r3 = function_call.args[3]
            .as_literal()
            .expect("Wrong argument type")
            .as_number()
            .expect("Wrong argument type");

        let res = U256([r3, r2, r1, r0]);

        let res = format!("{:#x}", res);

        Ok(Some(Literal::hex_number(res)?))
    } else {
        Ok(None)
    }
}

fn convert_builtin(function_call: &mut FunctionCall) -> Result<()> {
    let name = function_call.name.0.clone();

    match name.as_str() {
        "__yul__ext_literal" => {}
        // "__yul_datasize" => {
        //     // function_call.name.0 = "datasize".into();
        //     // return Ok(Some());
        // }
        // "__yul_dataoffset" => {
        //     function_call.name.0 = "dataoffset".into();
        //     return Ok(Some(name));
        // }
        _ => {
            if let Some(num_args) = utils::builtin_args_num(&name) {
                if function_call.args.len() != num_args {
                    return Err(anyhow!(
                        "{} call builtin arguments: {name}",
                        error::WRONG_ARG
                    ));
                }

                let res = name
                    .strip_prefix("__yul_")
                    .ok_or(anyhow!("{} call builtin prefix: {name}", error::WRONG_ARG))?;

                function_call.name.0 = res.into()
            }
        }
    }

    Ok(())
}
