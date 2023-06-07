// Std
use std::os::unix::fs::MetadataExt;
use std::path::Path;

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
    pub name: String, // file or dir by pre_tree
    pub hash: Vec<u8>,
}

impl Entry {
    /// Create entry with the path.
    ///
    /// This path must be in the working directory.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let metadata = path.as_ref().metadata()?;
        let mode = metadata.mode();

        let object = Object::new(path.as_ref())?;
        let hash = object.to_hash();

        let name = path
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Ok(Self { mode, name, hash })
    }

    pub fn new_group<P: AsRef<Path>>(path: P, entries: Vec<Entry>) -> Result<Self> {
        let metadata = path.as_ref().metadata()?;
        let mode = metadata.mode();

        let tree = Tree::from_entries(entries);
        let hash = tree.to_hash();

        let name = path
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

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

        let mode = mode_name[0].parse::<u32>().unwrap();
        let name = mode_name[1];

        Ok(Self {
            mode,
            name: String::from(name),
            hash: hash.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header = format!("{} {}\0", self.mode, self.name);

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
        // TODO! Implement to display mode caluculater
        // if dir metadate.mode() -> 0100 0001 1110 1101
        // not 0100 0000 0000 0000 (040000): Directory
        //
        // 32-bit mode, split into (high to low bits)
        // high
        // - 16-bit
        //      0000 0000 0000 0000
        //
        // low
        // - 4-bit object type
        //     valid values in binary are 1000 (regular file), 1010 (symbolic link)
        //     and 1110 (gitlink)
        // - 3-bit unused
        // - 9-bit unix permission. Only 0755 and 0644 are valid for regular files.
        //     Symbolic links and gitlinks have value 0 in this field.
        //
        // 0100 0000 0000 0000 (040000): Directory
        // 1000 0001 1010 0100 (100644): Regular non-executable file
        // 1000 0001 1011 0100 (100664): Regular non-executable group-writeable file
        // 1000 0001 1110 1101 (100755): Regular executable file
        // 1010 0000 0000 0000 (120000): Symbolic link
        //
        // [Stack overflow How to read the mode field of git-ls-tree's output](https://stackoverflow.com/questions/737673/how-to-read-the-mode-field-of-git-ls-trees-output)
        //

        let object_type = {
            if self.mode == 0o100644 {
                "blob"
            } else {
                "tree"
            }
        };

        let mode = match object_type {
            "tree" => 0o40000,
            _ => self.mode,
        };

        write!(
            f,
            "{:0>6o} {} {}\t{}",
            mode,
            object_type,
            hex::encode(&self.hash),
            self.name
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

        let mut entries: Vec<Entry> = vec![];
        for dir_entry in read_dir {
            let path = dir_entry?.path();
            let entry = Entry::new(path)?;

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
        let mut contnets = content.split(|&b| b == b'\0');
        let mut header = contnets.next().unwrap();

        let mut entries = contnets
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

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_commit_new() {}

    #[test]
    fn test_commit_from_rawobject() {}

    #[test]
    fn test_commit_as_bytes() {}

    #[test]
    fn test_commit_display() {}
}
