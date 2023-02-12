//! **Update-index command** Base command: `git update-index
//! /// TODO: Documentation`


// Std
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

// External
use anyhow::Result;

// Internal
use crate::struct_set::Index;
use crate::util::gadget;


pub fn run(file_path: &str) -> Result<()> {

    let mut new_flag = false;
    let mut index = match Index::from_rawindex() {
        Ok(index) => index,
        Err(e) => { 
            println!("{}", e);
            new_flag = true;
            Index::empty()
        }
    };

    if !new_flag {
        let file_path = PathBuf::from(file_path);
        index.add(&file_path)?;
    }

    let index_path = gadget::get_index_path()?;
    let mut file = File::create(&index_path)?;
    file.write_all(&index.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn run_all() -> Result<()> {

    let index = Index::new_all()?;

    let index_path = gadget::get_index_path()?;
    let mut file = File::create(&index_path)?;
    file.write_all(&index.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn run_option_w() -> Result<()>  {

    let repo_path = gadget::get_repo_path()?;
    let mut all_paths = gadget::get_all_paths_ignore(&repo_path);
    all_paths.sort();

    println!("[List of files to be tracked]");
    for path in all_paths {
        println!("{}", path.strip_prefix(&repo_path).unwrap().to_str().unwrap())
    }

    Ok(())
}
