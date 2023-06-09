//! **ocat command** ... Base command: `git cat-file`
//!
//! Retrieve objects in the database and output their
//! contents to standard output.

// External
use anyhow::Result;

// Internal
use crate::repo::NssRepository;

/// Register the object into object database (repository)
/// and Display on standart-output.
pub fn run_option_p(repository: NssRepository, hash: &str) -> Result<()> {
    let object = repository.read_object(hash)?;
    println!("{}", object);

    Ok(())
}

/// Output the object type
pub fn run_option_t(repository: NssRepository, hash: &str) -> anyhow::Result<()> {
    let object = repository.read_object(hash)?;

    println!("{}", object.as_str());

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_run_option_t() {}
}
