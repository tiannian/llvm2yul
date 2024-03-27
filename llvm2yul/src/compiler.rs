use std::collections::{BTreeMap, HashMap};

use anyhow::{anyhow, Result};
use llvm_ir::{
    function::Parameter, instruction::Call, BasicBlock, Function, Instruction, Module, Type,
};
use llvm_ir_analysis::{FunctionAnalysis, ModuleAnalysis};
use yuler::{Block, FunctionCall, FunctionDefinition, Ident, Object, Statement};

use crate::{
    is_builtin,
    utils::{self, build_list_by_type},
    Config,
};

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

        let mut function_defination = FunctionDefinition::new(name);

        // build parameters
        for p in &func.parameters {
            let mut p = self.compile_function_parameter(p)?;
            function_defination.args.append(&mut p);
        }

        // build return value
        function_defination.rets = self.compile_function_ret(&func.return_type)?;

        // build block
        self.compile_function_body(&mut function_defination.block, func)?;

        self.func_caches
            .insert(func.name.clone(), function_defination.clone());

        Ok(function_defination)
    }

    pub fn compile_function_parameter(&self, parameter: &Parameter) -> Result<Vec<Ident>> {
        log::info!("Parameter: {:?}", parameter);

        utils::build_list_by_type(Some(&parameter.name), &parameter.ty, false)
    }

    pub fn compile_function_ret(&self, ty: &Type) -> Result<Vec<Ident>> {
        log::debug!("Return type: {:?}", ty);

        build_list_by_type(None, ty, true)
    }

    pub fn compile_function_body(&self, block: &mut Block, func: &Function) -> Result<()> {
        // let function_analysis = FunctionAnalysis::new(func);
        // let control_flow = function_analysis.control_flow_graph();

        let mut blocks = BTreeMap::new();

        for block in &func.basic_blocks {
            let generated_block = self.compile_basic_block(block)?;
            blocks.insert(block.name.clone(), generated_block);
        }

        Ok(())
    }

    pub fn compile_basic_block(&self, bb: &BasicBlock) -> Result<Block> {
        let mut block = Block::default();

        for inst in &bb.instrs {
            block.0.append(&mut self.compile_inst(inst)?);
        }

        Ok(block)
    }

    pub fn compile_inst(&self, inst: &Instruction) -> Result<Vec<Statement>> {
        let calls = Vec::new();

        match inst {
            // Instruction::Add(i) => {}
            // Instruction::Sub(i) => {}
            Instruction::Call(i) => {
                log::debug!("{:#?}", i)
            }
            _ => return Err(anyhow!("Unspported instruction: {:?}", inst)),
        }

        Ok(calls)
    }

    // pub fn compile_call_inst(&self, call: Call) -> Result<FunctionCall> {
    //     // let function_call = FunctionCall::n
    // }
}
