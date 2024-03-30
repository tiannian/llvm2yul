use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use llvm_ir::Function;
use llvm_ir_analysis::FunctionAnalysis;
use yuler::{FunctionDefinition, Ident};

use crate::{utils, BlockCompiler};

pub(crate) type ExtendedArgsMap = BTreeMap<String, Vec<usize>>;

#[derive(Debug)]
pub struct FunctionCompiler<'a> {
    extended_args: Option<&'a ExtendedArgsMap>,
    llvm_func: &'a Function,
    func: FunctionDefinition,
}

impl<'a> FunctionCompiler<'a> {
    pub fn new(llvm_func: &'a Function) -> Result<Self> {
        let func = FunctionDefinition::new(Ident::new(&llvm_func.name)?);

        Ok(Self {
            extended_args: None,
            llvm_func,
            func,
        })
    }

    pub fn set_extended_args(&mut self, extended_args: &'a ExtendedArgsMap) {
        self.extended_args = Some(extended_args)
    }

    pub fn compile_function_header(&mut self) -> Result<Vec<usize>> {
        let mut extended_args = Vec::new();

        // Compile function return type
        let mut rets = utils::build_list_by_type(None, &self.llvm_func.return_type, true)?;
        extended_args.push(rets.len());
        self.func.rets.append(&mut rets);

        // Compile function header
        // Compile function parameters
        for paramter in &self.llvm_func.parameters {
            let mut args = utils::build_list_by_type(Some(&paramter.name), &paramter.ty, false)?;
            extended_args.push(args.len());
            self.func.args.append(&mut args)
        }

        Ok(extended_args)
    }

    pub fn compile_function_body(self) -> Result<FunctionDefinition> {
        let extended_args = &self
            .extended_args
            .ok_or(anyhow!("Fatel: No extended args map set"))?;

        let mut blocks = BTreeMap::new();
        // Compile all blocks
        {
            for bb in &self.llvm_func.basic_blocks {
                log::debug!("Compile block: {}", bb.name);

                let block_compiler = BlockCompiler::new(bb, extended_args);

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
