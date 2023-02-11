//! **Write-tree command** Base command: `git write-tree`
//! 
//! /// TODO: Documentation
//! 

// Std
use std::io::prelude::*;
use std::fs::File;

// External
use anyhow::Result;

// Internal
use crate::struct_set::{Index, Tree, Hashable};
use crate::util::{gadget, file_system};

pub fn run() -> Result<()>{
    let index = Index::from_rawindex()?;
    let tree = Tree::try_from(index)?;

    let hash = hex::encode(tree.to_hash());
    let tree_dir_path = gadget::get_repo_path()?;
    file_system::write_tree(&hash, tree, tree_dir_path)?;

    println!("Tree hash: {}", hash);

    // memo tree hash value
    let memo_path = gadget::get_memo_path()?;
    gadget::create_dir(&memo_path)?;
    let mut file = File::create(memo_path.join("for_reg"))?;
    file.write_all(hash.as_bytes())?;
    file.flush()?;

    Ok(())
}
