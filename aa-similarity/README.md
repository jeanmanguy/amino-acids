# aa-similarity

![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/aa-similarity)
[![Crates.io version](https://img.shields.io/crates/v/aa-similarity)](https://crates.io/crates/aa-similarity/)


## Usage

Add the following to your `Cargo.toml`

```toml
[dependencies]
aa-similarity = "0.1.0"
```


### Examples

```rust
use aa_similarity::{Blosum65, Similarity, AminoAcid};

assert_eq!(
    Blosum65::similarity(
        AminoAcid::GlutamicAcid,
        AminoAcid::AsparticAcid
    ),
    2
);
```

`aa-similarity` re-exports `AminoAcid` from [`aa-name`]. Amino acids from an alignment can be converted from chars or string like so:

```rust
use aa_similarity::{Blosum62, Similarity, AminoAcid};

let ala = AminoAcid::try_from('A')?;
let tyr = AminoAcid::try_from('Y')?;

assert_eq!(Blosum62::similarity(ala, tyr), -2);
```

See: [`aa-name`].


[`aa-name`]: https://github.com/jeanmanguy/amino-acids/tree/main/aa-name

## Supported matrices

Source: ftp://ftp.ncbi.nih.gov/blast/matrices/

- Blosum30
- Blosum35
- Blosum40
- Blosum45
- Blosum50
- Blosum55
- Blosum60
- Blosum62
- Blosum65
- Blosum70
- Blosum75
- Blosum80
- Blosum85
- Blosum90
- Blosum100
- Blosumn
- Dayhoff
- Identity
- Pam10
- Pam20
- Pam30
- Pam40
- Pam50
- Pam60
- Pam70
- Pam80
- Pam90
- Pam100
- Pam110
- Pam120
- Pam130
- Pam140
- Pam150
- Pam160
- Pam170
- Pam180
- Pam190
- Pam200
- Pam210
- Pam220
- Pam230
- Pam240
- Pam250
- Pam260
- Pam270
- Pam280
- Pam290
- Pam300
- Pam310
- Pam320
- Pam330
- Pam340
- Pam350
- Pam360
- Pam370
- Pam380
- Pam390
- Pam400
- Pam410
- Pam420
- Pam430
- Pam440
- Pam450
- Pam460
- Pam470
- Pam480
- Pam490
- Pam500


## Contributing

The project is maintained by Jean Manguy. Please submit a bug report or a feature request [on the Github issues page](https://github.com/jeanmanguy/amino-acids/issues/new/choose).

## License

`aa-similarity` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for
details.