//! **Go-to command** ... Base command: `git switch`
//! 
//! Update the working directory and index based on 
//! the specified commit.

// Std
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;

use anyhow::Context;
// External
use anyhow::{Result, bail};

// Internal
use crate::struct_set::{Tree, Index, Object};
use crate::util::{gadget, file_system};

pub fn run(target: &str) -> Result<()> {
    // target needs to be commit hash
    let tree = to_base_tree(target)?;

    // clean working directory
    delete_file()?;

    // restoration by tree
    let repo_path = gadget::get_repo_path()?;
    match create_file(tree.clone(), repo_path) {
        Ok(_) => {
            // update index
            let index = Index::try_from(tree)?;
            let index_path = gadget::get_index_path()?;
            let mut file = File::create(&index_path)?;
            file.write_all(&index.as_bytes())?;
            file.flush()?;
        },
        Err(e) => {
            let head_hash =read_head()?.unwrap();
            let raw_content = file_system::read_object(head_hash)?;
            let commit = match Object::from_content(raw_content)? {
                Object::Commit(c) => c,
                _ => bail!("{} is not commit hash", target)
            };

            let raw_content = file_system::read_object(&commit.tree_hash)?;
            let tree = match Object::from_content(raw_content)? {
                Object::Tree(t) => t,
                _ => bail!("{} is not tree hash", target)
            };

            let repo_path = gadget::get_repo_path()?;
            create_file(tree, repo_path)?;
            bail!("{}\nCan't go to {}", e, target)
        }
    }

    update_head(target)?;

    Ok(())
}

fn to_base_tree(target: &str) -> Result<Tree, anyhow::Error> {
    let raw_content = file_system::read_object(target)?;
    let commit = match Object::from_content(raw_content)? {
        Object::Commit(c) => c,
        _ => bail!("{} is not commit hash", target)
    };

    // target commit hash needs to have tree hash
    let raw_content = file_system::read_object(&commit.tree_hash)?;

    match Object::from_content(raw_content)? {
        Object::Tree(t) => Ok(t),
        _ => bail!("{} is not tree hash", &commit.tree_hash)
    }
    
}

fn delete_file() -> Result<()> {
    let index = Index::from_rawindex()?;

    for path in index.filemetas {
        fs::remove_file(path.filename)?
    }

    Ok(())
}

fn create_file(tree: Tree, prefix: PathBuf) -> Result<()>  {

    for entry in tree.entries {
        let entry_hash = hex::encode(entry.hash);

        let raw_content = file_system::read_object(entry_hash)?;
        match Object::from_content(raw_content)? {
            Object::Blob(b) => {
                let path = prefix.join(entry.name);
                gadget::create_dir(&path.parent().unwrap()).context("No create")?;
                let mut file = File::create(&path).context("No create")?;
                file.write_all(&b.content)?;
                file.flush()?;
            },
            Object::Tree(t) => {
                create_file(t, prefix.join(entry.name))?
            }
        _ => bail!("This tree has commit hash. Please check lk-snap command!")
        };
    }

    Ok(())
}

fn read_head() -> Result<Option<String>> {
    let head_path = gadget::get_head_path()?;

    let mut file = File::open(head_path).unwrap();
    let mut referece = String::new();
    file.read_to_string(&mut referece).unwrap();

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    if prefix_path[1].contains("/") {
        let bookmarker = prefix_path[1].split('/').collect::<Vec<&str>>()[2];
        let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

        let mut file = File::open(bookmark_path).unwrap();
        let mut hash = String::new();
        file.read_to_string(&mut hash).unwrap();

        return Ok(Some(hash))
    }

    Ok(Some(prefix_path[1].to_owned()))
}

pub fn update_head(target: &str) -> Result<()> {

    let head_path = gadget::get_head_path()?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(head_path)?;

    file.write_all(format!("bookmarker: {}", target).as_bytes())?;

    Ok(())
}
