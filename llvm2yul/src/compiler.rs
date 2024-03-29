use std::collections::BTreeMap;

use anyhow::Result;
use llvm_ir::{Function, Module};
use llvm_ir_analysis::ModuleAnalysis;
use yuler::{FunctionCall, FunctionDefinition, Ident, Object};

use crate::utils;

#[derive(Debug, Default)]
pub struct Compiler {
    pub object_funcs: BTreeMap<String, Vec<String>>,
    pub func_caches: BTreeMap<String, FunctionDefinition>,
}

impl Compiler {
    pub fn compile_function(&self, llvm_func: &Function) -> Result<FunctionDefinition> {
        let mut func = FunctionDefinition::new(Ident::new(&llvm_func.name)?);

        log::debug!("Compile function {}", llvm_func.name);

        // Compile function header
        // Compile function parameters
        for paramter in &llvm_func.parameters {
            let mut args = utils::build_list_by_type(Some(&paramter.name), &paramter.ty, false)?;

            func.args.append(&mut args)
        }

        // Compile function return type
        let mut rets = utils::build_list_by_type(None, &llvm_func.return_type, true)?;
        func.rets.append(&mut rets);

        // Compile block

        // Compile termiantor

        Ok(func)
    }

    pub fn compile_object(&mut self, module: &Module, entry: &str) -> Result<Object> {
        let module_analysis = ModuleAnalysis::new(module);
        let call_graph = module_analysis.call_graph();

        let functions = utils::get_all_callees(&call_graph, &entry);
        log::debug!("All callee for function {entry} is {:?}", functions);

        let name = Ident::new(entry)?;
        let mut object = Object::new(name.clone());

        for func in &module.functions {
            if functions.contains(&func.name) || func.name == entry {
                log::info!("Compile function: {}", func.name);
                let function = self.compile_function(func)?;

                object.code.0.push(function.into());
            }
        }

        object.code.0.push(
            FunctionCall {
                name,
                args: Vec::new(),
            }
            .into(),
        );

        Ok(object)
    }
}
