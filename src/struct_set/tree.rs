// Std
use std::ffi::OsString;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

// External
use anyhow::Result;
use serde::{Deserialize, Serialize};

// Internal
use super::{FileMeta, Hashable, Object};

/// **Entry Struct**
///
/// This struct contains blob( or tree) object's mode, name, hash.
/// Since blob and tree do not know their own names, it is necessary
/// to string them together in this structure.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Entry {
    pub mode: u32,
    pub name: OsString,
    pub hash: Vec<u8>,
}

impl Entry {
    /// Create entry with the path.
    ///
    /// This path must be in the working directory.
    pub fn new<P: AsRef<Path>>(path: P, object: Object) -> Result<Self> {
        let metadata = path.as_ref().metadata()?;
        let mode = metadata.mode();

        let hash = object.to_hash();

        let name = path.as_ref().file_name().unwrap().to_os_string();

        Ok(Self { mode, name, hash })
    }

    pub fn new_group<P: AsRef<Path>>(path: P, entries: Vec<Entry>) -> Result<Self> {
        let metadata = path.as_ref().metadata()?;
        let mode = metadata.mode();

        let tree = Tree::from_entries(entries);
        let hash = tree.to_hash();

        let name = path.as_ref().file_name().unwrap().to_os_string();

        Ok(Self { mode, name, hash })
    }

    /// Create Entry with RawObject.
    ///
    /// **Note:** This related function is intended to be called through Tree sturuct.
    fn from_rawobject(meta: &[u8], hash: &[u8]) -> Result<Self> {
        // meta = b"<pre_file hash><this file mode> <this file relative path>"
        // hash_next = b"<this_file hash><next file mode> <next file relative path>"

        let meta = String::from_utf8(meta.to_vec()).unwrap();
        let mode_name = meta.split_whitespace().collect::<Vec<&str>>();

        Ok(Self {
            mode: mode_name[0].parse::<u32>().unwrap(),
            name: OsString::from(mode_name[1]),
            hash: hash.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("{} {}\0", self.mode, self.name.to_str().unwrap());

        [header.as_bytes(), &self.hash].concat()
    }
}

impl From<FileMeta> for Entry {
    /// Create Entry with FileMeta.
    ///
    /// **Note:** This from function is intended for use when converting
    /// an Index to a Tree.
    fn from(filemeta: FileMeta) -> Self {
        Entry {
            mode: filemeta.mode,
            name: filemeta.filename,
            hash: filemeta.hash,
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let object_type = match self.mode.to_be_bytes()[2] >> 4 {
            4 => "tree",
            8 => "blob",
            _ => "unknown",
        };

        write!(
            f,
            "{:0>6o} {} {}\t{}",
            self.mode,
            object_type,
            hex::encode(&self.hash),
            self.name.to_str().unwrap()
        )
    }
}

/// **Tree Struct**
///
/// This struct represents a directory object.
#[derive(Debug, Clone)]
pub struct Tree {
    pub entries: Vec<Entry>,
}

impl Tree {
    /// Create Tree with the path.
    ///
    /// This path must be in the working directory.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let read_dir = path.as_ref().read_dir()?;

        let ignores = vec![PathBuf::from(".git"), PathBuf::from(".nss")];

        let mut entries: Vec<Entry> = vec![];
        for dir_entry in read_dir {
            let path = dir_entry?.path();

            if ignores.iter().any(|p| p == path.file_name().unwrap()) {
                continue;
            }

            let object = Object::new(&path)?;
            let entry = Entry::new(path, object)?;

            entries.push(entry)
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Self { entries })
    }

    pub fn from_entries(entries: Vec<Entry>) -> Self {
        Self { entries }
    }

    /// Create Object with RawObject.
    pub fn from_rawobject(content: &[u8]) -> Result<Self> {
        let entries: Vec<Entry> = Vec::new();
        let mut contnets = content.splitn(2, |&b| b == b'\0');
        let mut header = contnets.next().unwrap();
        let split_content = split_content(contnets.next().unwrap());

        let mut entries = split_content
            .iter()
            .try_fold(entries, |mut acc, x| {
                let (hash, next_header) = x.split_at(20);
                let entry = Entry::from_rawobject(header, hash).unwrap();

                acc.push(entry);
                header = next_header;

                Some(acc)
            })
            .unwrap();

        entries.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Self { entries })
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (self.entries)
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Hashable for Tree {
    fn as_bytes(&self) -> Vec<u8> {
        // "tree content_size\0entry\nentry\nentry\n..." to bytes
        let entries = self
            .entries
            .iter()
            .map(|x| x.as_bytes())
            .collect::<Vec<_>>();

        let content = entries.concat();
        let header = format!("tree {}\0", content.len());

        [Vec::from(header.as_bytes()), content].concat()
    }
}

fn split_content(contents: &[u8]) -> Vec<&[u8]> {
    let mut result: Vec<&[u8]> = Vec::new();
    let mut index = 0;

    while let Some(b_index) = &contents[index + 20..]
        .iter()
        .position(|&byte| byte == b'\0')
    {
        let split_index = index + 20 + b_index;

        result.push(&contents[index..split_index]);
        index = split_index + 1;

        if index + 20 > contents.len() {
            break;
        }
    }

    result.push(&contents[index..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use testdir::testdir;

    #[test]
    fn test_entry_new() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {:?}", temp_dir);

        // Create a temporary file for testing
        let temp_file = temp_dir.join("first.rs");
        let test_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("tests")
            .join("test_repo")
            .join("first.rs");
        fs::copy(&test_file, &temp_file).unwrap();

        // Vertify existed file
        let object = Object::new(&temp_file).unwrap();
        let result = Entry::new(&temp_file, object);
        assert!(result.is_ok());

        let entry = result.unwrap();
        assert_eq!(entry.mode, 0o100644);
        assert_eq!(entry.name, OsString::from("first.rs"));
        assert_eq!(
            hex::encode(entry.hash),
            "5c73008ba75573c20d6a8a6e557d0556d4a84133"
        );

        // Vertify existed folder
        let object = Object::new(&temp_dir).unwrap();
        let result = Entry::new(&temp_dir, object);
        assert!(result.is_ok());

        let entry = result.unwrap();
        assert_eq!(entry.mode, 0o040755);
        assert_eq!(entry.name, OsString::from("test_entry_new"));
        assert_eq!(
            hex::encode(entry.hash),
            "c192349d0ee530038e5d925fdd701652ca755ba8"
        );
    }

    // TODO: Test
    #[test]
    fn test_entry_group_new() {
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

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_from_rawobject() {
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

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_entry_from_filemata() {
        // Create a temporary directory for testing
        let temp_dir = testdir!();
        println!("Test Directory: {}", temp_dir.display());

        // meta = b"<pre_file hash><this file mode> <this file relative path>"
        // hash_next = b"<this_file hash><next file mode> <next file relative path>"

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_entry_as_bytes() {
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

        // Vertify
        let object = Object::new(&temp_file).unwrap();
        let entry = Entry::new(&temp_file, object).unwrap();
        assert_eq!(
            entry.as_bytes(),
            b"33188 first.rs\0\\s\x00\x8b\xa7Us\xc2\rj\x8anU}\x05V\xd4\xa8A3"
        );

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_entry_display() {
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

        // Vertify display
        let object = Object::new(&temp_file).unwrap();
        let entry = Entry::new(&temp_file, object).unwrap();
        assert_eq!(
            format!("{}", entry),
            "100644 blob 5c73008ba75573c20d6a8a6e557d0556d4a84133\tfirst.rs"
        );

        // Clean up: Remove the test dir
        fs::remove_dir_all(temp_dir).unwrap();
    }

    // #[test]
    // fn test_commit_new() {
    //     let content = fs::read_to_string(".nssignore").unwrap();
    //     println!("{:?}", content);
    //     let lines = content.lines();

    //     let repo_path = PathBuf::from("User/noshishi/work");

    //     for l in lines
    //         .into_iter()
    //         .filter(|line| !line.contains('#') || line.is_empty())
    //         .map(|line| repo_path.join(line))
    //     {
    //         println!("{:?}", l);
    //     }
    // }

    #[test]
    fn test_commit_from_rawobject() {
        let (a, b) = b"\\s\x00\x8b\xa7Us\xc2\rj\x8anU}\x05V\xd4\xa8A3".split_at(20);
        println!("{:?} : {:?}", a, b);
    }

    #[test]
    fn test_commit_as_bytes() {}

    #[test]
    fn test_commit_display() {}
}
