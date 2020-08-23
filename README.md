# aa-regex

![Rust](https://github.com/jeanmanguy/aa-regex/workflows/Rust/badge.svg?branch=master)
[![Lifecycle: experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://www.tidyverse.org/lifecycle/#experimental)


Utility macros to build regular expression matching protein sequences.

**Warning**: Work in progress, very experimental library, this is my very first crate.

## Motivation

 We can use regular expressions to match motifs on peptidic sequences, but writing them is a bit tedious. Some can become very complicated to read and understand. This crate tries to provide macros to help writing regular expressions at compile time. It uses procedural macros to generate strings that can be concatenated and used as regular expressions using the reduced alphabet of 20 amino acids. Any residue can now be limited to any valid amino acid instead of `.` (which would macth any valid unicode character) or manually writing the 20 amino acids and brackets. Does it make the regular expression search faster? I don't know.

## Setup

Add this to your `Cargo.toml`:

```toml
[dependencies]
aa-regex = { git = "https://github.com/jeanmanguy/aa-regex" }
```

## Usage

### Any

```rust
use aa_regex::any;

let any_amino_acid = any!();
// => "[ARNDCEQGHILKMFPSTWYV]"

let any_aromatics = any!('W', 'F', 'Y');
// => "[WFY]"
```

### Except

```rust
use aa_regex::except;

let no_proline = except!('P');
// => "[ARNDCEQGHILKMFSTWYV]"
```

### Aliases

TODO

- `any_aromatics!()`
- `except_aromatics!()`
- `except_proline!()`
- `any_charged!()`
- ...

### Concatenation

You can use the `concat!` macro from `std` to assemble the regular expression.

```rust
concat!(any!('D', 'E'), any!('R', 'H', 'K'))
```

## Ideas & bugs

Please create a new issue on the [project repository](https://github.com/jeanmanguy/aa-regex/issues).

## License

Aa-regex is distributed under the terms of the Apache License (Version 2.0). See [LICENSE](./LICENSE) for details.
