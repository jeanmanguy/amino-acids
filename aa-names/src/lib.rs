use std::{convert::TryFrom, str::FromStr};
use thiserror::Error;

pub trait Names {
    const NAME: &'static str;
    const CODE1: char;
    const CODE3: &'static str;

    fn name(&self) -> &str {
        Self::NAME
    }

    fn one_letter_code(&self) -> char {
        Self::CODE1
    }

    fn three_letters_code(&self) -> &str {
        Self::CODE3
    }
}

#[derive(Error, Debug)]
pub enum ErroKind {
    #[error("cannot parse amino acid from: `{0}`")]
    ParseAminoAcidError(String),
}

macro_rules! aa {
    ($($name:ident $one_letter_code:literal $full_name:literal $three_letter_code:literal),*) => {
        $(
            #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Default)]
            pub struct $name;

            impl Names for $name {
                const NAME: &'static str = $full_name;
                const CODE1: char = $one_letter_code;
                const CODE3: &'static str = $three_letter_code;
            }

        )*

        #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
        pub enum AminoAcid {
            $(
                $name($name),
            )*
            Gap
        }

        impl FromStr for AminoAcid {
            type Err = ErroKind;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $three_letter_code | $full_name => Ok(Self::$name($name {})),
                    )*
                    "-" => Ok(Self::Gap),
                    _ => Err(ErroKind::ParseAminoAcidError(s.to_string())),
                }
            }
        }

        impl TryFrom<char> for AminoAcid {
            type Error = ErroKind;

            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $one_letter_code => Ok(Self::$name($name {})),
                    )*
                    '-' => Ok(Self::Gap),
                    _ => Err(ErroKind::ParseAminoAcidError(value.to_string())),
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

        assert_eq!(ala, AminoAcid::Alanine(Alanine {}));
    }

    #[test]
    fn test_from_char() {
        let tyr: AminoAcid = AminoAcid::try_from('Y').unwrap();
        assert_eq!(tyr, AminoAcid::Tyrosine(Tyrosine {}))
    }
}
