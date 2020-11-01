# aa-colour 

![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/aa-colour)
[![Crates.io version](https://img.shields.io/crates/v/aa-colour)](https://crates.io/crates/aa-colour/)

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


## Contributing

The project is maintained by Jean Manguy. Please submit a bug report or a feature request [on the Github issues page](https://github.com/jeanmanguy/amino-acids/issues/new/choose).

## License

`aa-colour` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for
details.