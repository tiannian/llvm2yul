use anyhow::Result;

pub struct Literal(pub String);

impl Literal {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        Ok(Self(s.into()))
    }
}

pub struct Ident(pub String);

impl Ident {
    pub fn new(s: impl Into<String>) -> Result<Self> {
        Ok(Self(s.into()))
    }
}
