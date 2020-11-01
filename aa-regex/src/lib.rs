/*! Macros for peptidic sequences regular expressions

 Collection of macros to help crafting regular expression matching peptidic sequences.

 ## Usage

 ```rust
 use aa_regex::{any, any_of, except };

let any = any!(); // => let any = "[ARNDCEQGHILKMFPSTWYV]";
assert_eq!(any, "[ARNDCEQGHILKMFPSTWYV]");

let any_aromatics = any_of!(W, F, Y); // => let any_aromatics = "[WFY]";
assert_eq!(any_aromatics, "[WFY]");

let no_proline = except!(P); // => let no_proline = "[ARNDCEQGHILKMFSTWYV]";
assert_eq!(no_proline, "[ARNDCEQGHILKMFSTWYV]");

let motif = concat!(any_of!(R, H, K), except!(P)); // => let motif = "[RHK][ARNDCEQGHILKMFSTWYV]";
assert_eq!(motif, "[RHK][ARNDCEQGHILKMFSTWYV]")
 ```
*/
// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
// #![allow(dead_code)]
// #![allow(clippy::missing_errors_doc)]

use proc_macro::{Literal, TokenStream, TokenTree};
use proc_macro2::Span;
use proc_macro_error::{abort, proc_macro_error};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Ident, Token};

#[derive(Debug)]
struct AaInput(Vec<Ident>);

impl Parse for AaInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Self(vars.into_iter().collect()))
    }
}

static AA_LIST: &[u8] = b"ARNDCEQGHILKMFPSTWYV";

/// Any amino acids
///
/// ## Usage
///
/// ```
/// #[macro_use]
///  use aa_regex::any;
///
/// let any = any!(); // => let any = "[ARNDCEQGHILKMFPSTWYV]";
/// assert_eq!(any, "[ARNDCEQGHILKMFPSTWYV]");
/// ```
///
/// ## Compilation errors
///
/// Compilation will fail if:
///
/// - arguments are given to the macro
#[proc_macro]
#[proc_macro_error]
pub fn any(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as AaInput);

    if !parsed.0.is_empty() {
        abort!(Span::call_site(), "No argument allowed")
    }

    // TODO check emty input
    core::iter::once(TokenTree::Literal(Literal::string(
        "[ARNDCEQGHILKMFPSTWYV]",
    )))
    .collect()
}

/// Any of the selected amino acids
///
/// ## Usage
///
/// ```
/// #[macro_use]
///  use aa_regex::any_of;
///
/// let one = any_of!(P);
/// assert_eq!(one, "P");
///
/// let some = any_of!(C, D, E);
/// assert_eq!(some, "[CDE]")
/// ```
///
/// ## Compilation errors
///
/// Compilation will fail if:
///
/// - non amino acid characters are used
/// - an amino acid was already added
/// - no amino acid to add
///  
#[proc_macro]
#[proc_macro_error]
pub fn any_of(input: TokenStream) -> TokenStream {
    let mut buffer = vec![b'['];
    let mut counter = 1_usize;
    let parsed = parse_macro_input!(input as AaInput);

    if parsed.0.is_empty() {
        abort!(Span::call_site(), "no amino acid to add")
    }
    for x in &parsed.0 {
        if buffer.len() > 20 {
            // 21 => 20 aa + [
            abort!(x.span(), "Cannot add more amino acids")
        }

        let as_string = x.to_string().to_uppercase();

        if as_string.len() > 1 {
            abort!(x.span(), "Expected only one character"; help = "only 1-letter amino acid code are accepted");
        }

        let aa_to_add = as_string.as_bytes().first().unwrap();
        if AA_LIST.contains(aa_to_add) {
            if None == buffer.iter().position(|aa| aa == aa_to_add) {
                buffer.push(*aa_to_add);
                counter += 1;
            } else {
                abort!(x.span(), "Amino acid already added");
            }
        } else {
            abort!(x.span(), "Not an amino acid");
        }
    }

    // add the finishing ] or remove the starting  [
    if counter > 2 {
        buffer.push(b']');
        counter += 1;
    } else {
        buffer.remove(0_usize);
        counter -= 1;
    }

    core::iter::once(TokenTree::Literal(Literal::string(
        core::str::from_utf8(&buffer[..counter]).unwrap(),
    )))
    .collect()
}

/// Except some amino acids
///
/// ## Usage
///
/// ```
/// #[macro_use]
///  use aa_regex::except;
///
/// let some = except!(C, D, E);
/// ```
///
/// ## Compilation errors
///
/// Compilation will fail if:
///
/// - non amino acid characters are used
/// - an amino acid was already removed
/// - there are too many exceptions
/// - there are no exceptions
///
#[proc_macro]
#[proc_macro_error]
pub fn except(input: TokenStream) -> TokenStream {
    let mut buffer = vec![
        b'[', b'A', b'R', b'N', b'D', b'C', b'E', b'Q', b'G', b'H', b'I', b'L', b'K', b'M', b'F',
        b'P', b'S', b'T', b'W', b'Y', b'V', b']',
    ];

    let mut counter = 22_usize;

    let parsed = parse_macro_input!(input as AaInput);

    if parsed.0.is_empty() {
        abort!(Span::call_site(), "no exceptions to add"; help = "add exceptions like `except!(P)` to avoid prolines for example")
    }

    for x in &parsed.0 {
        if buffer.len() < 4 {
            abort!(x.span(), "Cannot add more exceptions")
        }
        let as_string = x.to_string().to_uppercase();
        if as_string.len() > 1 {
            abort!(x.span(), "Expected only one character"; help = "only 1-letter amino acid code are accepted");
        }
        let aa_to_remove = as_string.as_bytes().first().unwrap();
        if AA_LIST.contains(aa_to_remove) {
            if let Some(position) = buffer.iter().position(|aa| aa == aa_to_remove) {
                buffer.remove(position);
                counter -= 1;
            } else {
                abort!(x.span(), "Exception already in place");
            }
        } else {
            abort!(x.span(), "Not an amino acid");
        }
    }

    core::iter::once(TokenTree::Literal(Literal::string(
        core::str::from_utf8(&buffer[..counter]).unwrap(),
    )))
    .collect()
}
