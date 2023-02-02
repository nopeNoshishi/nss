//! **init command** ... Base command: `git init`
//! 
//! Create current directory as a new repository.

// Std
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

// External
use anyhow::Result;

// Internal
use crate::util::gadget;

/// Build the necessary repository directories.
/// 
/// **Note:** Basically, the repository is managed through Absolute Pas.
pub fn run() -> Result<()> {
    let nss_dirs = [".nss", ".nss/bookmarks", ".nss/objects",
                               ".nss/bookmarks/local", ".nss/memo"];
    
    for dir_path in nss_dirs {
        gadget::create_dir(&PathBuf::from(dir_path))?
    }

    // Resister repository directory path
    let nss_path = ".nss/repo";
    let mut file = File::create(nss_path)?;
    let repo_location = env::current_dir()?;
    file.write_all(repo_location.to_str().unwrap().as_bytes())?;

    // Identify author's working commit
    let head_path = ".nss/HEAD";
    let mut file = File::create(head_path)?;
    file.write_all(b"bookmarker: bookmarks/local/voyage")?;

    // Manage user's profile
    let config_path = ".nss/config";
    let mut file = File::create(config_path)?;
    file.write_all(b"remotes: []")?;

    // Index (Stagin area) cashe
    let index_path = ".nss/INDEX";
    File::create(index_path)?;

    // Create first main bookmarker
    let first_bookmark_path = ".nss/bookmarks/local/voyage";
    File::create(first_bookmark_path)?;

    let repo_path = gadget::get_repo_path()?;
    let repo_name = repo_path.file_name().unwrap();
    println!("Created repository! Repository name: {:?}", repo_name);

    Ok(())
}
