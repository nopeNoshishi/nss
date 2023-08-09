//! **Go-to command** ... Base command: `git switch`
//!
//! Update the working directory and index based on
//! the specified commit.

// External
use anyhow::{bail, Result};

// Internal
use nss_core::nss_io::file_system;
use nss_core::repository::NssRepository;
use nss_core::struct_set::{Index, Object, Tree};

// TODO: when delete or create , use tempolary dir
pub fn run(repository: &NssRepository, target: &str) -> Result<()> {
    // Get target index
    let tree = to_base_tree(repository, target)?;
    let target_index = Index::try_from_tree(repository, tree)?;

    // Clear working directory by reference HEAD Index
    let head_index = repository.read_index()?;
    delete_file(repository, &head_index)?;

    // restoration by tree
    match create_file(repository, &target_index) {
        Ok(_) => {
            // Target index -> HEAD Index
            repository.write_index(target_index)?;
        }
        Err(e) => {
            // Rollback
            create_file(repository, &head_index)?;
            bail!("{}\nCan't go to {}", e, target)
        }
    }

    repository.write_head(target)?;

    Ok(())
}

fn to_base_tree(repository: &NssRepository, target: &str) -> Result<Tree> {
    let commit_hash = match repository.read_bookmark(target) {
        Ok(c) => c,
        Err(_) => target.to_string(),
    };

    let commit = match repository.read_object(&commit_hash)? {
        Object::Commit(c) => c,
        _ => bail!("{} is not commit hash", commit_hash),
    };

    // target commit hash needs to have tree hash
    match repository.read_object(&commit.tree_hash)? {
        Object::Tree(t) => Ok(t),
        _ => bail!("{} is not tree hash", &commit.tree_hash),
    }
}

fn delete_file(repository: &NssRepository, index: &Index) -> Result<()> {
    for path in index.filemetas.iter() {
        match file_system::remove_file(repository.path().join(&path.filename)) {
            Ok(..) => (),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => (),
                _ => bail!("Unexpected error occurred while deleting the file！"),
            },
        }
    }

    Ok(())
}

fn create_file(repository: &NssRepository, index: &Index) -> Result<()> {
    let prefix = repository.path();

    for filemeta in index.filemetas.clone() {
        let entry_hash = hex::encode(filemeta.hash);

        match repository.read_object(entry_hash)? {
            Object::Blob(blob) => {
                let path = prefix.join(filemeta.filename);
                file_system::create_dir(path.parent().unwrap())?;
                file_system::create_with_buffer(path, &blob.content)?;
            }
            _ => bail!("Index has tree object, so your commit is broken! (in go-to branch)"),
        };
    }

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
