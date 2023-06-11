// Std
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// External
use anyhow::Result;
use serde::{Deserialize, Serialize};

// Internal
use super::Hashable;

/// **Blob Struct**
///
/// This struct represents a file object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Blob {
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
}

impl Blob {
    /// Create a raw object with hash value.
    ///
    /// This path must be in the working directory.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path.as_ref())?;
        let mut content: Vec<u8> = Vec::new();
        file.read_to_end(&mut content)?;

        Ok(Self { content })
    }

    /// Create Object with RawObject.
    pub fn from_rawobject(contnet: &[u8]) -> Result<Self> {
        Ok(Self {
            content: contnet.to_vec(),
        })
    }
}

impl std::fmt::Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.content.clone()).unwrap())
    }
}

impl Hashable for Blob {
    fn as_bytes(&self) -> Vec<u8> {
        // "blob filesize\0contnet" to bytes
        let header = format!("blob {}\0", self.content.len());
        let store = [header.as_bytes(), &self.content].concat();

        store
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_blob_new() {
        // Create a temporary directory for testing
        let temp_dir = env::temp_dir().join("nss_test_new_blob");
        println!("Test Directory: {}", temp_dir.display());
        fs::create_dir(&temp_dir).unwrap();

        // Create a temporary file for testing
        let file_path = temp_dir.join("test_file.txt");
        let buffer = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";

        let mut file = File::create(&file_path).unwrap();
        file.write_all(buffer).unwrap();

        // Create a Blob instance using the temporary file
        let blob = Blob::new(&file_path);

        assert!(blob.is_ok());

        // Verify the Blob instance's properties
        let blob = blob.unwrap();
        assert_eq!(blob.content, buffer);

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_blob_from_rawobject() {
        // Create a sample content as bytes
        let content = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";

        // Create a Blob instance from the raw object content
        let blob = Blob::from_rawobject(content).unwrap();

        // Verify the Blob instance's properties
        assert_eq!(blob.content, content.to_vec());
    }

    #[test]
    fn test_blob_as_bytes() {
        // Create a Blob instance
        let content = b"#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";
        let blob = Blob {
            content: content.to_vec(),
        };

        // Convert the Blob to bytes
        let bytes = blob.as_bytes();

        // Verify the converted bytes
        let expected_bytes = b"blob 250\0#[allow(dead_code)]
fn commit(message: &str) -> std::io::Result<()> {
    let tree_hash = write_tree()?;
    match commit_tree(&tree_hash, message)? {
        Some(c) => update_ref(&c)?,
        _ => println!(\"Nothing to commit\")
    };

    Ok(())
}";
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_blob_display() {
        // Create a Blob instance
        let blob = Blob {
            content: b"Hello, world!".to_vec(),
        };

        // Format the Blob for display
        let display = format!("{}", blob);

        // Verify the formatted display string
        assert_eq!(display, "Hello, world!");
    }
}
