/*! # Substitution matrices for use in protein sequence analysis and alignment.

This crate implements the trait `Similarity` which provide the `similarity()` function for each substitution matrix. Similarity data from <ftp://ftp.ncbi.nih.gov/blast/matrices/> is parsed and creates a match between all possible combination of 2 amino acids.

```rust
use aa_similarity::{Blosum62, Blosum65, Similarity, AminoAcid};
use std::convert::TryFrom;
# use std::error::Error;
# fn test_lib() -> Result<(), Box<dyn std::error::Error>> {

// from AminoAcid
assert_eq!(
    Blosum65::similarity(AminoAcid::GlutamicAcid, AminoAcid::AsparticAcid),
    2
);

// from char
let ala = AminoAcid::try_from('A')?;
let tyr = AminoAcid::try_from('Y')?;

assert_eq!(Blosum62::similarity(ala, tyr), -2);

# Ok(())
# }
# test_lib().unwrap()
```
*/
// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::match_same_arms, clippy::too_many_lines)]

pub use aa_name::AminoAcid;

pub trait Similarity {
    /// Returns the similarity score between 2 amino acids
    fn similarity(lhs: AminoAcid, rhs: AminoAcid) -> i16;
}

include!(concat!(env!("OUT_DIR"), "/generated_matrices.rs"));
