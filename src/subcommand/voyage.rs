//! **init command** ... Base command: `git init`
//!
//! Create current directory as a new repository.

// Std
use std::path::Path;

// External
use anyhow::{bail, Result};

// Internal
use crate::nss_io::file_system::*;
use crate::repo::{User, Config};

/// Build the necessary repository directories.
///
/// **Note:** Basically, the repository is managed through Absolute Pas.
pub fn run<P: AsRef<Path>>(repo_path: P) -> Result<()> {
    let repo_path = repo_path.as_ref();

    // Initial Directory
    create_dir(repo_path.join(".nss"))?;
    create_dir(repo_path.join(".nss").join("bookmarks"))?;
    create_dir(repo_path.join(".nss").join("objects"))?;
    create_dir(repo_path.join(".nss").join("bookmarks").join("local"))?;
    create_dir(repo_path.join(".nss").join("memo"))?;

    // Initial File
    // TODO: Consider what to do when some of the folders in the repository are missing.
    match create_file_with_buffer(
        repo_path.join(".nss").join("repo"),
        repo_path.to_str().unwrap().as_bytes(),
    ) {
        Ok(..) => (),
        Err(..) => bail!("Repository already existed!"),
    };
    create_file_with_buffer(
        repo_path.join(".nss").join("HEAD"),
        b"bookmarker: bookmarks/local/voyage",
    )?;

    let config = Config::new(User::new(whoami::username(), None));
    create_file_with_buffer(repo_path.join(".nss").join("config"), toml::to_string(&config)?.as_bytes())?;
    create_file_with_buffer(repo_path.join(".nss").join("INDEX"), b"")?;
    create_file_with_buffer(
        repo_path
            .join(".nss")
            .join("bookmarks")
            .join("local")
            .join("voyage"),
        b"",
    )?;

    let repo_name = repo_path.file_name().unwrap();
    println!(
        "Created repository! Repository name: {}",
        repo_name.to_str().unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use testdir::testdir;

    #[test]
    fn test_run() {
        // Create a temporary directory for testing
        let temp_dir = testdir! {};
        println!("Test Directory: {}", temp_dir.display());

        // Run the function test
        assert!(run(&temp_dir).is_ok());

        // Verify that the expected files and directories are created
        assert!(temp_dir.join(".nss").is_dir());
        assert!(temp_dir.join(".nss").join("bookmarks").is_dir());
        assert!(temp_dir.join(".nss").join("objects").is_dir());
        assert!(temp_dir
            .join(".nss")
            .join("bookmarks")
            .join("local")
            .is_dir());
        assert!(temp_dir.join(".nss").join("memo").is_dir());

        assert!(temp_dir.join(".nss").join("repo").is_file());
        assert!(temp_dir.join(".nss").join("HEAD").is_file());
        assert!(temp_dir.join(".nss").join("config").is_file());
        assert!(temp_dir.join(".nss").join("INDEX").is_file());
        assert!(temp_dir
            .join(".nss")
            .join("bookmarks")
            .join("local")
            .join("voyage")
            .is_file());

        // Already existed repository
        assert!(run(&temp_dir).is_err());

        // Clean up: Remove the temporary directory
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
