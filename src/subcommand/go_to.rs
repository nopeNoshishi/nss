//! **Go-to command** ... Base command: `git switch`
//!
//! Update the working directory and index based on
//! the specified commit.

// Std
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::Context;
// External
use anyhow::{bail, Result};

// Internal
use crate::struct_set::{Index, Object, Tree};
use crate::util::file_system;
use crate::util::gadget::NssRepository;

// TODO: when delete or create , use tempolary dir
pub fn run(repository: NssRepository, target: &str) -> Result<()> {
    // target needs to be commit hash
    let tree = to_base_tree(repository.path(), target)?;

    // clean working directory
    delete_file()?;

    // restoration by tree
    match create_file(repository.path(), tree.clone(), repository.path()) {
        Ok(_) => {
            // update index
            let index = Index::try_from(tree)?;
            let mut file = File::create(repository.index_path())?;
            file.write_all(&index.as_bytes())?;
            file.flush()?;
        }
        Err(e) => {
            let head_hash = read_head(repository.clone())?.unwrap();
            let raw_content = file_system::read_object(repository.path(), head_hash)?;
            let commit = match Object::from_content(raw_content)? {
                Object::Commit(c) => c,
                _ => bail!("{} is not commit hash", target),
            };

            let raw_content = file_system::read_object(repository.path(), commit.tree_hash)?;
            let tree = match Object::from_content(raw_content)? {
                Object::Tree(t) => t,
                _ => bail!("{} is not tree hash", target),
            };

            create_file(repository.path(), tree, repository.path())?;
            bail!("{}\nCan't go to {}", e, target)
        }
    }

    update_head(repository, target)?;

    Ok(())
}

fn to_base_tree(repo_path: PathBuf, target: &str) -> Result<Tree, anyhow::Error> {
    let raw_content = file_system::read_object(repo_path.clone(), target)?;
    let commit = match Object::from_content(raw_content)? {
        Object::Commit(c) => c,
        _ => bail!("{} is not commit hash", target),
    };

    // target commit hash needs to have tree hash
    let raw_content = file_system::read_object(repo_path, &commit.tree_hash)?;

    match Object::from_content(raw_content)? {
        Object::Tree(t) => Ok(t),
        _ => bail!("{} is not tree hash", &commit.tree_hash),
    }
}

fn delete_file() -> Result<()> {
    let index = Index::from_rawindex()?;

    for path in index.filemetas {
        fs::remove_file(path.filename)?
    }

    Ok(())
}

fn create_file(repo_path: PathBuf, tree: Tree, prefix: PathBuf) -> Result<()> {
    for entry in tree.entries {
        let entry_hash = hex::encode(entry.hash);

        let raw_content = file_system::read_object(repo_path.clone(), entry_hash)?;
        match Object::from_content(raw_content)? {
            Object::Blob(b) => {
                let path = prefix.join(entry.name);
                file_system::create_dir(path.parent().unwrap()).context("No create")?;
                let mut file = File::create(&path).context("No create")?;
                file.write_all(&b.content)?;
                file.flush()?;
            }
            Object::Tree(t) => create_file(repo_path.clone(), t, prefix.join(entry.name))?,
            _ => bail!("This tree has commit hash. Please check lk-snap command!"),
        };
    }

    Ok(())
}

fn read_head(repository: NssRepository) -> Result<Option<String>> {
    let mut file = File::open(repository.head_path()).unwrap();
    let mut referece = String::new();
    file.read_to_string(&mut referece).unwrap();

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    if prefix_path[1].contains('/') {
        let bookmarker = prefix_path[1].split('/').collect::<Vec<&str>>()[2];

        let mut file = File::open(repository.bookmarks_path(bookmarker)).unwrap();
        let mut hash = String::new();
        file.read_to_string(&mut hash).unwrap();

        return Ok(Some(hash));
    }

    Ok(Some(prefix_path[1].to_owned()))
}

pub fn update_head(repository: NssRepository, target: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(repository.head_path())?;

    file.write_all(format!("bookmarker: {}", target).as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_delete_file() {}
}
