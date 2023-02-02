//! **Write-tree command** Base command: `git write-tree`
//! 
//! /// TODO: Documentation
//! 


use std::io::prelude::*;
use std::fs::File;
use  std::path::PathBuf;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use anyhow::Result;

use crate::struct_set::{Index, Tree, Object, Hashable};
use crate::util::gadget;

pub fn run() -> Result<()>{
    let index = Index::from_rawindex()?;

    // create blob object
    let mut dir_paths: Vec<PathBuf> = vec![];
    for filemeta in index.clone().filemetas {
        let file_path = PathBuf::from(filemeta.filename);
        write_object(&file_path)?;

        let repo_path = gadget::get_repo_path()?;
        let dir_name = file_path.parent().unwrap().to_path_buf();
        dir_paths.push(repo_path.join(dir_name));
    }

    // create tree object
    dir_paths.sort();
    dir_paths.dedup();
    for dir_path in dir_paths {
        write_object(&dir_path)?;
    }

    //TODO: 上記で作成した親ディレクトリだけを拾いたい。
    let tree = Tree::try_from(index)?;

    let hash = hex::encode(tree.to_hash());
    let object_path = gadget::get_objects_path(&hash)?;
    gadget::create_dir(&object_path.parent().unwrap().to_path_buf())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&mut &tree.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush()?;

    println!("Tree hash: {}", hash);

    // memo tree hash value
    let memo_path = gadget::get_memo_path()?;
    let mut file = File::create(memo_path.join("for_reg"))?;
    file.write_all(hash.as_bytes())?;
    file.flush()?;

    Ok(())
}

/// TODO: 全てのツリーが作成できるようにする。
pub fn write_object(path: &PathBuf) -> Result<()> {    
    let object = Object::new(path)?;

    let all_path = hex::encode(object.to_hash());
    let object_path = gadget::get_objects_path(&all_path)?;
    gadget::create_dir(&object_path.parent().unwrap().to_path_buf())?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&mut &object.as_bytes())?;
    let compressed = encoder.finish()?;

    let mut file = File::create(object_path)?;
    file.write_all(&compressed)?;
    file.flush()?;

    Ok(())
}