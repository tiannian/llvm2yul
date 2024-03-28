use llvm_ir::Name;

pub fn yul_ident_name(n: &Name) -> String {
    match n {
        Name::Name(n) => tidy_name(n),
        Name::Number(n) => format!("_l{n}"),
    }
}

pub fn tidy_name(s: &str) -> String {
    s.replace(['.', '$'], "_")
}
