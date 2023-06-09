//! **hasher command** ... Base command: `git hash-object`
//!
//! Generate objects from actual files and directories.

// Std
use std::path::Path;

// External
use anyhow::Result;

// Internal
use crate::struct_set::{Hashable, Object};
use crate::repo::NssRepository;

/// Calculate the hash value of the given file and output this.
pub fn run<P: AsRef<Path>>(target_path: P) -> Result<()> {
    let object = Object::new(target_path.as_ref())?;
    println!("{}", hash_to_hex(object.to_hash()));

    Ok(())
}

/// Register the object into object database (repository) and output this.
pub fn run_option_w<P: AsRef<Path>>(target_path: P, repository: NssRepository) -> Result<()> {
    let object = Object::new(target_path)?;
    let hash = hash_to_hex(object.to_hash());

    repository.write_object(object)?;
    println!("{}", hash);

    Ok(())
}

fn hash_to_hex(hash: Vec<u8>) -> String {
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::ZlibDecoder;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use testdir::testdir;

    #[test]
    fn test_run() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {:?}", temp_dir);

        // Create a temporary file for testing
        let temp_file = temp_dir.join("first.rs");
        let buffer = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";

        let mut file = File::create(&temp_file).unwrap();
        file.write_all(buffer).unwrap();

        // Vertfiy blob hash with existed file
        let res = run(temp_dir.clone().join("first.rs"));
        assert!(res.is_ok());

        // Vertfiy blob hash with no existed file
        let res = run(temp_dir.clone().join("second.rs"));
        assert!(res.is_err());

        // Vertfiy tree hash with existed file
        let res = run(temp_dir.clone());
        assert!(res.is_ok());

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_run_option_w() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        let test_repo = NssRepository::new(temp_dir.clone());
        println!("Test Directory: {:?}", temp_dir);

        // Create a temporary nss directory for testing
        let nss_objects_dir = temp_dir.join(".nss").join("objects");
        fs::create_dir_all(&nss_objects_dir).unwrap();

        // Create a temporary file for blob testing
        let temp_file = temp_dir.join("first.rs");
        let buffer = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";

        let mut file = File::create(&temp_file).unwrap();
        file.write_all(buffer).unwrap();
        file.flush().unwrap();

        // Create a temporary file for tree testing
        fs::create_dir(&temp_dir.join("test_sub_dir")).unwrap();
        let temp_file = temp_dir.join("test_sub_dir").join("first.rs");
        let buffer = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";

        let mut file = File::create(&temp_file).unwrap();
        file.write_all(buffer).unwrap();
        file.flush().unwrap();

        // Vertfiy blob hash with existed file and output of objects
        let res = run_option_w(temp_dir.clone().join("first.rs"), test_repo.clone());
        assert!(res.is_ok());

        let test_object_path = test_repo.objects_path("5c73008ba75573c20d6a8a6e557d0556d4a84133");

        let mut file = File::open(test_object_path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let mut decoder = ZlibDecoder::new(&bytes[..]);
        let mut object_content: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut object_content).unwrap();

        let buffer_blob = b"blob 250\0#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";
        assert_eq!(object_content, buffer_blob);

        // Vertfiy blob hash with no existed file and no output of objects
        let res = run_option_w(temp_dir.clone().join("second.rs"), test_repo.clone());
        assert!(res.is_err());

        // Vertfiy tree hash with existed file
        let res = run_option_w(temp_dir.clone().join("test_sub_dir"), test_repo.clone());
        assert!(res.is_ok());

        let test_object_path = test_repo.objects_path("c192349d0ee530038e5d925fdd701652ca755ba8");

        let mut file = File::open(test_object_path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let mut decoder = ZlibDecoder::new(&bytes[..]);
        let mut object_content: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut object_content).unwrap();

        let buffer_tree =
            b"tree 35\033188 first.rs\0\\s\x00\x8b\xa7Us\xc2\rj\x8anU}\x05V\xd4\xa8A3";
        assert_eq!(object_content, buffer_tree);

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_hash_to_hex() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {:?}", temp_dir);

        // Create a temporary file for testing
        let temp_file = temp_dir.join("first.rs");
        let buffer = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";

        let mut file = File::create(&temp_file).unwrap();
        file.write_all(buffer).unwrap();

        // Vertify blob hash
        let object = Object::new(temp_file).unwrap();
        assert_eq!(
            hash_to_hex(object.to_hash()),
            "5c73008ba75573c20d6a8a6e557d0556d4a84133"
        );

        // Vertify tree hash
        let object = Object::new(temp_dir.clone()).unwrap();
        assert_eq!(
            hash_to_hex(object.to_hash()),
            "c192349d0ee530038e5d925fdd701652ca755ba8"
        );

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
