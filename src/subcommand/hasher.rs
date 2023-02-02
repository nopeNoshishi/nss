//! **hasher command** ... Base command: `git hash-object`
//! 
//! Generate objects from actual files and directories.

// Std
use std::path::Path;

// External
use anyhow::Result;


// Internal
use crate::util::file_system;
use crate::struct_set::{Object, Hashable};

/// Calculate the hash value of the given file and output this.
pub fn run<P: AsRef<Path>> (path: P) -> Result<()> {
    let object = Object::new(path.as_ref())?;
    let hash = hex::encode(object.to_hash());
    println!("{}", &hash);

    Ok(())
}

/// Register the object into object database (repository) and output this.
pub fn run_option_w<P: AsRef<Path>> (path: P) -> Result<()> {    
    let object = Object::new(path.as_ref())?;

    let hash = hex::encode(object.to_hash());
    file_system::write_object(&hash, object)?;

    println!("{}", hash);

    Ok(())
}
