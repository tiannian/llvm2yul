use std::{collections::BTreeMap, path::Path};

use anyhow::{anyhow, Result};
use llvm_ir::{types::Types, Function, Module};
use llvm_ir_analysis::ModuleAnalysis;
use yuler::{FunctionCall, FunctionDefinition, Ident, Object};

use crate::{utils, Config, FunctionCompiler};

#[derive(Debug, Default)]
pub struct Compiler {
    func_caches: BTreeMap<String, FunctionDefinition>,
    objects_caches: BTreeMap<String, Object>,

    config: Config,
}

impl Compiler {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    fn compile_function(
        &mut self,
        llvm_func: &Function,
        llvm_types: &Types,
    ) -> Result<(FunctionDefinition, Vec<String>)> {
        let mut func_compiler = FunctionCompiler::new(llvm_func, llvm_types, &self.config)?;

        func_compiler.compile_function_header()?;

        func_compiler.compile_function_body()
    }

    pub fn compile_object_from_bitcode(&mut self, path: &Path, entry: &str) -> Result<Object> {
        let module =
            Module::from_bc_path(path).map_err(|e| anyhow!("Failed to open module: {}", e))?;

        self.compile_object(&module, entry)
    }

    pub fn compile_object_from_textir(&mut self, path: &Path, entry: &str) -> Result<Object> {
        let module =
            Module::from_ir_path(path).map_err(|e| anyhow!("Failed to open module: {}", e))?;

        self.compile_object(&module, entry)
    }

    pub fn compile_object(&mut self, module: &Module, entry: &str) -> Result<Object> {
        if let Some(object) = self.objects_caches.get(entry) {
            Ok(object.clone())
        } else {
            let module_analysis = ModuleAnalysis::new(module);
            let call_graph = module_analysis.call_graph();

            let functions = utils::get_all_callees(&call_graph, entry);
            log::debug!("All callee for function {entry} is {:?}", functions);

            let name = Ident::new(entry)?;
            let mut object = Object::new(name.clone());

            for func in &module.functions {
                if let Some(function) = self.func_caches.get(&func.name) {
                    log::debug!("Hit Compiled function: {}", func.name);

                    object.code.0.push(function.clone().into());
                } else if functions.contains(&func.name) || func.name == entry {
                    log::debug!("Compile function: {}", func.name);

                    let (function, objects) = self.compile_function(func, &module.types)?;

                    object.code.0.push(function.into());

                    for o in objects {
                        object.objects.push(self.compile_object(module, &o)?);
                    }
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
}
