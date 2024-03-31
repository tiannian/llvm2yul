use std::collections::BTreeMap;

use anyhow::Result;
use llvm_ir::{types::Types, Function, Module};
use llvm_ir_analysis::ModuleAnalysis;
use yuler::{FunctionCall, FunctionDefinition, Ident, Object};

use crate::{utils, FunctionCompiler};

#[derive(Debug, Default)]
pub struct Compiler {
    func_caches: BTreeMap<String, FunctionDefinition>,
}

impl Compiler {
    pub fn compile_function(
        &mut self,
        llvm_func: &Function,
        llvm_types: &Types,
    ) -> Result<FunctionDefinition> {
        let mut func_compiler = FunctionCompiler::new(llvm_func, llvm_types)?;

        func_compiler.compile_function_header()?;

        func_compiler.compile_function_body()
    }

    pub fn compile_object(&mut self, module: &Module, entry: &str) -> Result<Object> {
        let module_analysis = ModuleAnalysis::new(module);
        let call_graph = module_analysis.call_graph();

        let functions = utils::get_all_callees(&call_graph, entry);
        log::debug!("All callee for function {entry} is {:?}", functions);

        let name = Ident::new(entry)?;
        let mut object = Object::new(name.clone());

        for func in &module.functions {
            if functions.contains(&func.name) || func.name == entry {
                log::debug!("Compile function: {}", func.name);
                let function = self.compile_function(func, &module.types)?;

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
