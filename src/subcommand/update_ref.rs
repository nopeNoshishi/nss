//! **update-marker command** Base command: `git update-ref`
//!
//! /// TODO: Documentation

// Std
use std::fs::OpenOptions;
use std::io::prelude::*;

// External
use anyhow::{bail, Result};

// Internal
use crate::util::file_system;
use crate::util::gadget::NssRepository;

pub fn run(repository: NssRepository, new_commit: &str) -> Result<()> {
    let raw_content = file_system::read_object(repository.path(), new_commit)?;
    if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == *"c" {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(repository.head_path())?;

        file.write_all(format!("bookmarker: {}", new_commit).as_bytes())?;
    } else {
        bail!("Not commit hash ({})", new_commit)
    }

    Ok(())
}

pub fn run_option_b(
    repository: NssRepository,
    bookmarker: &str,
    new_commit: &str,
    old_commit: Option<&str>,
) -> Result<()> {
    let raw_content = file_system::read_object(repository.path(), new_commit)?;
    if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == *"c" {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(repository.bookmarks_path(bookmarker))?;
        let mut bookmark_hash = String::new();
        file.read_to_string(&mut bookmark_hash)?;

        if bookmark_hash.is_empty() {
            file.write_all(new_commit.as_bytes())?;
        } else if let Some(commit) = old_commit {
            if bookmark_hash == commit {
                file.write_all(new_commit.as_bytes())?;
            } else {
                bail!(
                    "This bookmarker has the difference old hash ({})",
                    bookmark_hash
                );
            }
        } else {
            bail!(
                "This bookmarker has the difference old hash ({})",
                bookmark_hash
            );
        }
    } else {
        bail!("Not commit hash <new commit> ({})", new_commit)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_run_option_b() {}
}
