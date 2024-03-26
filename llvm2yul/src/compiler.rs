use std::collections::HashMap;

use anyhow::Result;
use llvm_ir::{Function, Module};
use llvm_ir_analysis::ModuleAnalysis;
use yuler::{FunctionCall, FunctionDefinition, Ident, Object};

use crate::{is_builtin, utils, Config};

pub struct Compiler {
    config: Config,

    func_caches: HashMap<String, FunctionDefinition>,
}

impl Compiler {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            func_caches: HashMap::default(),
        })
    }

    pub fn compile(&mut self, module: Module) -> Result<Object> {
        // build entry
        let object = self.compile_object(self.config.entry.clone(), &module)?;

        // build other function

        Ok(object)
    }

    pub fn compile_object(&mut self, func_name: String, module: &Module) -> Result<Object> {
        let module_analysis = ModuleAnalysis::new(module);
        let call_graph = module_analysis.call_graph();

        let functions = utils::get_all_callees(&call_graph, &func_name);
        log::debug!("All callee for function {func_name} is {:?}", functions);

        let name = Ident::new(&func_name)?;
        let mut object = Object::new(name.clone());

        for func in &module.functions {
            if functions.contains(&func.name) && !is_builtin(&func.name) || func.name == func_name {
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

    pub fn compile_function(&mut self, func: &Function) -> Result<FunctionDefinition> {
        if let Some(f) = self.func_caches.get(&func.name) {
            log::debug!("Hit function {} cache", func.name);
            return Ok(f.clone());
        }

        log::debug!("Missing funcion {} cache, build it", func.name);

        let name = Ident::new(&func.name)?;

        let function_defination = FunctionDefinition::new(name);

        self.func_caches
            .insert(func.name.clone(), function_defination.clone());

        Ok(function_defination)
    }

    pub fn compile_function_header(&mut self) -> Result<()> {
        Ok(())
    }
}

// pub fn compile_function(func: &Function, idx: usize) -> Result<FunctionDefinition> {
//     let mut name_incr = 0;
//
//     let name = Ident::new(format!("f{}", idx))?;
//     let mut args = Vec::new();
//     let mut symbol_map = HashMap::new();
//
//     macro_rules! incr_and_build_name {
//         ($t:expr, $n:expr) => {{
//             let name = Ident::new(format!("v{name_incr}"))?;
//             symbol_map.insert($n, name.clone());
//             $t.push(name);
//             name_incr += 1;
//         }};
//     }
//
//     // args
//     for arg in &func.parameters {
//         match arg.ty.as_ref() {
//             Type::IntegerType { bits: _ } => incr_and_build_name!(args, arg.name.clone()),
//             Type::PointerType { addr_space: _ } => incr_and_build_name!(args, arg.name.clone()),
//             Type::ArrayType {
//                 element_type: _,
//                 num_elements: _,
//             } => {
//                 todo!()
//             }
//             Type::StructType {
//                 element_types: _,
//                 is_packed: _,
//             } => {
//                 todo!()
//             }
//             Type::NamedStructType { name: _ } => {
//                 todo!()
//             }
//             _ => return Err(anyhow!("Unsupported type")),
//         }
//     }
//
//     // return value
//     let mut rets = Vec::new();
//     let ret_name = Name::Name(Box::new("__return_value".into()));
//
//     match func.return_type.as_ref() {
//         Type::VoidType => {}
//         Type::IntegerType { bits: _ } => incr_and_build_name!(rets, ret_name),
//         Type::PointerType { addr_space: _ } => incr_and_build_name!(rets, ret_name),
//         Type::ArrayType {
//             element_type: _,
//             num_elements: _,
//         } => {
//             todo!()
//         }
//         Type::NamedStructType { name: _ } => {
//             todo!()
//         }
//         Type::StructType {
//             element_types: _,
//             is_packed: _,
//         } => {
//             todo!()
//         }
//         _ => {}
//     }
//
//     // block
//     let mut block = Block::default();
//     let funcion_analysis = FunctionAnalysis::new(func);
//     let control_flow = funcion_analysis.control_flow_graph();
//
//     for block in &func.basic_blocks {}
//
//     let function = FunctionDefinition {
//         name,
//         args,
//         rets,
//         block,
//     };
//
//     Ok(function)
// }
