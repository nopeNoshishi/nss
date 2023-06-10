//! **hasher command** ... Base command: `git hash-object`
//!
//! Generate objects from actual files and directories.

// Std
use std::io::Write;
use std::path::Path;

// External
use anyhow::Result;

// Internal
use crate::repo::NssRepository;
use crate::struct_set::{Hashable, Object};

/// Calculate the hash value of the given file and output this.
#[allow(unused_must_use)]
pub fn run<P: AsRef<Path>, W: Write>(w: &mut W, target_path: P) -> Result<()> {
    let object = Object::new(target_path.as_ref())?;
    writeln!(w, "{}", hex::encode(object.to_hash()));

    Ok(())
}

/// Register the object into object database (repository) and output this.
#[allow(unused_must_use)]
pub fn run_option_w<P: AsRef<Path>, W: Write>(
    w: &mut W,
    target_path: P,
    repository: NssRepository,
) -> Result<()> {
    let object = Object::new(target_path)?;
    let hash = hex::encode(object.to_hash());

    repository.write_object(object)?;
    writeln!(w, "{}", hash);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::ZlibDecoder;
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::PathBuf;
    use testdir::testdir;

    fn help_decode(test_object_path: PathBuf) -> Result<Vec<u8>> {
        println!("{}", test_object_path.display());
        let mut file = File::open(test_object_path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let mut decoder = ZlibDecoder::new(&bytes[..]);
        let mut object_content: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut object_content).unwrap();

        Ok(object_content)
    }

    #[test]
    fn test_run() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {}", temp_dir.display());

        // Create a temporary file for testing
        let temp_file = temp_dir.join("first.rs");
        let test_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("test_repo")
            .join("first.rs");
        fs::copy(&test_file, &temp_file).unwrap();

        // Vertfiy blob hash with existed file
        let mut buf = Vec::<u8>::new();
        let res = run(&mut buf, &temp_file);
        assert!(res.is_ok());
        assert_eq!(buf, b"5c73008ba75573c20d6a8a6e557d0556d4a84133\n");

        // Vertfiy blob hash with no existed file
        let mut buf = Vec::<u8>::new();
        let res = run(&mut buf, &temp_dir.join("second.rs"));
        assert!(res.is_err());

        // Vertfiy tree hash with existed file
        let mut buf = Vec::<u8>::new();
        let res = run(&mut buf, &temp_dir);
        assert!(res.is_ok());
        assert_eq!(buf, b"c192349d0ee530038e5d925fdd701652ca755ba8\n");

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_run_option_w() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {}", temp_dir.display());

        // Create a temporary file for testing
        let temp_file = temp_dir.join("first.rs");
        let test_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("test_repo")
            .join("first.rs");
        fs::copy(&test_file, &temp_file).unwrap();

        // Create a temporary nss directory for testing
        let test_repo = NssRepository::new(temp_dir.clone());
        let nss_objects_dir = temp_dir.join(".nss").join("objects");
        fs::create_dir_all(&nss_objects_dir).unwrap();

        // Vertfiy blob hash with existed file and output of objects
        let mut buf = Vec::<u8>::new();
        let res = run_option_w(&mut buf, &temp_file, test_repo.clone());
        assert!(res.is_ok());
        assert_eq!(buf, b"5c73008ba75573c20d6a8a6e557d0556d4a84133\n");

        let test_object_path = test_repo.objects_path("5c73008ba75573c20d6a8a6e557d0556d4a84133");

        let buffer_blob = b"blob 250\0#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";
        assert_eq!(help_decode(test_object_path).unwrap(), buffer_blob);

        // Vertfiy blob hash with no existed file and no output of objects
        let mut buf = Vec::<u8>::new();
        let res = run_option_w(&mut buf, &temp_dir.join("second.rs"), test_repo.clone());
        assert!(res.is_err());

        // Vertfiy tree hash with existed file
        let mut buf = Vec::<u8>::new();
        let res = run_option_w(&mut buf, &temp_dir, test_repo.clone());
        assert!(res.is_ok());
        assert_eq!(buf, b"c192349d0ee530038e5d925fdd701652ca755ba8\n");

        let test_object_path = test_repo.objects_path("c192349d0ee530038e5d925fdd701652ca755ba8");

        let buffer_tree =
            b"tree 35\033188 first.rs\0\\s\x00\x8b\xa7Us\xc2\rj\x8anU}\x05V\xd4\xa8A3";
        assert_eq!(help_decode(test_object_path).unwrap(), buffer_tree);

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
