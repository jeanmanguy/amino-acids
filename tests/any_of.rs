use aa_regex::*;
use regex::Regex;

#[test]
fn one() {
    assert_eq!(any_of!(C), "C")
}

#[test]
fn one_regex() {
    let re = Regex::new(any_of!(C));
    assert!(re.is_ok())
}

#[test]
fn some() {
    assert_eq!(any_of!(C, D), "[CD]")
}

#[test]
fn some_regex() {
    let re = Regex::new(any_of!(C, D));
    assert!(re.is_ok())
}

// #[test]
// fn concat() {
//     let multi_position_regex = concat!(any_of!(R), any_of!(), any_of!(C, D));
//     assert_eq!(multi_position_regex, "R[ARNDCEQGHILKMFPSTWYV][CD]");
// }

// #[test]
// fn concat_regex() {
//     let multi_position_regex = concat!(any_of!('R'), any_of!(), any_of!('C', 'D'));
//     let re = Regex::new(multi_position_regex);
//     assert!(re.is_ok())
// }

#[test]
fn fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fails/any_of_*.rs");
}
