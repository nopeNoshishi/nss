//! **Reg command** ... Base command: `git commit` and `git commit-tree`

// Std
use std::fs;
use std::fs::File;
use std::io::prelude::*;

// External
use anyhow::{Result, bail};
use colored::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

// Internal
use crate::struct_set::Commit;
use crate::struct_set::object::Hashable;
use crate::util::gadget::{self, get_objects_path};

pub fn run(massage: &str) -> Result<()> {
    let hash = fs::read_to_string(".nss/memo/for_reg")?;

    let object_path = gadget::get_objects_path(&hash)?;

    if !object_path.exists() {
        bail!("Can't find tree object. Please write-tree first");
    }


    let head_hash = read_head()?.unwrap();

    // TODO! Get two parent. Parse author and commiter.
    // TODO: 最初のコメットだけ親がいないパターンを網羅する。
    let commit = Commit::new(  
        hash,
        head_hash,
        "nopeNoshishi".to_string(),
        "nopeNoshishi@nope.noshishi".to_string(),
        massage.to_string()
    ).unwrap();

    let hash = hex::encode(commit.to_hash());
    let object_path = get_objects_path(&hash)?;
    gadget::create_dir(&object_path.parent().unwrap().to_path_buf()).unwrap();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&mut &commit.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();

    let mut file = File::create(object_path).unwrap();
    file.write_all(&compressed).unwrap();
    file.flush().unwrap();

    let old_hash = commit.parent.as_str();
    let new_hash = hash.as_str();

    println!("{}: {} --> {}: {}",
        format!("OLD").bright_blue(),
        &old_hash[0..7],
        format!("NEW").bright_yellow(),
        &new_hash[0..7]);

    Ok(())
}

fn read_head() -> Result<Option<String>> {
    let head_path = gadget::get_head_path()?;

    let mut file = File::open(head_path)?;
    let mut referece = String::new();
    file.read_to_string(&mut referece).unwrap();

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    if prefix_path[1].contains("/") {
        let bookmarker = prefix_path[1].split('/').collect::<Vec<&str>>()[2];
        let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

        let mut file = File::open(bookmark_path).unwrap();
        let mut hash = String::new();
        file.read_to_string(&mut hash).unwrap();

        if hash == "".to_string() {
            return Ok(None)
        }

        return Ok(Some(hash))
    }

    Ok(Some(prefix_path[1].to_owned()))
}


