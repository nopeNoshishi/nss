// Std
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

// External
use anyhow::Result;

// Internal
use super::{Object, Hashable, Index, FileMeta};
use crate::util::gadget;

/// **Entry Struct**
/// 
/// This struct contains blob( or tree) object's mode, name, hash.
/// Since blob and tree do not know their own names, it is necessary 
/// to string them together in this structure.
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entry {
    pub mode: u32,
    pub name: String, // file or dir by pre_tree
    pub hash: Vec<u8>
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

        let name = path.as_ref().file_name().unwrap()
            .to_str().unwrap()
            .to_string();

        Ok(Self { mode, name, hash, })
    }

    /// Create Entry with RawObject.
    /// 
    /// **Note:** This related function is intended to be called through Tree sturuct.
    fn from_rawobject(meta: &[u8], hash: &[u8]) -> Result<Self> {
        // meta = b"<pre_file hash><this file mode> <this file relative path>"
        // hash_next = b"<this_file hash><next file mode> <next file relative path>"

        let meta =  String::from_utf8(meta.to_vec()).unwrap();
        let mode_name = meta.split_whitespace().collect::<Vec<&str>>();
    
        let mode = mode_name[0].parse::<u32>().unwrap();
        let name = mode_name[1];
    
        Ok(Self {
            mode: mode, 
            name: String::from(name), 
            hash: hash.to_vec(),
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let store = format!("{} {}\0{}", self.mode, self.name, hex::encode(&self.hash));

        Vec::from(store.as_bytes())
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

        write!(
            f,
            "{:0>6o} {} {}\t{}",
            self.mode,
            object_type,
            String::from_utf8(self.hash.to_vec()).unwrap(),
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

        Ok(Self { entries: entries })
    }

    /// Create Object with RawObject.
    pub fn from_rawobject(content: &[u8]) -> Result<Self> {
        let entries: Vec<Entry> = Vec::new();
        let mut contnets = content.split(|&b| b == b'\0');
        let mut header = contnets.next().unwrap();

        let mut entries = contnets.try_fold(entries, |mut acc, x| {
            let (hash, next_header) = x.split_at(40);
            let entry = Entry::from_rawobject(header, hash).unwrap();
    
            acc.push(entry);
            header = next_header;
            Some(acc)
        }).unwrap();

        entries.sort_by(|a, b| a.name.cmp(&b.name));
    
        Ok(Self { entries })
    }
}

impl TryFrom<Index> for Tree {
    type Error = anyhow::Error;

    fn try_from(index: Index) -> Result<Self> {
        let mut index_paths: Vec<PathBuf> = vec![];

        for filemeta in index.filemetas {
            let path = PathBuf::from(filemeta.filename);
            if path.to_str().unwrap().contains("/") {
                let mut iter = path.iter();
                index_paths.push(PathBuf::from(iter.next().unwrap()));
            } else {
                index_paths.push(path);
            }
        }
        let entry_paths: HashSet<PathBuf> = index_paths.into_iter().collect();

        let mut entries: Vec<Entry> = vec![];
        for entry_path in entry_paths {
            let repo_path = gadget::get_repo_path()?;
            entries.push(Entry::new(&repo_path.join(entry_path)).unwrap());
        }
        entries.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(Self {
            entries: entries
        })
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (&self.entries)
                .into_iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Hashable for Tree {
    fn as_bytes(&self) -> Vec<u8> {
        // "tree content_size\0entry\nentry\nentry\n..." to bytes
        let header = format!("tree {}\0", self.entries.len());
        let entries = self.entries
            .iter()
            .map(|x| x.as_bytes())
            .collect::<Vec<_>>();

        [Vec::from(header.as_bytes()), entries.concat()].concat()
    }
}
