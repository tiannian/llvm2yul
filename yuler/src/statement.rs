use std::io::Write;

use anyhow::Result;

use crate::{Assignment, FunctionCall, Ident, Literal, Value, VariableDeclare};

#[derive(Debug, Clone)]
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

impl From<VariableDeclare> for Statement {
    fn from(value: VariableDeclare) -> Self {
        Self::VariableDeclare(value)
    }
}

impl From<Assignment> for Statement {
    fn from(value: Assignment) -> Self {
        Self::Assignment(value)
    }
}

impl From<FunctionCall> for Statement {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}

impl From<If> for Statement {
    fn from(value: If) -> Self {
        Self::If(value)
    }
}

impl Statement {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        match self {
            Self::VariableDeclare(v) => v.write(w)?,
            Self::Assignment(v) => v.write(w)?,
            Self::FunctionCall(v) => v.write(w)?,
            Self::If(v) => v.write(w)?,
            Self::Break => w.write_all(b"break")?,
            Self::Continue => w.write_all(b"continue")?,
            Self::Leave => w.write_all(b"leave")?,
            _ => {}
        }

        w.write_all(b";")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Block(pub Vec<Statement>);

impl Block {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(b"{\n")?;

        for v in &self.0 {
            v.write(w)?;

            w.write_all(b"\n")?;
        }

        w.write_all(b"}")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Value,
    pub block: Block,
}

impl If {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(b"if ")?;
        self.cond.write(w)?;
        w.write_all(b" ")?;
        self.block.write(w)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Switch {
    pub cond: Value,
    pub cases: Vec<CaseBlock>,
    pub default: Option<Block>,
}

impl Switch {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CaseBlock {
    pub cond: Literal,
    pub block: Block,
}

impl CaseBlock {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub init: Option<Block>,
    pub cond: Value,
    pub incr: Option<Block>,
}

impl ForLoop {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub rets: Vec<Ident>,
    pub block: Block,
}

impl FunctionDefinition {
    pub fn write(&self, w: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod stmt_tests {
    use anyhow::Result;

    use crate::{
        value_tests::build_fc,
        variable_tests::{build_as, build_vd},
        Assignment, Block, If, Statement, Value, VariableDeclare,
    };

    fn build_block() -> Result<Block> {
        let vd: Statement = build_vd()?.into();
        let ass: Statement = build_as()?.into();
        let fc: Statement = build_fc()?.into();

        let block = Block(vec![vd, ass, fc]);

        Ok(block)
    }

    fn build_if() -> Result<If> {
        let cond: Value = build_fc()?.into();
        let block = build_block()?;

        Ok(If { cond, block })
    }

    #[test]
    fn test_cd_block() -> Result<()> {
        let iff: Statement = build_if()?.into();

        let block = Block(vec![iff]);

        let mut res = Vec::new();
        block.write(&mut res)?;

        let res = String::from_utf8(res).unwrap();
        println!("{}", res);

        // println!("{:?}", String::from_utf8(res));
        // assert_eq!(res, b"let a, a := a();\na, a := a();");

        Ok(())
    }
}
