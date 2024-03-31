use anyhow::{anyhow, Result};
use llvm_ir::{
    types::{NamedStructDef, Types},
    Name, Type,
};
use yuler::Ident;

use crate::utils;

pub struct TypeFlatter<'a> {
    types: &'a Types,
}

impl<'a> TypeFlatter<'a> {
    pub fn new(types: &'a Types) -> Self {
        Self { types }
    }

    pub fn flatten_parameter(&self, name: &Name, ty: &Type) -> Result<Vec<Ident>> {
        let mut tokens = Vec::new();

        let name = utils::yul_ident_name(name);
        let name = utils::tidy_name(&name);

        self._iter_type(&mut tokens, &name, ty, false)?;

        Ok(tokens)
    }

    pub fn flatten_return_type(&self, ty: &Type) -> Result<Vec<Ident>> {
        let mut tokens = Vec::new();

        self._iter_type(&mut tokens, "__yn_return", ty, true)?;

        Ok(tokens)
    }

    fn _iter_type(
        &self,
        tokens: &mut Vec<Ident>,
        name: &str,
        ty: &Type,
        void_generated: bool,
    ) -> Result<()> {
        let ident = Ident::new(name)?;

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
                tokens.push(ident);
            }
            Type::PointerType { addr_space: _ } => tokens.push(ident),
            Type::ArrayType {
                element_type,
                num_elements,
            } => {
                let e = element_type.as_ref();
                for i in 0..*num_elements {
                    let added_name = format!("{}_a{}", name, i);

                    self._iter_type(tokens, &added_name, e, void_generated)?;
                }
            }
            Type::StructType {
                element_types,
                is_packed: _,
            } => {
                for (i, e) in element_types.iter().enumerate() {
                    let e = e.as_ref();

                    let added_name = format!("{}_s{}", name, i);

                    self._iter_type(tokens, &added_name, e, void_generated)?;
                }
            }
            Type::NamedStructType { name } => {
                let ty = self
                    .types
                    .named_struct_def(name)
                    .ok_or(anyhow!("Linked error, failed to get named struct type."))?;

                if let NamedStructDef::Defined(ty) = ty {
                    self._iter_type(tokens, name, ty, void_generated)?;
                } else {
                    return Err(anyhow!("Linked error, no opaque supported"));
                }
            }
            _ => return Err(anyhow!("Unspported Type: {}", ty)),
        }

        Ok(())
    }
}
