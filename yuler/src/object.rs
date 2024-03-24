use crate::{Block, Literal};

pub struct Object {
    pub code: Block,
    pub data: Literal,
    pub objects: Vec<Object>,
}
