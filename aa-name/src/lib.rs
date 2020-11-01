/*! AA-name

Base enum for amino acids (20 basic proteinoformes amino acids)

``` rust
let ala: AminoAcid = "Ala".parse().unwrap();
assert_eq!(ala, AminoAcid::Alanine);

let tyr: AminoAcid = AminoAcid::try_from('Y').unwrap();
assert_eq!(tyr, AminoAcid::Tyrosine);
```

*/

use std::{convert::TryFrom, str::FromStr};
use thiserror::Error;

/// # Amino acid codes
///
/// As defined in [Dayhoff1965].
///
/// ```rust
/// use aa_name::{AminoAcid, Code};
///
/// let alanine = AminoAcid::Alanine;
/// let tyrosine = AminoAcid::Tyrosine;
///
/// // get one letter code
/// assert_eq!(alanine.single_letter(), 'A');
///
/// // get three letters code
/// assert_eq!(tyrosine.three_letters(), "Tyr");
///
/// let mut buffer = Vec::new();
/// tyrosine.write_single_letter(&mut buffer).unwrap();
/// alanine.write_single_letter(&mut buffer).unwrap();
/// assert_eq!(std::str::from_utf8(&buffer).unwrap(), "YA");
///
/// let mut buffer = Vec::new();
/// tyrosine.write_three_letters(&mut buffer).unwrap();
/// alanine.write_three_letters(&mut buffer).unwrap();
/// assert_eq!(std::str::from_utf8(&buffer).unwrap(), "TyrAla");
/// ```
///
/// ## References
/// - [Dayhoff1965]: Dayhoff, MO, RV Eck, MA Chang, and MR Sochard. 1965. ‘Atlas of Protein Sequence and Structure’. <https://ntrs.nasa.gov/citations/19660014530>
pub trait Code {
    fn single_letter(&self) -> char;
    fn three_letters(&self) -> String;
    fn write_single_letter(
        &self,
        writer: impl std::io::Write,
    ) -> std::result::Result<(), std::io::Error>;
    fn write_three_letters(
        &self,
        writer: impl std::io::Write,
    ) -> std::result::Result<(), std::io::Error>;
}

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("cannot parse amino acid from: `{0}`")]
    ParseAminoAcidError(String),
}

macro_rules! aa {

    ($($name:ident $one_letter_code:literal $full_name:literal $three_letters_code:literal),*) => {

        /// One variant for each amino acid
        ///
        /// ``` rust
        /// let ala: AminoAcid = "Ala".parse().unwrap();
        /// assert_eq!(ala, AminoAcid::Alanine);
        ///
        /// let tyr: AminoAcid = AminoAcid::try_from('Y').unwrap();
        /// assert_eq!(tyr, AminoAcid::Tyrosine);
        /// ```
        #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
        pub enum AminoAcid {
            $(
                $name,
            )*
        }

        impl ::std::fmt::Display for AminoAcid {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $(
                        Self::$name => write!(f, $full_name)
                    ),*
                }
            }
        }

        impl Code for AminoAcid {
            fn single_letter(&self) -> char {
                match *self {
                    $(
                        Self::$name => $one_letter_code
                    ),*
                }

            }

            fn three_letters(&self) -> String {
                match *self {
                    $(
                        Self::$name => String::from($three_letters_code),
                    )*
                }
            }

            fn write_single_letter(&self, mut writer: impl std::io::Write) -> std::result::Result<(), std::io::Error> {
                match *self {
                    $(
                        Self::$name => write!(writer, "{}", $one_letter_code)
                    ),*
                }
            }

            fn write_three_letters(&self, mut writer: impl std::io::Write) -> std::result::Result<(), std::io::Error> {
                match *self {
                    $(
                        Self::$name => write!(writer, $three_letters_code)
                    ),*
                }
            }
        }

        impl FromStr for AminoAcid {
            type Err = ErrorKind;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $three_letters_code | $full_name => Ok(Self::$name),
                    )*
                    _ => Err(ErrorKind::ParseAminoAcidError(s.to_string())),
                }
            }
        }

        impl TryFrom<char> for AminoAcid {
            type Error = ErrorKind;

            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $one_letter_code => Ok(Self::$name),
                    )*
                    _ => Err(ErrorKind::ParseAminoAcidError(value.to_string())),
                }
            }
        }
    };
}

aa! {
    Alanine 'A' "Alanine" "Ala",
    Arginine 'R' "Arginine" "Arg",
    Asparagine 'N' "Asparagine" "Asn",
    AsparticAcid 'D' "Aspartic acid" "Asp",
    Cysteine 'C' "Cysteine" "Cys",
    GlutamicAcid 'E' "Glutamic acid" "Glu",
    Glutamine 'Q' "Glutamine" "Gln",
    Glycine 'G' "Glycine" "Gly",
    Histidine 'H' "Histidine" "His",
    Isoleucine 'I' "Isoleucine" "Ile",
    Leucine 'L' "Leucine" "Leu",
    Lysine 'K' "Lysine" "Lys",
    Methionine 'M' "Methionine" "Met",
    Phenylalanine 'F' "Phenylalanine" "Phe",
    Proline 'P' "Proline" "Pro",
    Serine 'S' "Serine" "Ser",
    Threonine 'T' "Threonine" "Thr",
    Tryptophan 'W' "Tryptophan" "Trp",
    Tyrosine 'Y' "Tyrosine" "Tyr",
    Valine 'V' "Valine" "Val"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let ala: AminoAcid = "Ala".parse().unwrap();

        assert_eq!(ala, AminoAcid::Alanine);
    }

    #[test]
    fn test_from_char() {
        let tyr: AminoAcid = AminoAcid::try_from('Y').unwrap();
        assert_eq!(tyr, AminoAcid::Tyrosine)
    }
}
