// Std
use std::os::unix::prelude::MetadataExt;
use std::path::PathBuf;

// External
use anyhow::Result;
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};

// Internal
use super::{Blob, Hashable};
use crate::util::gadget;

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
    pub fn new(path: &PathBuf) -> Result<Self> {
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

        let repo_path = gadget::get_repo_path()?;

        // absolute path -> relative path (from repo path)
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

/// TODO: Documentation
impl PartialEq for FileMeta {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::PathBuf;

    // #[test]
    // fn test_meta() {
    //     let metadata = PathBuf::from("/Users/noshishi/study/nss/test/first.txt").metadata().unwrap();

    //     let ctime = metadata.ctime() as u32;
    //     let ctime_nsec = metadata.ctime_nsec() as u32;
    //     let mtime = metadata.mtime() as u32;
    //     let mtime_nsec = metadata.mtime_nsec() as u32;
    //     let dev: u32 = metadata.dev() as u32;
    //     let ino = metadata.ino() as u32;
    //     let mode = metadata.mode();
    //     let uid = metadata.uid();
    //     let gid = metadata.gid();
    //     let filesize = metadata.size() as u32;

    //     println!("{:?}", ctime.to_be_bytes());
    //     println!("{:?}", ctime_nsec.to_be_bytes());
    //     println!("{:?}", mtime.to_be_bytes());
    //     println!("{:?}", mtime_nsec.to_be_bytes());
    //     println!("{:?}", dev.to_be_bytes());
    //     println!("{:?}", ino.to_be_bytes());
    //     println!("{:?}", mode.to_be_bytes());
    //     println!("{:0o}", mode);
    //     println!("{:?}", uid.to_be_bytes());
    //     println!("{:?}", gid.to_be_bytes());
    //     println!("{:?}", filesize.to_be_bytes());
    // }
    // fn test_file_meta_new() {
    //     // Create a temporary file
    //     let mut file = NamedTempFile::new().unwrap();
    //     writeln!(file, "Hello, world!").unwrap();

    //     // Create a FileMeta instance using the temporary file
    //     let file_meta = FileMeta::new(file.path()).unwrap();

    //     // Verify the FileMeta instance's properties
    //     assert_eq!(file_meta.mode, 0o100644);
    //     assert_eq!(file_meta.filename, file.path().to_str().unwrap());
    //     assert_eq!(file_meta.hash.len(), 20); // Assuming SHA-1 hash size

    //     // Clean up: The temporary file will be deleted automatically
    // }

    // #[test]
    // fn test_file_meta_from_rawindex() {
    //     // Create a sample raw index buffer
    //     let buf = [
    //         0, 0, 0, 1,  // ctime
    //         0, 0, 0, 2,  // ctime_nsec
    //         0, 0, 0, 3,  // mtime
    //         0, 0, 0, 4,  // mtime_nsec
    //         0, 0, 0, 5,  // dev
    //         0, 0, 0, 6,  // ino
    //         0, 0, 0, 7,  // mode
    //         0, 0, 0, 8,  // uid
    //         0, 0, 0, 9,  // gid
    //         0, 0, 0, 10, // filesize
    //         1, 2, 3, 4, 5, 6, 7, 8, 9, 10, // hash
    //         0, 1, // filename_size
    //         b'f', b'i', b'l', b'e', b'.', b't', b'x', b't', // filename
    //     ];

    //     // Create a FileMeta instance from the raw index buffer
    //     let file_meta = FileMeta::from_rawindex(&buf);

    //     // Verify the FileMeta instance's properties
    //     assert_eq!(file_meta.mode, 7);
    //     assert_eq!(file_meta.filename, "file.txt");
    //     assert_eq!(file_meta.hash, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    // }

    // #[test]
    // fn test_file_meta_as_bytes() {
    //     // Create a FileMeta instance
    //     let file_meta = FileMeta {
    //         ctime: 1,
    //         ctime_nsec: 2,
    //         mtime: 3,
    //         mtime_nsec: 4,
    //         dev: 5,
    //         ino: 6,
    //         mode: 7,
    //         uid: 8,
    //         gid: 9,
    //         filesize: 10,
    //         hash: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    //         filename_size: 1,
    //         filename: "file.txt".to_string(),
    //     };

    //     // Convert the FileMeta to bytes
    //     let bytes = file_meta.as_bytes();

    //     // Verify the converted bytes
    //     let expected_bytes = [
    //         0, 0, 0, 1,  // ctime
    //         0, 0, 0, 2,  // ctime_nsec
    //         0, 0, 0, 3,  // mtime
    //         0, 0, 0, 4,  // mtime_nsec
    //         0, 0, 0, 5,  // dev
    //         0, 0, 0, 6,  // ino
    //         0, 0, 0, 7,  // mode
    //         0, 0, 0, 8,  // uid
    //         0, 0, 0, 9,  // gid
    //         0, 0, 0, 10, // filesize
    //         1, 2, 3, 4, 5, 6, 7, 8, 9, 10, // hash
    //         0, 1, // filename_size
    //         b'f', b'i', b'l', b'e', b'.', b't', b'x', b't', // filename
    //     ];

    //     assert_eq!(bytes, expected_bytes);
    // }
// }
