use std::io::Write;

use anyhow::Result;

use crate::{
    Assignment, ForLoop, FunctionCall, FunctionDefinition, If, Switch, VariableDeclare, Writer,
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

macro_rules! define_impl {
    ($t:ty, $e:ident) => {
        impl From<$e> for $t {
            fn from(value: $e) -> Self {
                Self::$e(value)
            }
        }
    };
}

define_impl!(Statement, VariableDeclare);
define_impl!(Statement, Assignment);
define_impl!(Statement, FunctionCall);
define_impl!(Statement, If);
define_impl!(Statement, Switch);
define_impl!(Statement, ForLoop);
define_impl!(Statement, FunctionDefinition);

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
            Self::FunctionDefinition(v) => v.write(w)?,
        }

        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod stmt_tests {
    use anyhow::Result;

    use crate::{
        for_loop_tests::build_for_loop,
        function_defination_tests::build_function_defination,
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
        let fd: Statement = build_function_defination()?.into();

        let block = Block(vec![
            iff,
            Statement::Continue,
            Statement::Break,
            Statement::Leave,
            sw,
            fl,
            fd,
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
