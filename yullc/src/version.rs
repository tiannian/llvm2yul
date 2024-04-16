use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Version {
    #[value(name = "v0.8.25")]
    V0_8_25,
    #[value(name = "v0.8.24")]
    V0_8_24,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LLVMIRType {
    Bitcode,
    TextIR,
}

impl LLVMIRType {
    pub fn is_bitcode(&self) -> bool {
        matches!(self, Self::Bitcode)
    }
}
