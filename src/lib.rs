pub mod aa;
mod builder;
mod errors;
mod utils;

pub use errors::AARegexError;

pub fn new() -> Result<(), AARegexError> {
    // Ok(())
    Err(AARegexError::Unknown)
}
