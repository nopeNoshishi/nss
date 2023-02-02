// Std
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

// External
use anyhow::Result;

// Internal
use super::Hashable;

/// **Blob Struct**
/// 
/// This struct represents a file object.
#[derive(Debug, Clone)]
pub struct Blob {
    // file content as bytes
    pub content: Vec<u8>
}

impl Blob {
    /// Create a raw object with hash value.
    /// 
    /// This path must be in the working directory.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path.as_ref())?;
        let mut content: Vec<u8> = Vec::new();
        file.read_to_end(&mut content)?;
        
        Ok(Self {
            content: content
        })        
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

        Vec::from(store)
    }
}
