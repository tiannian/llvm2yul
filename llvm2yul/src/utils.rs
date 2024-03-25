pub fn tidy_name(name: impl Into<String>) -> String {
    let s = name.into();

    s.replace(".-", "")
}
