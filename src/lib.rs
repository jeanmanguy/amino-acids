//! Macros for peptidic sequences
//!
//! Collection of macros to help crafting regular expression matching peptidic sequences.
use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Result, Token,
};

macro_rules! all_aa_vec {
    () => {
        vec![
            'A', 'R', 'N', 'D', 'C', 'E', 'Q', 'G', 'H', 'I', 'L', 'K', 'M', 'F', 'P', 'S', 'T',
            'W', 'Y', 'V',
        ]
    };
}

macro_rules! q {
    () => {
        "\""
    };
}

macro_rules! any_start {
    () => {
        "\"["
    };
}

macro_rules! any_end {
    () => {
        "]\""
    };
}

trait AaRegexBuilder {
    fn build(&self) -> Result<proc_macro2::TokenStream>;
}

fn build<T: AaRegexBuilder + Sized>(aas: T) -> Result<proc_macro2::TokenStream> {
    aas.build()
}

struct AaInput(Vec<char>);

impl Parse for AaInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut aas: Vec<char> = Vec::new();

        if input.is_empty() {
            aas = all_aa_vec!();
        } else {
            loop {
                let aa = input.parse::<syn::LitChar>()?;
                if all_aa_vec!().contains(&aa.value()) {
                    aas.push(aa.value());
                } else {
                    return Err(Error::new(aa.span(), "expected amino acid 1-letter code"));
                }
                if input.parse::<Option<Token![,]>>()?.is_none() {
                    // no comma -> end
                    break;
                }
            }
        }

        Ok(AaInput(aas))
    }
}

impl AaRegexBuilder for AaInput {
    fn build(&self) -> Result<proc_macro2::TokenStream> {
        let mut out = String::new();

        let joined = self.0.to_owned().into_iter().collect::<String>();

        if self.0.len() == 1 {
            out += q!();
            out += &joined;
            out += q!();
        } else {
            out += any_start!();
            out += &joined;
            out += any_end!();
        }

        let result = out.parse::<proc_macro2::TokenStream>().unwrap();

        Ok(result)
    }
}

/// Any amino acid or any of...
///
/// Any of all valid amino acids or any of the selected amino acids
///
/// ```
/// #[macro_use]
///  use aa_regex::any;
///
/// let some = any!('C', 'D', 'E');
/// let all = any!();
/// ```
#[proc_macro]
pub fn any(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AaInput);
    build(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
