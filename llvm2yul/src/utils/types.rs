use anyhow::{anyhow, Result};
use llvm_ir::{Name, Type};
use yuler::Ident;

use super::{tidy_name, yul_ident_name};

fn _iter_type(
    tokens: &mut Vec<Ident>,
    name: Option<&Name>,
    ty: &Type,
    void_generated: bool,
) -> Result<()> {
    let sname = if let Some(name) = name {
        let iname = yul_ident_name(name);
        tidy_name(&iname)
    } else {
        "__yn_return".into()
    };

    let sname = if tokens.is_empty() {
        Ident::new(sname)?
    } else {
        Ident::new(format!("{}_{}", sname, tokens.len()))?
    };

    match ty {
        Type::VoidType => {
            if !void_generated {
                return Err(anyhow!("Unspported Void Type"));
            }
        }
        Type::IntegerType { bits } => {
            if bits > &256 {
                return Err(anyhow!("Unsupported length: {bits}"));
            }
            tokens.push(sname);
        }
        Type::PointerType { addr_space: _ } => tokens.push(sname),
        Type::StructType {
            element_types,
            is_packed: _,
        } => {
            for e in element_types {
                let e = e.as_ref();

                _iter_type(tokens, name, e, void_generated)?;
            }
        }
        _ => return Err(anyhow!("Unspported Type: {}", ty)),
    }

    Ok(())
}

pub fn flatten_struct_type(
    name: Option<&Name>,
    ty: &Type,
    void_generated: bool,
) -> Result<Vec<Ident>> {
    let mut res = Vec::new();

    _iter_type(&mut res, name, ty, void_generated)?;

    Ok(res)
}
