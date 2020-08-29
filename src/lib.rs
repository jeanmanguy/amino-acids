//! AA-colour
//!
//! Work in progress
//!
//! ```rust
//! use aa_colour::{AaColour, palettes::Clustal};
//!
//! println!("{}", AaColour::colour::<Clustal>(&'A').unwrap());
//! println!("{}", AaColour::blank::<Clustal>(&'-').unwrap());
//! ```

pub mod error;
pub mod palettes;

use core::fmt;
use error::AaColourError;
use palettes::{Palette, Rgb};

/// Amino acid background colour
#[derive(Debug, Clone)]
pub struct AaColour {
    aa: char,
    colour: &'static Rgb,
}

impl fmt::Display for AaColour {
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

impl AaColour {
    pub fn colour<P: Palette>(x: &char) -> Result<AaColour, AaColourError> {
        let col = match_char_aa::<P>(x)?;

        Ok(AaColour {
            aa: *x,
            colour: col,
        })
    }

    pub fn blank<P: Palette>(x: &char) -> Result<AaColour, AaColourError> {
        Ok(AaColour {
            aa: *x,
            colour: P::GAP,
        })
    }
}

macro_rules! match_aa {
    ($(
        $aa:literal $colour:ident
    ),* $(,)?) => {
        fn match_char_aa<P: Palette>(x: &char) -> Result<&'static Rgb, AaColourError> {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::palettes::Clustal;

    #[test]
    fn all_amino_acids_clustal() {
        let aas = vec![
            'A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
            'V', 'W', '-',
        ];
        for aa in aas {
            print!("{}", AaColour::colour::<Clustal>(&aa).unwrap());
        }
        println!(); //
    }

    #[test]
    fn all_amino_acids_clustal_blanks() {
        let aas = vec![
            'A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T',
            'V', 'W', '-',
        ];
        for aa in aas {
            print!("{}", AaColour::blank::<Clustal>(&aa).unwrap());
        }
        println!(); //
    }
}
