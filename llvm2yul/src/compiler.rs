use std::collections::BTreeMap;

use anyhow::Result;
use llvm_ir::{Function, Module};
use llvm_ir_analysis::ModuleAnalysis;
use yuler::{FunctionCall, FunctionDefinition, Ident, Object};

use crate::{utils, ExtendedArgsMap, FunctionCompiler};

#[derive(Debug, Default)]
pub struct Compiler {
    extended_args: ExtendedArgsMap,
    func_caches: BTreeMap<String, FunctionDefinition>,
}

impl Compiler {
    pub fn compile_function(&mut self, llvm_func: &Function) -> Result<FunctionDefinition> {
        let mut func_compiler = FunctionCompiler::new(llvm_func)?;

        let extended_args = func_compiler.compile_function_header()?;
        self.extended_args
            .insert(llvm_func.name.clone(), extended_args);

        func_compiler.set_extended_args(&self.extended_args);

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
