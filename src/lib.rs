//! AA-colour
//!
//! ```rust
//! use aa_colour::{clustal_aa, Rgb};
//! assert_eq!(clustal_aa(&b'A').unwrap(), Rgb { r: 25u8, g: 127u8, b: 229u8})
//! ```

pub mod colors;
pub mod error;
pub mod palettes;

use core::marker::PhantomData;
use error::AaColourError;
use palettes::{Clustal, Palette};

/// RGB color
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Aa<'a, P: Palette>(&'a u8, PhantomData<P>);

impl<'a, T: Palette> Aa<'a, T> {
    fn colourise(&self) -> Result<Rgb, AaColourError> {
        match self.0 {
            b'A' | b'a' => Ok(T::A.to_owned()),
            _ => Err(AaColourError::NotAnAminoAcid(*self.0 as char)),
        }
    }
}

/// Returns the colour for an amino acid according to the Clustal palette
pub fn clustal_aa(aa: &u8) -> Result<Rgb, AaColourError> {
    Aa::<Clustal>(aa, PhantomData).colourise()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testme() {
        let aa = &b'A';
        let y = clustal_aa(aa).unwrap();
        println!("{:?}", y);
    }
}
