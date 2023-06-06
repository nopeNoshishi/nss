//! **Bookmark command** Base command: `git branch`
//!
//! A bookmark is a special tool that allows you to refer to
//! a specific commit. You can easily go back to the book
//! (change history) that you have carefully built up.

// Std
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

// External
use anyhow::{bail, Context, Result};

// Internal
use crate::util::{file_system, gadget};

/// Create a new bookmarker to argument commit hash.
///
/// **Note:** If you do not specify a hash, it refers to the
/// value pointed to by HEAD.
pub fn run(book_name: &str, hash: Option<&String>) -> Result<()> {
    let bookmark_path = gadget::get_bookmarks_path(book_name)?;
    match hash {
        Some(v) => {
            let raw_content = file_system::read_object(v)?;
            if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == *"c" {
                let mut file = OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(bookmark_path)
                    .with_context(|| format!("{} already exits", book_name))?;

                file.write_all(v.as_bytes())?;
            } else {
                bail!("Not commit hash ({})", v)
            }
        }
        _ => {
            let head_path = gadget::get_head_path()?;
            let head_hash = fs::read_to_string(head_path)?;
            let mut file = File::create(bookmark_path)?;
            file.write_all(head_hash.as_bytes())?
        }
    }
    Ok(())
}

/// Change the pointer of existing bookmarks.
///
/// **Note:** If you do not specify a hash, it refers to the
/// value pointed to by HEAD.
pub fn run_option_r(bookmarker: &str, hash: &String) -> Result<()> {
    let taregt_path = gadget::get_bookmarks_path(bookmarker)?;

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(taregt_path)
        .with_context(|| format!("No such bookmarker: {}", bookmarker))?;

    let raw_content = file_system::read_object(hash)?;
    if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == *"c" {
        file.write_all(hash.as_bytes())?;
    } else {
        bail!("Not commit hash ({})", hash)
    }

    Ok(())
}

// TODO: off  opitionする！
