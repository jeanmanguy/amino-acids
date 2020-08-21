use displaydoc::Display;
use thiserror::Error;

#[derive(Display, Error, Debug, PartialEq)]
/// Errors
pub enum AARegexError {
    /// Invalid character for an amino acid: `{0}`
    InvalidAminoAcidCharacter(char),
    /// Unknown error
    Unknown,
}
