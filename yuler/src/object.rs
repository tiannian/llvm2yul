use crate::{Block, HexLiteral};

pub struct Object {
    pub code: Block,
    pub data: HexLiteral,
    pub objects: Vec<Object>,
}
