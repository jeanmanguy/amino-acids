#[doc(hidden)]
#[macro_export]
macro_rules! __regex_start {
    () => {
        "["
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __regex_end {
    () => {
        "]"
    };
}

#[macro_export]
macro_rules! all {
    () => {
        "ARNDCEQGHILKMFPSTWYV"
    };
}

#[macro_export]
macro_rules! any {
    () => {
        concat!($crate::__regex_start!(), $crate::all!(), $crate::__regex_end!())
    };
    ($($aa:expr),+) => {
        concat!(
            // start any bracket + comma
            $crate::__regex_start!()
            ,
            // the amino acid to include + a comma
            $($aa ,)+
            // end any bracket (final comma provided by the repetition)
            $crate::__regex_end!()
        )
    };
}

// #[inline]
// pub fn any_except(exceptions: &str) -> String {
//     // TODO: check if exceptions are valid amino acids
//     // TODO: make version acceptiing a Vec of exceptions instead
//     let mut aas = any_aa();
//     aas.retain(|c| !exceptions.contains(c));
//     aas
// }

#[macro_export]
macro_rules! charged_pos {
    () => {
        "[RHK]"
    };
}

#[macro_export]
macro_rules! no_charged_pos {
    () => {
        "[ANDCEQGILMFPSTWYV]"
    };
}

#[macro_export]
macro_rules! charged_neg {
    () => {
        "[DE]"
    };
}

#[macro_export]
macro_rules! no_charged_neg {
    () => {
        "[ARNCQGHILKMFPSTWYV]"
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test_case::test_case;

    #[test]
    fn test_any() {
        // TODO: split and use test case
        assert_eq!(any!().len(), 22);
        assert_eq!(any!('C'), "[C]");
        assert_eq!(any!("C", "G"), "[CG]");
    }
}
