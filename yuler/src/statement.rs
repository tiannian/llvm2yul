use std::io::Write;

use anyhow::Result;

use crate::{
    Assignment, Block, ForLoop, FunctionCall, Ident, If, Literal, Switch, Value, VariableDeclare,
    Writer,
};

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclare(VariableDeclare),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    If(If),
    Switch(Switch),
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

impl From<Switch> for Statement {
    fn from(value: Switch) -> Self {
        Self::Switch(value)
    }
}

impl From<ForLoop> for Statement {
    fn from(value: ForLoop) -> Self {
        Self::ForLoop(value)
    }
}

impl Statement {
    pub fn write<W>(&self, w: &mut Writer<W>) -> Result<()>
    where
        W: Write,
    {
        match self {
            Self::VariableDeclare(v) => v.write(w)?,
            Self::Assignment(v) => v.write(w)?,
            Self::FunctionCall(v) => v.write(w)?,
            Self::If(v) => v.write(w)?,
            Self::Switch(v) => v.write(w)?,
            Self::Break => w.write_str("break")?,
            Self::Continue => w.write_str("continue")?,
            Self::Leave => w.write_str("leave")?,
            Self::ForLoop(v) => v.write(w)?,
            _ => {}
        }

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
pub(crate) mod stmt_tests {
    use anyhow::Result;

    use crate::{
        for_loop_tests::build_for_loop,
        value_tests::build_fc,
        variable_tests::{build_as, build_vd},
        Block, CaseBlock, If, Literal, Statement, Switch, Value, Writer,
    };

    pub fn build_block() -> Result<Block> {
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

    fn build_switch() -> Result<Switch> {
        let cast_cond = Literal::hex_number("0x01")?;
        let block = build_block()?;
        let case = CaseBlock {
            cond: cast_cond,
            block,
        };

        let cond: Value = build_fc()?.into();
        let sw = Switch {
            cond,
            cases: vec![case],
            default: None,
        };

        Ok(sw)
    }

    #[test]
    fn test_cd_block() -> Result<()> {
        let iff: Statement = build_if()?.into();
        let sw: Statement = build_switch()?.into();
        let fl: Statement = build_for_loop()?.into();

        let block = Block(vec![
            iff,
            Statement::Continue,
            Statement::Break,
            Statement::Leave,
            sw,
            fl,
        ]);

        let mut res = Writer::new(Vec::new(), "    ");
        block.write(&mut res)?;

        let res = String::from_utf8(res.w).unwrap();
        println!("{}", res);

        // println!("{:?}", String::from_utf8(res));
        // assert_eq!(res, b"let a, a := a();\na, a := a();");

        Ok(())
    }
}
