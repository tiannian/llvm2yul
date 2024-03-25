use std::collections::HashSet;

use anyhow::Result;
use llvm_ir::Module;
use llvm_ir_analysis::ModuleAnalysis;
use yuler::{Ident, Object};

use crate::Config;

pub struct Compiler {
    module: Module,
    object: Object,

    config: Config,
}

impl Compiler {
    pub fn new(module: Module, config: Config) -> Result<Self> {
        let name = Ident::new(&config.contract_name)?;

        Ok(Self {
            module,
            object: Object::new(name),

            config,
        })
    }

    pub fn compile(&mut self) -> Result<()> {
        // Check function declare?
        // Parse function call graph.

        // Build deploy contract
        log::debug!("Build Constructor Contract");
        self.compile_contract(self.config.entry.clone())?;

        // Build deployed contract
        log::debug!("Build Deployed Contract");
        self.compile_contract(self.config.deployed_entry.clone())?;
        Ok(())
    }

    pub fn compile_contract(&mut self, entry: String) -> Result<()> {
        let mut functions = HashSet::new();

        functions.insert(entry.clone());

        let module_analysis = ModuleAnalysis::new(&self.module);
        let call_graph = module_analysis.call_graph();

        fn _iter_functions(
            functions: &mut HashSet<String>,
            call_graph: &llvm_ir_analysis::CallGraph<'_>,
            entry: &str,
        ) {
            let it = call_graph.callees(entry);
            for name in it {
                if !functions.contains(name) {
                    functions.insert(name.into());

                    _iter_functions(functions, call_graph, entry)
                }
            }
        }

        _iter_functions(&mut functions, &call_graph, &entry);

        drop(call_graph);

        for name in &functions {
            self.compile_function(name)?;
        }

        Ok(())
    }

    pub fn compile_function(&mut self, _name: &str) -> Result<()> {
        Ok(())
    }

    pub fn build(self) -> Object {
        self.object
    }
}
