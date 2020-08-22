pub mod aa;
mod errors;

pub use errors::AARegexError;

pub fn new() -> Result<(), AARegexError> {
    // Ok(())
    Err(AARegexError::Unknown)
}
