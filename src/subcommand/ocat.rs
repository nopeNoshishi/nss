//! **ocat command** ... Base command: `git cat-file`
//! 
//! Retrieve objects in the database and output their 
//! contents to standard output.

// External
use anyhow::Result;

// Internal
use crate::struct_set::Object;
use crate::util::file_system;

/// Register the object into object database (repository) 
/// and Display on standart-output.
pub fn run_option_p(hash: &str) -> Result<()> {
    let raw_content = file_system::read_object(hash)?;
    let object: Object = Object::from_content(raw_content)?;
    
    println!("{}", object);

    Ok(())
}

/// Output the object type
pub fn run_option_t(hash: &str) -> anyhow::Result<()> {
    let raw_content = file_system::read_object(hash)?;
    let object: Object = Object::from_content(raw_content)?;
    
    println!("{}", object.as_str());

    Ok(())
}

