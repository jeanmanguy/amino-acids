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

fn join_regex_aa(aas: &[char]) -> Result<proc_macro2::TokenStream> {
    let mut out = String::new();
    let joined = aas.to_owned().into_iter().collect::<String>();

    if aas.len() == 1 {
        out += q!();
        out += &joined;
        out += q!();
    } else {
        out += any_start!();
        out += &joined;
        out += any_end!();
    }

    Ok(out.parse::<proc_macro2::TokenStream>().unwrap()) // TODO: get rid of unwrap
}

/* ----------------------------------- Any ---------------------------------- */

struct AnyInput(Vec<char>);

impl Parse for AnyInput {
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

        Ok(AnyInput(aas))
    }
}

impl AaRegexBuilder for AnyInput {
    fn build(&self) -> Result<proc_macro2::TokenStream> {
        join_regex_aa(&self.0)
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
    let input = parse_macro_input!(input as AnyInput);
    build(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/* --------------------------------- Except --------------------------------- */

struct ExceptInput(Vec<char>);

impl Parse for ExceptInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut aas_to_remove: Vec<char> = Vec::new();

        loop {
            let aa = input.parse::<syn::LitChar>()?;
            if all_aa_vec!().contains(&aa.value()) {
                aas_to_remove.push(aa.value());
            } else {
                return Err(Error::new(aa.span(), "expected amino acid 1-letter code"));
            }
            if input.parse::<Option<Token![,]>>()?.is_none() {
                // no comma -> end
                break;
            }
        }
        let mut aas = all_aa_vec!();
        aas.retain(|c| !aas_to_remove.contains(c));

        if aas.is_empty() {
            return Err(Error::new(
                input.span(),
                "cannot remove all the possible amino acids",
            ));
        }

        Ok(ExceptInput(aas))
    }
}

impl AaRegexBuilder for ExceptInput {
    fn build(&self) -> Result<proc_macro2::TokenStream> {
        join_regex_aa(&self.0)
    }
}

/// Except some amino acids
///
/// ```
/// #[macro_use]
///  use aa_regex::except;
///
/// let some = except!('C', 'D', 'E');
/// ```
#[proc_macro]
pub fn except(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExceptInput);
    build(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
