use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Config {
    pub basic_types: BTreeSet<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut basic_types = BTreeSet::new();

        basic_types.insert("patine_core::uint::U256".into());
        basic_types.insert("patine_core::uint::U128".into());
        basic_types.insert("patine_core::uint::U64".into());
        basic_types.insert("patine_core::uint::U32".into());
        basic_types.insert("patine_core::uint::U16".into());
        basic_types.insert("patine_core::uint::U8".into());

        Self { basic_types }
    }
}
