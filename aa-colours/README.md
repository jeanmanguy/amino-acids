# aa-colour ![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)

[![Rust](https://github.com/jeanmanguy/aa-colour/workflows/Rust/badge.svg?branch=master)](https://github.com/jeanmanguy/aa-regex/actions?query=workflow%3ARust)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/aa-colour)
[![Crates.io version](https://img.shields.io/crates/v/aa-colour)](https://crates.io/crates/aa-colour/)
[![Crates.io license](https://img.shields.io/crates/l/aa_regex)](https://github.com/jeanmanguy/aa-regex/blob/master/LICENSE)

🎨 Add colours to your amino acids in the terminal.

Code inspired from the crates [owo-colors](https://github.com/jam1garner/owo-colors) and [colored](https://github.com/mackwic/colored).

## Usage

```rust
use aa_colour::{AaColourise, palettes::Clustal};

println!("{}", AaColour::colour::<Clustal>(&'A').unwrap());
println!("{}", AaColour::blank::<Clustal>(&'-').unwrap());
```

## Features

- [X] colourise a single amino acid according to different palettes
  - [X] Clustal
  - [ ] Amino acid scales
    - Hydrophobicity
    - Disorder propensity
    - etc.
- [X] allow for amino acids to not be coloured (for protein alignments)
