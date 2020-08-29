# aa-colour

🎨 Add colours to your amino acids in the terminal.

Work in progress and experimnetal crate. Changes are likely to break the API.

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
