//! **Reg command** ... Base command: `git commit` and `git commit-tree`

// Std
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

// External
use anyhow::{Result, bail};
use colored::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;

// Internal
use crate::struct_set::{Index, Tree, Commit, Hashable};
use crate::util::{gadget, file_system};

pub fn run(massage: &str) -> Result<()> {
    write_tree()?;
    let hash = fs::read_to_string(".nss/memo/for_reg")?;

    let object_path = gadget::get_objects_path(&hash)?;

    if !object_path.exists() {
        bail!("Can't find tree object. Please write-tree first");
    }

    
    let head_hash = match head_hash()? {
        Some(h) => h,
        _ => "None".to_owned()
    };

    // TODO! Get two parent. Parse author and commiter.
    // TODO: 最初のコメットだけ親がいないパターンを網羅する。
    let commit = Commit::new(  
        hash,
        head_hash,
        "nopeNoshishi".to_string(),
        "nopeNoshishi@nope.noshishi".to_string(),
        massage.to_string()
    )?;

    let hash = hex::encode(commit.to_hash());
    let object_path = gadget::get_new_objects_path(&hash)?;
    gadget::create_dir(&object_path.parent().unwrap().to_path_buf()).unwrap();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&mut &commit.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();

    let mut file = File::create(object_path).unwrap();
    file.write_all(&compressed).unwrap();
    file.flush().unwrap();

    let old_hash = commit.parent.as_str();
    let new_hash = hash.as_str();

    match old_hash {
        "None" => {
            println!("{}: {} --> {}: {}",
            format!("OLD").bright_blue(),
            &old_hash,
            format!("NEW").bright_yellow(),
            &new_hash[0..7]);

            let book_path = read_head()?;
            let bookmarker = book_path.split('/').collect::<Vec<&str>>()[2];
            update_bookmark(bookmarker, new_hash, None)?;
        },
        _ => {
            println!("{}: {} --> {}: {}",
            format!("OLD").bright_blue(),
            &old_hash[0..7],
            format!("NEW").bright_yellow(),
            &new_hash[0..7]);

            let book_path = read_head()?;
            let bookmarker = book_path.split('/').collect::<Vec<&str>>()[2];
            update_bookmark(bookmarker, new_hash, Some(old_hash))?;
        }
    }

    Ok(())
}


fn read_head() -> Result<String> {
    let head_path = gadget::get_head_path()?;

    let mut file = File::open(head_path)?;
    let mut referece = String::new();
    file.read_to_string(&mut referece).unwrap();

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    return Ok(prefix_path[1].to_string())
}

fn head_hash() -> Result<Option<String>> {

    let head_item = read_head()?;
    if head_item.contains("/") {
        let bookmarker = head_item.split('/').collect::<Vec<&str>>()[2];
        let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

        let mut file = File::open(bookmark_path).unwrap();
        let mut hash = String::new();
        file.read_to_string(&mut hash).unwrap();

        if hash == "".to_string() {
            return Ok(None)
        }

        return Ok(Some(hash))
    }

    Ok(Some(head_item))
}

fn update_bookmark(bookmarker: &str, new_commit: &str, old_commit: Option<&str>) -> Result<()> {
    let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

    let raw_content = file_system::read_object(new_commit)?;
    if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == String::from("c") {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(bookmark_path)?;
        let mut bookmark_hash = String::new();
        file.read_to_string(&mut bookmark_hash)?;

        if bookmark_hash == "".to_owned() {
            file.write_all(new_commit.as_bytes())?;
        } else if bookmark_hash == old_commit.unwrap().to_owned() {
            file.write_all(new_commit.as_bytes())?;
        } else {
            bail!("This bookmarker has the differnce old hash ({})", bookmark_hash)
        }
    } else {
        bail!("Not commit hash <new commit> ({})", new_commit)
    }

    Ok(())
}

fn write_tree() -> Result<()>{
    let index = Index::from_rawindex()?;
    let tree = Tree::try_from(index)?;

    let hash = hex::encode(tree.to_hash());
    let tree_dir_path = gadget::get_repo_path()?;
    file_system::write_tree(&hash, tree, tree_dir_path)?;

    // memo tree hash value
    let memo_path = gadget::get_memo_path()?;
    gadget::create_dir(&memo_path)?;
    let mut file = File::create(memo_path.join("for_reg"))?;
    file.write_all(hash.as_bytes())?;
    file.flush()?;

    Ok(())
}
