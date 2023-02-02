// Std
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;

// External
use anyhow::Result;
use byteorder::{ByteOrder, BigEndian};

// Internal
use crate::util::gadget;
use super::{Object, Hashable};

/// TODO: Documentation
#[derive(Debug, Clone)]
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
    pub fn new(path: &PathBuf) -> Result<Self> {
        // Exstract metadata on file
        let metadata = path.metadata().unwrap();
        let ctime = metadata.ctime() as u32;
        let ctime_nsec = metadata.ctime_nsec() as u32;
        let mtime = metadata.mtime() as u32;
        let mtime_nsec = metadata.mtime_nsec() as u32;
        let dev = metadata.dev() as u32;
        let ino = metadata.ino() as u32;
        let mode = metadata.mode() as u32;
        let uid = metadata.uid() as u32;
        let gid= metadata.gid() as u32;
        let filesize = metadata.size() as u32;

        let object = Object::new(path).unwrap();
        let hash = object.to_hash();

        let repo_path = gadget::get_repo_path()?;

        // absolute path -> relative path (from repo path)
        let filename = path.strip_prefix(&repo_path).unwrap()
            .to_str().unwrap()
            .to_string();
        let filename_size = filename.len() as u16;


        Ok(Self {
            ctime, ctime_nsec, mtime, mtime_nsec, dev, ino, mode, uid, gid, 
            filesize, hash, filename_size, filename
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
        let gid= BigEndian::read_u32(&buf[32..36]);
        let filesize = BigEndian::read_u32(&buf[36..40]);
        let hash = Vec::from(&buf[40..60]);
        let filename_size = BigEndian::read_u16(&buf[60..62]);
        let filename = String::from_utf8(Vec::from(&buf[62..(62+(filename_size as usize))])).unwrap();

        Self {
            ctime, ctime_nsec, mtime, mtime_nsec, dev, ino, mode, uid, gid, 
            filesize, hash, filename_size, filename
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let entry_meta = [self.ctime.to_be_bytes(), self.ctime_nsec.to_be_bytes(),
            self.mtime.to_be_bytes(), self.mtime_nsec.to_be_bytes(), self.dev.to_be_bytes(),
            self.ino.to_be_bytes(), self.mode.to_be_bytes(), self.uid.to_be_bytes(),
            self.gid.to_be_bytes(), self.filesize.to_be_bytes()].concat();

        let filemeta_vec = [entry_meta, self.hash.clone(), Vec::from(self.filename_size.to_be_bytes()),
            self.filename.as_bytes().to_vec()].concat();
        
        filemeta_vec
    }
}

/// TODO: Documentation
impl PartialEq for FileMeta {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
