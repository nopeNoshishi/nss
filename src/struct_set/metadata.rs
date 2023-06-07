// Std
use std::path::Path;

// External
use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};

// Internal
use super::{Blob, Hashable};
use crate::util::file_system;

/// TODO: Documentation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileMeta {
    pub ctime: u32,
    pub ctime_nsec: u32,
    pub mtime: u32,
    pub mtime_nsec: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub filesize: u32,
    pub hash: Vec<u8>,
    pub filename_size: u16,
    pub filename: String,
}

impl FileMeta {
    /// TODO: Documentation
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // NOTE: Only unix metadata
        use std::os::unix::prelude::MetadataExt;

        let path = path.as_ref();
        // Exstract metadata on file
        let metadata = path.metadata().unwrap();
        let ctime = metadata.ctime() as u32;
        let ctime_nsec = metadata.ctime_nsec() as u32;
        let mtime = metadata.mtime() as u32;
        let mtime_nsec = metadata.mtime_nsec() as u32;
        let dev = metadata.dev() as u32;
        let ino = metadata.ino() as u32;
        let mode = metadata.mode();
        let uid = metadata.uid();
        let gid = metadata.gid();
        let filesize = metadata.size() as u32;

        let object = Blob::new(path).unwrap();
        let hash = object.to_hash();

        // absolute path -> relative path (from repo path)
        let repo_path = file_system::exists_repo::<P>(None)?;
        let filename = path
            .strip_prefix(&repo_path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let filename_size = filename.len() as u16;

        Ok(Self {
            ctime,
            ctime_nsec,
            mtime,
            mtime_nsec,
            dev,
            ino,
            mode,
            uid,
            gid,
            filesize,
            hash,
            filename_size,
            filename,
        })
    }

    /// TODO: Documentation
    pub fn from_rawindex(buf: &[u8]) -> Self {
        let ctime = BigEndian::read_u32(&buf[0..4]);
        let ctime_nsec = BigEndian::read_u32(&buf[4..8]);
        let mtime = BigEndian::read_u32(&buf[8..12]);
        let mtime_nsec = BigEndian::read_u32(&buf[12..16]);
        let dev = BigEndian::read_u32(&buf[16..20]);
        let ino = BigEndian::read_u32(&buf[20..24]);
        let mode = BigEndian::read_u32(&buf[24..28]);
        let uid = BigEndian::read_u32(&buf[28..32]);
        let gid = BigEndian::read_u32(&buf[32..36]);
        let filesize = BigEndian::read_u32(&buf[36..40]);
        let hash = Vec::from(&buf[40..60]);
        let filename_size = BigEndian::read_u16(&buf[60..62]);
        let filename =
            String::from_utf8(Vec::from(&buf[62..(62 + (filename_size as usize))])).unwrap();

        Self {
            ctime,
            ctime_nsec,
            mtime,
            mtime_nsec,
            dev,
            ino,
            mode,
            uid,
            gid,
            filesize,
            hash,
            filename_size,
            filename,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let entry_meta = [
            self.ctime.to_be_bytes(),
            self.ctime_nsec.to_be_bytes(),
            self.mtime.to_be_bytes(),
            self.mtime_nsec.to_be_bytes(),
            self.dev.to_be_bytes(),
            self.ino.to_be_bytes(),
            self.mode.to_be_bytes(),
            self.uid.to_be_bytes(),
            self.gid.to_be_bytes(),
            self.filesize.to_be_bytes(),
        ]
        .concat();

        let filemeta_vec = [
            entry_meta,
            self.hash.clone(),
            Vec::from(self.filename_size.to_be_bytes()),
            self.filename.as_bytes().to_vec(),
        ]
        .concat();

        filemeta_vec
    }
}

impl PartialEq for FileMeta {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_filemeta_new() {}

    #[test]
    fn test_filemeta_from_rawindex() {}

    #[test]
    fn test_filemeta_partialeq() {}
}
