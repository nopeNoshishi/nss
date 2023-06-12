//! **ocat command** ... Base command: `git cat-file`
//!
//! Retrieve objects in the database and output their
//! contents to standard output.

// Std
use std::io::Write;

// External
use anyhow::Result;

// Internal
use nss_core::repository::NssRepository;

/// Register the object into object database (repository)
/// and Display on standart-output.
#[allow(unused_must_use)]
pub fn run_option_p<W: Write>(w: &mut W, repository: &NssRepository, hash: &str) -> Result<()> {
    let object = repository.read_object(hash)?;
    writeln!(w, "{}", object);

    Ok(())
}

/// Output the object type
#[allow(unused_must_use)]
pub fn run_option_t<W: Write>(w: &mut W, repository: &NssRepository, hash: &str) -> Result<()> {
    let object = repository.read_object(hash)?;
    writeln!(w, "{}", object.as_str());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use testdir::testdir;

    #[test]
    fn test_run_option_p() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {}", temp_dir.display());

        // Create a temporary nss directory for testing
        let nss_objects_dir = temp_dir.join(".nss").join("objects").join("5c");
        fs::create_dir_all(&nss_objects_dir).unwrap();
        let nss_objects_dir = temp_dir.join(".nss").join("objects").join("c1");
        fs::create_dir_all(&nss_objects_dir).unwrap();
        let nss_objects_dir = temp_dir.join(".nss").join("objects").join("a0");
        fs::create_dir_all(&nss_objects_dir).unwrap();

        // Create a temporary object for testing
        let test_repo = NssRepository::new(temp_dir.clone());
        let test_object_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("test_object");

        // test blob
        let test_object = test_object_root
            .join("blob")
            .join("5c73008ba75573c20d6a8a6e557d0556d4a84133");
        fs::copy(
            test_object,
            test_repo.objects_path("5c73008ba75573c20d6a8a6e557d0556d4a84133"),
        )
        .unwrap();

        // test tree
        let test_object = test_object_root
            .join("tree")
            .join("c192349d0ee530038e5d925fdd701652ca755ba8");
        fs::copy(
            test_object,
            test_repo.objects_path("c192349d0ee530038e5d925fdd701652ca755ba8"),
        )
        .unwrap();

        // test commit
        let test_object = test_object_root
            .join("commit")
            .join("a02b83cb54ba139e5c9d623a2fcf5424552946e0");
        fs::copy(
            test_object,
            test_repo.objects_path("a02b83cb54ba139e5c9d623a2fcf5424552946e0"),
        )
        .unwrap();

        // Vertfiy blob object
        let blob_output = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}\n";
        let mut buf = Vec::<u8>::new();
        assert!(run_option_p(
            &mut buf,
            &test_repo,
            "5c73008ba75573c20d6a8a6e557d0556d4a84133"
        )
        .is_ok());
        assert_eq!(buf, blob_output);

        // Vertfiy no blob object
        let mut buf = Vec::<u8>::new();
        assert!(run_option_p(
            &mut buf,
            &test_repo,
            "aggregrq438252345d9292nt2nco24t2nfc94323"
        )
        .is_err());

        // Vertfiy tree object
        let tree_output = b"100644 blob 5c73008ba75573c20d6a8a6e557d0556d4a84133\tfirst.rs\n";
        let mut buf = Vec::<u8>::new();
        assert!(run_option_p(
            &mut buf,
            &test_repo,
            "c192349d0ee530038e5d925fdd701652ca755ba8"
        )
        .is_ok());
        assert_eq!(buf, tree_output);

        // Vertfiy commit object
        let commit_output = b"tree c192349d0ee530038e5d925fdd701652ca755ba8
parent None
author noshishi\0
committer noshishi\0
date 1686487974

initial
\n";
        let mut buf = Vec::<u8>::new();
        assert!(run_option_p(
            &mut buf,
            &test_repo,
            "a02b83cb54ba139e5c9d623a2fcf5424552946e0"
        )
        .is_ok());
        assert_eq!(buf, commit_output);

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_run_option_t() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {}", temp_dir.display());

        // Create a temporary nss directory for testing
        let nss_objects_dir = temp_dir.join(".nss").join("objects").join("5c");
        fs::create_dir_all(&nss_objects_dir).unwrap();
        let nss_objects_dir = temp_dir.join(".nss").join("objects").join("c1");
        fs::create_dir_all(&nss_objects_dir).unwrap();
        let nss_objects_dir = temp_dir.join(".nss").join("objects").join("a0");
        fs::create_dir_all(&nss_objects_dir).unwrap();

        // Create a temporary object for testing
        let test_repo = NssRepository::new(temp_dir.clone());
        let test_object_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("test_object");

        // test blob
        let test_object = test_object_root
            .join("blob")
            .join("5c73008ba75573c20d6a8a6e557d0556d4a84133");
        fs::copy(
            test_object,
            test_repo.objects_path("5c73008ba75573c20d6a8a6e557d0556d4a84133"),
        )
        .unwrap();

        // test tree
        let test_object = test_object_root
            .join("tree")
            .join("c192349d0ee530038e5d925fdd701652ca755ba8");
        fs::copy(
            test_object,
            test_repo.objects_path("c192349d0ee530038e5d925fdd701652ca755ba8"),
        )
        .unwrap();

        // test commit
        let test_object = test_object_root
            .join("commit")
            .join("a02b83cb54ba139e5c9d623a2fcf5424552946e0");
        fs::copy(
            test_object,
            test_repo.objects_path("a02b83cb54ba139e5c9d623a2fcf5424552946e0"),
        )
        .unwrap();

        // Vertfiy blob object
        let mut buf = Vec::<u8>::new();
        assert!(run_option_t(
            &mut buf,
            &test_repo,
            "5c73008ba75573c20d6a8a6e557d0556d4a84133"
        )
        .is_ok());
        assert_eq!(buf, "blob\n".as_bytes());

        // Vertfiy no blob object
        let mut buf = Vec::<u8>::new();
        assert!(run_option_p(
            &mut buf,
            &test_repo,
            "aggregrq438252345d9292nt2nco24t2nfc94323"
        )
        .is_err());

        // Vertfiy tree object
        let mut buf = Vec::<u8>::new();
        assert!(run_option_t(
            &mut buf,
            &test_repo,
            "c192349d0ee530038e5d925fdd701652ca755ba8"
        )
        .is_ok());
        assert_eq!(buf, "tree\n".as_bytes());

        // Vertfiy commit object
        let mut buf = Vec::<u8>::new();
        assert!(run_option_t(
            &mut buf,
            &test_repo,
            "a02b83cb54ba139e5c9d623a2fcf5424552946e0"
        )
        .is_ok());
        assert_eq!(buf, "commit\n".as_bytes());

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }
}
