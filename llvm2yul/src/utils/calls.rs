use anyhow::{anyhow, Result};
use llvm_ir::{instruction::Call, Constant};
use yuler::Ident;

use super::yul_ident_name;

pub fn build_call_function_name(call: &Call) -> Result<Ident> {
    let operand = call
        .function
        .as_ref()
        .right()
        .ok_or(anyhow!("function must be operand"))?;

    let constant = operand
        .as_constant()
        .ok_or(anyhow!("called function must be constant"))?;

    if let Constant::GlobalReference { name, ty: _ } = constant {
        Ident::new(yul_ident_name(name))
    } else {
        Err(anyhow!("call global function only"))
    }
}
