use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use llvm_ir::{Function, Module, Name, Type};
use llvm_ir_analysis::{FunctionAnalysis, ModuleAnalysis};
use yuler::{Block, FunctionDefinition, Ident, Object};

use crate::{is_builtin, Config};

pub struct Compiler {
    module: Module,

    config: Config,
}

impl Compiler {
    pub fn new(module: Module, config: Config) -> Result<Self> {
        Ok(Self { module, config })
    }

    pub fn compile(&mut self) -> Result<Object> {
        // Check function declare?

        // Build deployed contract
        let mut deployed = Object::new(Ident::new(format!(
            "{}_deployed",
            self.config.contract_name
        ))?);

        log::debug!("Build Deployed Contract");
        self.compile_contract(self.config.deployed_entry.clone(), &mut deployed)?;

        // Build deploy contract
        let mut constructor = Object::new(Ident::new(&self.config.contract_name)?);

        log::debug!("Build Constructor Contract");
        self.compile_contract(self.config.entry.clone(), &mut constructor)?;

        constructor.objects.push(deployed);

        Ok(constructor)
    }

    pub fn compile_contract(&mut self, entry: String, object: &mut Object) -> Result<()> {
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

        for (i, func) in self.module.functions.iter().enumerate() {
            if functions.contains(&func.name) && !is_builtin(&func.name) {
                log::debug!("Compile function");

                let function = compile_function(func, i)?;

                object.code.0.push(function.into());
            }
        }

        Ok(())
    }
}

pub fn compile_function(func: &Function, idx: usize) -> Result<FunctionDefinition> {
    let mut name_incr = 0;

    let name = Ident::new(format!("f{}", idx))?;
    let mut args = Vec::new();
    let mut symbol_map = HashMap::new();

    macro_rules! incr_and_build_name {
        ($t:expr, $n:expr) => {{
            let name = Ident::new(format!("v{name_incr}"))?;
            symbol_map.insert($n, name.clone());
            $t.push(name);
            name_incr += 1;
        }};
    }

    // args
    for arg in &func.parameters {
        match arg.ty.as_ref() {
            Type::IntegerType { bits: _ } => incr_and_build_name!(args, arg.name.clone()),
            Type::PointerType { addr_space: _ } => incr_and_build_name!(args, arg.name.clone()),
            Type::ArrayType {
                element_type: _,
                num_elements: _,
            } => {
                todo!()
            }
            Type::StructType {
                element_types: _,
                is_packed: _,
            } => {
                todo!()
            }
            Type::NamedStructType { name: _ } => {
                todo!()
            }
            _ => return Err(anyhow!("Unsupported type")),
        }
    }

    // return value
    let mut rets = Vec::new();
    let ret_name = Name::Name(Box::new("__return_value".into()));

    match func.return_type.as_ref() {
        Type::VoidType => {}
        Type::IntegerType { bits: _ } => incr_and_build_name!(rets, ret_name),
        Type::PointerType { addr_space: _ } => incr_and_build_name!(rets, ret_name),
        Type::ArrayType {
            element_type: _,
            num_elements: _,
        } => {
            todo!()
        }
        Type::NamedStructType { name: _ } => {
            todo!()
        }
        Type::StructType {
            element_types: _,
            is_packed: _,
        } => {
            todo!()
        }
        _ => {}
    }

    // block
    let mut block = Block::default();
    let funcion_analysis = FunctionAnalysis::new(func);
    let control_flow = funcion_analysis.control_flow_graph();

    for block in &func.basic_blocks {}

    let function = FunctionDefinition {
        name,
        args,
        rets,
        block,
    };

    Ok(function)
}
