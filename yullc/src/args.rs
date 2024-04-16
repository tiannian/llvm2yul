use std::{
    fs::{self, File},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use llvm2yul::{Compiler, Config};
use yuler::Writer;

use crate::{LLVMIRType, Version};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub solidity_path: PathBuf,

    #[arg(long)]
    pub solidity_version: Option<Version>,

    #[arg(long)]
    pub basic_type_symbols: Vec<String>,

    #[arg(short = 'c', long)]
    pub output_contracts: Vec<String>,

    #[arg(short, long, default_value = "./target")]
    pub output_dir: PathBuf,

    #[arg(long, default_value = "bitcode")]
    pub llvm_ir_type: LLVMIRType,

    pub input: PathBuf,
}

impl Args {
    pub fn execute(self) -> Result<()> {
        let yul_output_dir = self.output_dir.join("yuls");
        fs::create_dir_all(&yul_output_dir)?;

        let config = Config::default().basic_types(&self.basic_type_symbols);

        let mut compiler = Compiler::new(config);

        for entry in self.output_contracts {
            let (name, entry) = split_entry(&entry)?;

            let object = if self.llvm_ir_type.is_bitcode() {
                compiler.compile_object_from_bitcode(&self.input, entry)?
            } else {
                compiler.compile_object_from_textir(&self.input, entry)?
            };

            let file = File::create(yul_output_dir.join(format!("{name}.yul")))?;
            let mut writer = Writer::new(file, "    ");
            object.write(&mut writer)?;
        }

        Ok(())
    }
}

fn split_entry(name: &str) -> Result<(&str, &str)> {
    let s: Vec<&str> = name.split('=').collect();

    match s.len() {
        1 => Ok((name, name)),
        2 => Ok((s[0], s[1])),
        _ => Err(anyhow!("Failed to split contract name")),
    }
}
