use crate::{Assignment, FunctionCall, Ident, Literal, Value, VariableDeclare};

pub enum Statement {
    VariableDeclare(VariableDeclare),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    If(If),
    Break,
    Continue,
    Leave,
    ForLoop(ForLoop),
    FunctionDefinition(FunctionDefinition),
}

pub struct Block(pub Vec<Statement>);

pub struct If {
    pub cond: Value,
    pub block: Block,
}

pub struct Switch {
    pub cond: Value,
    pub cases: Vec<CaseBlock>,
    pub default: Option<Block>,
}

pub struct CaseBlock {
    pub cond: Literal,
    pub block: Block,
}

pub struct ForLoop {
    pub init: Option<Block>,
    pub cond: Value,
    pub incr: Option<Block>,
}

pub struct FunctionDefinition {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub rets: Vec<Ident>,
    pub block: Block,
}
