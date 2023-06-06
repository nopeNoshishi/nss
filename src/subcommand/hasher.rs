//! **hasher command** ... Base command: `git hash-object`
//!
//! Generate objects from actual files and directories.

// Std
use std::path::Path;

// External
use anyhow::Result;

// Internal
use crate::struct_set::{Blob, Hashable};
use crate::util::file_system;

/// Calculate the hash value of the given file and output this.
pub fn run<P: AsRef<Path>>(path: P) -> Result<()> {

    let blob = Blob::new(path.as_ref())?;
    println!("{}", hex::encode(blob.to_hash()));

    Ok(())
}

/// Register the object into object database (repository) and output this.
pub fn run_option_w<P: AsRef<Path>>(path: P) -> Result<()> {
    let blob = Blob::new(path.as_ref())?;

    let hash = hex::encode(blob.to_hash());
    file_system::write_blob(&hash, blob)?;

    println!("{}", hash);

    Ok(())
}
