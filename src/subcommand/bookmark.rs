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
use crate::repo::NssRepository;

/// Create a new bookmarker to argument commit hash.
///
/// **Note:** If you do not specify a hash, it refers to the
/// value pointed to by HEAD.
pub fn run(repository: &NssRepository, book_name: &str, hash: Option<&String>) -> Result<()> {
    match hash {
        Some(v) => {
            let object = repository.read_object(v)?;
            if object.as_str() == "commit" {
                let mut file = OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(repository.bookmarks_path(book_name))
                    .with_context(|| format!("{} already exits", book_name))?;

                file.write_all(v.as_bytes())?;
            } else {
                bail!("Not commit hash ({})", v)
            }
        }
        _ => {
            let head_hash = fs::read_to_string(repository.head_path())?;
            let mut file = File::create(repository.bookmarks_path(book_name))?;
            file.write_all(head_hash.as_bytes())?
        }
    }
    Ok(())
}

/// Change the pointer of existing bookmarks.
///
/// **Note:** If you do not specify a hash, it refers to the
/// value pointed to by HEAD.
pub fn run_option_r(repository: &NssRepository, bookmarker: &str, hash: &String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(repository.bookmarks_path(bookmarker))
        .with_context(|| format!("No such bookmarker: {}", bookmarker))?;

    let object = repository.read_object(hash)?;
    if object.as_str() == "commit" {
        file.write_all(hash.as_bytes())?;
    } else {
        bail!("Not commit hash ({})", hash)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_run_option_r() {}
}
