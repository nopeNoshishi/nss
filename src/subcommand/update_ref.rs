//! **update-marker command** Base command: `git update-ref`
//! 
//! /// TODO: Documentation

// Std
use std::io::prelude::*;
use std::fs::OpenOptions;

// External
use anyhow::{Result, bail};

// Internal
use crate::util::{gadget, file_system};

pub fn run(new_commit: &str) -> Result<()> {

    let head_path = gadget::get_head_path()?;

    let raw_content = file_system::read_object(new_commit)?;
    if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == String::from("c") {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(head_path)?;

        file.write_all(format!("bookmarker: {}", new_commit).as_bytes())?;
    } else {
        bail!("Not commit hash ({})", new_commit)
    }

    Ok(())
}

pub fn run_option_b(bookmarker: &str, new_commit: &str, old_commit: Option<&str>) -> Result<()> {
    let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

    let raw_content = file_system::read_object(new_commit)?;
    if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == String::from("c") {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(bookmark_path)?;
        let mut bookmark_hash = String::new();
        file.read_to_string(&mut bookmark_hash)?;

        if bookmark_hash == "".to_owned() {
            file.write_all(new_commit.as_bytes())?;
        } else if bookmark_hash == old_commit.unwrap().to_owned() {
            file.write_all(new_commit.as_bytes())?;
        } else {
            bail!("This bookmarker has the differnce old hash ({})", bookmark_hash)
        }
    } else {
        bail!("Not commit hash <new commit> ({})", new_commit)
    }

    Ok(())
}
