// TODO: 読み書きが発生する部分はここにまとめる

// Std
use std::io::prelude::*;
use std::path::PathBuf;
use std::fs;
use std::fs::File;

// External
use anyhow::{Context, Result, bail};
use dirs::home_dir;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;

// Internal
use super::gadget;
use crate::struct_set::{Blob, Tree, Hashable, Commit};

pub fn exists_repo (repo_dir: Option<PathBuf>) -> Result<PathBuf> {

    let current_dir = match repo_dir {
        Some(p) => {
            if p == home_dir().unwrap() {
                bail!("not a nss repository (or any of the parent directories): .nss")
            } else {
                p
            }
        }
        _ => std::env::current_dir()?
    };
    
    let repo_path = current_dir.join(PathBuf::from(".nss"));
    let read_dir = fs::read_dir(&current_dir)?;

    for entry in read_dir {
        match entry?.path() == repo_path {
            true => {
                return Ok(current_dir)
            },
            false => continue
        }
    }

    return exists_repo(Some(current_dir.parent().unwrap().to_path_buf()))
}

pub fn write_blob<S: AsRef<str>> (hash: S, object: Blob) -> Result<()> {
    let object_path = gadget::get_new_objects_path(hash.as_ref())?;
    gadget::create_dir(object_path.parent().unwrap())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&mut &object.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush().unwrap();

    Ok(())
}

pub fn write_tree<S: AsRef<str>> (hash: S, tree: Tree) -> Result<()> {
    let object_path = gadget::get_new_objects_path(hash.as_ref())?;
    gadget::create_dir(object_path.parent().unwrap())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&mut &tree.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush().unwrap();

    Ok(())
}

pub fn write_commit<S: AsRef<str>> (hash: S, object: Commit) -> Result<()> {
    let object_path = gadget::get_new_objects_path(hash.as_ref())?;
    gadget::create_dir(object_path.parent().unwrap())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&mut &object.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush().unwrap();

    Ok(())
}

pub fn read_object<S: AsRef<str>> (hash: S) -> Result<Vec<u8>> {
    let hash_path = gadget::get_objects_path(hash.as_ref())?;

    // read objectz
    let mut file = File::open(hash_path)
        .with_context(|| format!("{} doesn't exit in this repository", hash.as_ref()))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .with_context(|| format!("{} content can't read", &hash.as_ref()))?;

    // decode content
    let mut decoder = ZlibDecoder::new(&bytes[..]);
    let mut object_content:Vec<u8> = Vec::new();
    decoder.read_to_end(&mut object_content)?;

    Ok(object_content)
}
