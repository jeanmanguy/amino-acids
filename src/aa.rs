/// Valid one letter code amino acid
const AMINO_ACIDS_LIST: &str = r"ARNDCEQGHILKMFPSTWYV";

/// Any amino acids
///
/// Note: Only uppercase amino acids codes are supported.
///
/// # Examples
/// ```
/// use aa_regex::aa;
/// let any_aa: String = aa::any();
/// assert_eq!(any_aa, "[ARNDCEQGHILKMFPSTWYV]")
/// ```
#[inline]
pub fn any() -> String {
    format!("[{}]", AMINO_ACIDS_LIST)
}

#[inline]
fn any_ofs(x: &str) -> String {
    // TODO: check correct aa
    format!("[{}]", x)
}

#[inline]
pub fn any_except(exceptions: &str) -> String {
    // TODO: check if exceptions are valid amino acids
    // TODO: make version acceptiing a Vec of exceptions instead
    let mut aas = any();
    aas.retain(|c| !exceptions.contains(c));
    aas
}

pub fn any_positive() -> String {
    any_ofs("RHK")
}

pub fn any_except_positive() {
    todo!()
}

pub fn any_except_negative() -> String {
    any_ofs("DE")
}

/// Any aromatics
///
/// - Tyrosine
/// - Tryptophan
/// - Phenylalanine
///
/// Note: Only uppercase amino acids codes are supported.
///
/// # Examples
/// ```
/// use aa_regex::aa;
/// let any_aa: String = aa::any_aromatic();
/// assert_eq!(any_aa, "[YWF]")
/// ```
pub fn any_aromatic() -> String {
    any_ofs("YWF")
}

pub fn any_except_aromatic() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test_case::test_case;

    #[test]
    fn test_any() {
        assert_eq!(any().len(), 22);
    }

    #[test]
    fn test_any_ofs() {
        assert_eq!(any().len(), 22);
    }

    #[test]
    fn test_any_except() {
        assert_eq!(any_ofs("CG").len(), 4);
    }
}
