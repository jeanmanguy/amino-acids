/*! AA-colour

Add colours to your amino acids in the terminal.

Should work with Linux, MacOS and Windows.

```rust
use aa_colour::{AaColour, palettes::Clustal};

println!("{}", AaColour::colour::<Clustal>('A').unwrap());
println!("{}", AaColour::blank::<Clustal>('-').unwrap());
```
*/
// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

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
    /// Coloured amino acid according to its name
    /// # Errors
    ///
    /// Fails if `x` is not a 1-letter code for an amino acid
    /// # Example
    ///
    /// ```rust
    /// use aa_colour::{AaColour, palettes::Clustal};
    ///
    /// let coloured_aa = AaColour::colour::<Clustal>('C');
    /// ````
    pub fn colour<P: Palette>(x: char) -> Result<Self, AaColourError> {
        let col = match_char_aa::<P>(x)?;

        Ok(Self { aa: x, colour: col })
    }

    /// Uncoloured amino acid
    /// # Errors
    ///
    /// Fails if `x` is not a 1-letter code for an amino acid (TODO)
    /// # Example
    ///
    /// ```rust
    /// use aa_colour::{AaColour, palettes::Clustal};
    ///
    /// let blank_aa = AaColour::blank::<Clustal>('C');
    /// ````
    pub fn blank<P: Palette>(x: char) -> Result<Self, AaColourError> {
        Ok(Self {
            aa: x,
            colour: P::GAP,
        })
    }
}

macro_rules! match_aa {
    ($(
        $aa:literal $colour:ident
    ),* $(,)?) => {
        fn match_char_aa<P: Palette>(x: char) -> Result<&'static Rgb, AaColourError> {
            match x.to_ascii_uppercase() {
                $(
                    $aa => Ok(P::$colour),
                )*
                _ => Err(AaColourError::NotAnAminoAcid(x)),
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
            print!("{}", AaColour::colour::<Clustal>(aa).unwrap());
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
            print!("{}", AaColour::blank::<Clustal>(aa).unwrap());
        }
        println!(); //
    }
}
