// Std
use std::path::Path;

// External
use anyhow::Result;
use sha1::{Digest, Sha1};

// Internal
use super::{Blob, Commit, Tree};

/// **Object Enum**
///
/// This enum connect to all object.
#[derive(Debug, Clone)]
pub enum Object {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

impl Object {
    /// Create object with the path.
    ///
    /// This path must be in the working directory.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        match path.as_ref().is_file() {
            true => Blob::new(path.as_ref()).map(Object::Blob),
            false => Tree::new(path.as_ref()).map(Object::Tree),
        }
    }

    pub fn from_content(raw_content: Vec<u8>) -> Result<Self> {
        let mut iter = raw_content.splitn(2, |&x| x == b'\0');

        // header ≒ b"<object-type> <contnet-size>"
        let header = iter.next().unwrap().to_vec();
        let header = String::from_utf8(header)?;
        let object_type = header.split(' ').collect::<Vec<&str>>()[0];

        // content ≒ b"<contnet>"
        let content = iter.next().unwrap();
        match object_type {
            "blob" => Blob::from_rawobject(content).map(Object::Blob),
            "tree" => Tree::from_rawobject(content).map(Object::Tree),
            "commit" => Commit::from_rawobject(content).map(Object::Commit),
            _ => todo!(),
        }
    }

    /// To tarnsform object name.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blob(_blob) => "blob",
            Self::Tree(_tree) => "tree",
            Self::Commit(_commit) => "commit",
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Blob(blob) => blob.fmt(f),
            Self::Tree(tree) => tree.fmt(f),
            Self::Commit(commit) => commit.fmt(f),
        }
    }
}

impl Hashable for Object {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Blob(blob) => blob.as_bytes(),
            Self::Tree(tree) => tree.as_bytes(),
            Self::Commit(commit) => commit.as_bytes(),
        }
    }

    fn to_hash(&self) -> Vec<u8> {
        match self {
            Self::Blob(blob) => blob.to_hash(),
            Self::Tree(tree) => tree.to_hash(),
            Self::Commit(commit) => commit.to_hash(),
        }
    }
}
pub trait Hashable {
    /// Content to bytes for calclating hash.
    fn as_bytes(&self) -> Vec<u8>;

    /// Content to hash by sha1 hash function.
    fn to_hash(&self) -> Vec<u8> {
        Vec::from(Sha1::digest(&self.as_bytes()).as_slice())
    }
}
