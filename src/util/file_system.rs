// TODO: 読み書きが発生する部分はここにまとめる

// Std
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// External
use anyhow::{bail, Context, Result};
use dirs::home_dir;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

// Internal
use super::gadget;
use crate::struct_set::{Blob, Commit, Hashable, Tree};

pub fn create_file_with_buffer<P: AsRef<Path>>(file_path: P, buffer: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path)?;

    file.write_all(buffer)?;

    Ok(())
}

pub fn create_dir<P: AsRef<Path>>(dir_path: P) -> std::io::Result<()> {
    fs::create_dir_all(dir_path)?;

    Ok(())
}

pub fn exists_repo<P: AsRef<Path>>(repo_dir: Option<P>) -> Result<PathBuf> {
    let current_dir = match repo_dir {
        Some(p) => {
            if p.as_ref().to_path_buf() == home_dir().unwrap() {
                bail!("not a nss repository (or any of the parent directories): .nss")
            } else {
                p.as_ref().to_path_buf()
            }
        }
        _ => std::env::current_dir()?,
    };

    let repo_path = current_dir.join(PathBuf::from(".nss"));
    let read_dir = fs::read_dir(&current_dir)?;

    for entry in read_dir {
        match entry?.path() == repo_path {
            true => return Ok(current_dir),
            false => continue,
        }
    }

    return exists_repo(Some(current_dir.parent().unwrap().to_path_buf()));
}

pub fn write_blob<S: AsRef<str>>(hash: S, object: Blob) -> Result<()> {
    let object_path = gadget::get_new_objects_path(hash.as_ref())?;
    create_dir(object_path.parent().unwrap())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&object.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush().unwrap();

    Ok(())
}

pub fn write_tree<S: AsRef<str>>(hash: S, tree: Tree) -> Result<()> {
    let object_path = gadget::get_new_objects_path(hash.as_ref())?;
    create_dir(object_path.parent().unwrap())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&tree.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush().unwrap();

    Ok(())
}

pub fn write_commit<S: AsRef<str>>(hash: S, object: Commit) -> Result<()> {
    let object_path = gadget::get_new_objects_path(hash.as_ref())?;
    create_dir(object_path.parent().unwrap())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&object.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush().unwrap();

    Ok(())
}

pub fn read_object<S: AsRef<str>>(hash: S) -> Result<Vec<u8>> {
    let hash_path = gadget::get_objects_path(hash.as_ref())?;

    // read objectz
    let mut file = File::open(hash_path)
        .with_context(|| format!("{} doesn't exit in this repository", hash.as_ref()))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .with_context(|| format!("{} content can't read", &hash.as_ref()))?;

    // decode content
    let mut decoder = ZlibDecoder::new(&bytes[..]);
    let mut object_content: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut object_content)?;

    Ok(object_content)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use testdir::testdir;

    #[test]
    fn test_create_file_with_buffer() {
        // Create a temporary directory for testing
        let temp_dir = testdir!{};
        println!("Test Directory: {:?}", temp_dir);

        // Target test file and buffer
        let file_path = temp_dir.join("test_file.txt");
        let buffer = b"Hello, world!";

        // Run the function under test
        assert!(create_file_with_buffer(&file_path, buffer).is_ok());

        // Verify that the file is created and its contents match the buffer
        let mut file = fs::File::open(&file_path).unwrap();
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents).unwrap();

        assert_eq!(file_contents, buffer);

        // Already existed file
        assert!(create_file_with_buffer(&file_path, buffer).is_err());

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_create_dir() {
        // Create a temporary directory for testing
        let temp_dir = testdir!{};
        println!("Test Directory: {:?}", temp_dir);

        // Target test folder
        let dir_path = temp_dir.join("test_dir").join("test_sub_dir");

        // Run the function under test
        assert!(create_dir(&dir_path).is_ok());

        // Verify that the expected directory are created
        assert!(temp_dir.join("test_dir").is_dir());
        assert!(temp_dir.join("test_dir").join("test_sub_dir").is_dir());

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_exists_repo() {
        // Create a temporary directory for testing
        let temp_dir = testdir!{};
        println!("Test Directory: {:?}", temp_dir);

        // Create the .nss directory inside the temporary directory
        let nss_dir = temp_dir.join(".nss");
        fs::create_dir(&nss_dir).unwrap();

        // Run the function under test with the repo_dir argument as PathBuf
        let result = exists_repo(Some(temp_dir.clone()));

        // Verify that the function returns the correct result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir);

        // Run the function under test with the repo_dir argument as &Path
        let result = exists_repo(Some(temp_dir.as_path()));

        // Verify that the function returns the correct result
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir);

        // Run the function under test without the repo_dir argument
        let result = exists_repo::<PathBuf>(None);

        // Verify that the function returns the correct result
        assert!(result.is_err());

        // Clean up: Remove the temporary directory
        fs::remove_dir_all(temp_dir).unwrap();
    }
}