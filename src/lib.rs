//! AA-colour
//!

pub mod error;
pub mod palettes;

use core::fmt;
use error::AaColourError;
use palettes::{Palette, Rgb};

/// Amino acid background colour
#[derive(Debug, Clone)]
pub struct AaColourDisplay {
    aa: char,
    colour: &'static Rgb,
}

impl fmt::Display for AaColourDisplay {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "\x1B[48;2;{};{};{};30m",
            self.colour.r, self.colour.g, self.colour.b
        ))?;
        fmt::Display::fmt(&self.aa, f)?;
        f.write_str("\x1b[0m")
    }
}

macro_rules! match_aa {
    ($(
        $aa:literal $colour:ident
    ),* $(,)?) => {
        fn match_char_aa<P: Palette>(x: &char) -> Result<&'static Rgb, AaColourError> {
            // TODO: use const lookup table
            match x.to_ascii_uppercase() {
                $(
                    $aa => Ok(P::$colour),
                )*
                _ => Err(AaColourError::NotAnAminoAcid(*x)),
            }
        }
    };
}

match_aa! {
    'A' A,
    'C' C,
    'D' D,
    'E' E,
    'F' F,
    'G' G,
    'H' H,
    'I' I,
    'K' K,
    'L' L,
    'M' M,
    'N' N,
    'P' P,
    'Q' Q,
    'R' R,
    'S' S,
    'T' T,
    'V' V,
    'W' W,
    'Y' Y,
    '-' GAP
}

pub trait AaColourise {
    fn colourise<P: Palette>(&self) -> Result<AaColourDisplay, AaColourError>;

    fn col_aa<P: Palette>(x: &Self) -> Result<AaColourDisplay, AaColourError> {
        x.colourise::<P>()
    }
}

impl AaColourise for char {
    fn colourise<P: Palette>(&self) -> Result<AaColourDisplay, AaColourError> {
        let col = match_char_aa::<P>(self)?;

        Ok(AaColourDisplay {
            aa: *self,
            colour: col,
        })
    }
}

impl AaColourise for u8 {
    fn colourise<P: Palette>(&self) -> Result<AaColourDisplay, AaColourError> {
        let aa = *self as char;
        let col = match_char_aa::<P>(&aa)?;

        Ok(AaColourDisplay { aa, colour: col })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::palettes::Clustal;

    #[test]
    fn testme2() {
        let aas = vec![
            'A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
            'V', 'W', '-',
        ];
        for aa in aas {
            println!("{}", AaColourise::col_aa::<Clustal>(&aa).unwrap());
        }
    }
}
