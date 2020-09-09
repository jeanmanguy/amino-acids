use aa_regex::*;
use regex::Regex;

#[test]
fn some() {
    assert_eq!(except!(C, D), "[ARNEQGHILKMFPSTWYV]")
}

#[test]
fn some_regex() {
    let re = Regex::new(except!(C, D));
    assert!(re.is_ok())
}

#[test]
fn fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/except_*.rs");
}
