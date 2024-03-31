use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use llvm_ir::{types::Types, Function};
use llvm_ir_analysis::FunctionAnalysis;
use yuler::{FunctionDefinition, Ident};

use crate::{utils, BlockCompiler, Config, TypeFlatter};

pub struct FunctionCompiler<'a> {
    llvm_func: &'a Function,
    llvm_types: &'a Types,
    config: &'a Config,
    func: FunctionDefinition,
}

impl<'a> FunctionCompiler<'a> {
    pub fn new(llvm_func: &'a Function, llvm_types: &'a Types, config: &'a Config) -> Result<Self> {
        let func = FunctionDefinition::new(Ident::new(&llvm_func.name)?);

        Ok(Self {
            llvm_func,
            func,
            config,
            llvm_types,
        })
    }

    pub fn compile_function_header(&mut self) -> Result<()> {
        // Compile function return type
        let flatter = TypeFlatter::new(self.llvm_types, self.config);
        let mut rets = flatter.flatten_return_type(&self.llvm_func.return_type)?;
        self.func.rets.append(&mut rets);

        // Compile function header
        // Compile function parameters
        for paramter in &self.llvm_func.parameters {
            let mut args = flatter.flatten_parameter(&paramter.name, &paramter.ty)?;
            self.func.args.append(&mut args)
        }

        Ok(())
    }

    pub fn compile_function_body(self) -> Result<FunctionDefinition> {
        let mut blocks = BTreeMap::new();
        // Compile all blocks
        {
            for bb in &self.llvm_func.basic_blocks {
                log::debug!("Compile block: {}", bb.name);

                let block_compiler = BlockCompiler::new(bb, self.llvm_types, self.config);

                let block = block_compiler.compile()?;

                blocks.insert(utils::yul_ident_name(&bb.name), block);
            }
        }

        // Compile termiantor
        let func = {
            let function_analysis = FunctionAnalysis::new(self.llvm_func);
            let control_flow = function_analysis.control_flow_graph();

            let entry = control_flow.entry();
            let entry = utils::yul_ident_name(entry);

            let mut block = blocks
                .remove(&entry)
                .ok_or(anyhow!("Fatel: Failed to get basic block: {entry}"))?;

            let mut func = self.func;

            func.block.0.append(&mut block);

            func
        };

        Ok(func)
    }
}
