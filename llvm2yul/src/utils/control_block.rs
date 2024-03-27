use std::collections::{BTreeMap, BTreeSet};

use anyhow::{anyhow, Result};
use llvm_ir::{
    terminator::{CondBr, Ret},
    BasicBlock, Name, Operand, Terminator,
};
use yuler::{Assignment, Block, FunctionCall, Ident, Statement};

use super::{build_list_by_type, yul_ident_name};

fn _iter_control_block(
    block_names: &mut BTreeSet<Name>,
    compiled_blocks: &mut BTreeMap<Name, Block>,
    origin_blocks: &BTreeMap<Name, &BasicBlock>,
    name: Name,
) -> Result<()> {
    let basic_block = origin_blocks.get(&name).ok_or(anyhow!("Fatel error"))?;

    let basic_block_name = basic_block.name.clone();
    let term = basic_block.term.clone();

    match term {
        Terminator::Ret(t) => {
            let generated_block = compiled_blocks
                .get_mut(&name)
                .ok_or(anyhow!("Fatel error"))?;

            compile_return_value(&t, generated_block)?
        }
        Terminator::Br(t) => {
            _iter_control_block(
                block_names,
                compiled_blocks,
                origin_blocks,
                basic_block_name.clone(),
            )?;

            let generated_block = compiled_blocks.get(&name).ok_or(anyhow!("Fatel error"))?;

            let block = &compiled_blocks
                .get(&t.dest)
                .ok_or(anyhow!("Fatel error: Failed to get block"))?;

            let mut res = generated_block.clone();
            res.0.extend_from_slice(&block.0);

            compiled_blocks.insert(basic_block_name, res);
        }
        Terminator::CondBr(t) => {}
        Terminator::Switch(t) => {}
        Terminator::Unreachable(_t) => {
            let generated_block = compiled_blocks
                .get_mut(&name)
                .ok_or(anyhow!("Fatel error"))?;

            generated_block
                .0
                .push(FunctionCall::new(Ident::new("invalid")?).into());
        }
        _ => return Err(anyhow!("Unsupported terminator")),
    }

    block_names.insert(name);

    Ok(())
}

pub fn build_control_block_from_entry(
    compiled_blocks: &mut BTreeMap<Name, Block>,
    origin_blocks: &BTreeMap<Name, &BasicBlock>,
    entry: Name,
) -> Result<()> {
    log::info!("{entry}");
    let mut block_names = BTreeSet::new();

    _iter_control_block(&mut block_names, compiled_blocks, origin_blocks, entry)?;

    Ok(())
}

fn compile_return_value(inst: &Ret, block: &mut Block) -> Result<()> {
    // This part need insert value inst support.

    if let Some(operand) = &inst.return_operand {
        // Match return value

        match operand {
            Operand::LocalOperand { name, ty } => {
                let rname = yul_ident_name(name);

                let dists = build_list_by_type(None, ty, true)?;

                if dists.is_empty() {
                    block.0.push(Statement::Leave);
                } else {
                    for (i, dist) in dists.into_iter().enumerate() {
                        let from = Ident::new(format!("{rname}_{i}"))?;

                        block.0.push(
                            Assignment {
                                names: vec![dist],
                                value: from.into(),
                            }
                            .into(),
                        );
                    }
                }
                block.0.push(Statement::Leave)
            }
            Operand::ConstantOperand(_) => {
                todo!()
            }
            _ => return Err(anyhow!("Unsupported operand: {}", inst)),
        }
    } else {
        block.0.push(Statement::Leave)
    }

    Ok(())
}

fn compile_cond_br(block_name: Name, br: CondBr) -> Result<()> {
    // Check loop or if

    if br.true_dest == block_name || br.false_dest == block_name {
        // Loop
    } else {
        // False
    }

    Ok(())
}
