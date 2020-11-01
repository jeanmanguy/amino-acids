use displaydoc::Display;
use thiserror::Error;

#[derive(Display, Error, Debug)]
pub enum AaColourError {
    /// Expected an amino acid, found {0}
    NotAnAminoAcid(char),
}
