use anyhow::{anyhow, Result};
use llvm2yul::{Compiler, Config};
use llvm_ir::Module;
use yuler::Writer;

fn main() -> Result<()> {
    env_logger::init();

    let config = Config::default("Contract".into());
    let mut compiler = Compiler::new(config)?;

    let module = Module::from_ir_path("llvm2yul/lls/store.ll").map_err(|e| anyhow!("{e}"))?;
    let object = compiler.compile(module)?;

    // println!("{:#?}", object);
    let res = Vec::new();
    let mut writer = Writer::new(res, "    ");
    object.write(&mut writer)?;

    println!("{}", String::from_utf8(writer.w)?);

    Ok(())
}
