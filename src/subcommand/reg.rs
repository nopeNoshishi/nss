//! **Reg command** ... Base command: `git commit` and `git commit-tree`

// Std
use std::collections::HashMap;
use std::path::PathBuf;

// External
use anyhow::{bail, Result};
use colored::*;

// Internal
use nss_core::error::*;
use nss_core::nss_io::file_system;
use nss_core::repository::NssRepository;
use nss_core::struct_set::{Commit, Entry, Hashable, Index, Object, Tree};

pub fn run(repository: &NssRepository, massage: &str) -> Result<()> {
    // Create tree object from index
    let hash = write_tree(repository)?;

    // Read head hash
    let head_hash = repository.read_head()?;

    let parents = match head_hash.len() {
        0 => vec![],
        _ => vec![head_hash.clone()],
    };

    let config = repository.read_config()?;

    // Build commit object
    let commit = Commit::new(
        hash,
        parents,
        format!(
            "{}\0 {}",
            config.username(),
            config.useremail().unwrap_or_default()
        ),
        format!(
            "{}\0 {}",
            config.username(),
            config.useremail().unwrap_or_default()
        ),
        massage.to_string(),
    )?;

    // Write commit object
    let hash = hex::encode(commit.to_hash());
    repository.write_object(commit)?;

    display_result(repository, head_hash.as_str(), hash.as_str())?;

    Ok(())
}

fn display_result(repository: &NssRepository, old_hash: &str, new_hash: &str) -> Result<()> {
    let bookmarker = repository
        .read_head_base()
        .unwrap_or("DetachHead".to_string());

    if &bookmarker != "DetachHead" {
        update_bookmark(repository, &bookmarker, new_hash, None)?;
    } else {
        println!("You are working at detached head!")
    }

    let old_hash = if old_hash.len() >= 7 {
        &old_hash[0..7]
    } else {
        "Detach Head"
    };

    println!(
        "{}: {} --> {}: {}",
        "OLD".bright_blue(),
        &old_hash[0..7],
        "NEW".bright_yellow(),
        &new_hash[0..7]
    );

    Ok(())
}

fn update_bookmark(
    repository: &NssRepository,
    bookmarker: &str,
    new_commit: &str,
    old_commit: Option<&str>,
) -> Result<()> {
    let bookmark_hash = repository.read_bookmark(bookmarker)?;

    if let Some(commit) = old_commit {
        if bookmark_hash != commit {
            bail!(RepositoryError::DontMatchHashAtBookmarker(bookmark_hash));
        }
    }

    repository.write_bookmark(bookmarker, new_commit.to_string())
}

fn write_tree(repository: &NssRepository) -> Result<String> {
    let index = repository.read_index()?;
    let tree_dir = tree_map(index)?;

    let mut repo_tree_hash = String::new();
    let mut dir_entry_map: HashMap<PathBuf, Entry> = HashMap::new();
    for m in tree_dir {
        let mut entries: Vec<Entry> = vec![];

        for path in m.1 {
            if path.is_file() {
                let object = Object::new(&path)?;
                let entry = Entry::new(path, object)?;
                entries.push(entry)
            } else {
                let entry = dir_entry_map.get(&path).unwrap().to_owned();
                entries.push(entry)
            }
        }

        let dir_entry = Entry::new_group(&m.0, entries.clone())?;
        dir_entry_map.insert(m.0.to_path_buf(), dir_entry);

        let tree = Tree::from_entries(entries);
        let hash = hex::encode(tree.to_hash());

        if let Err(err) = repository.write_object(tree) {
            if let Some(obt_err) = err.downcast_ref::<ObjectError>() {
                if obt_err != &ObjectError::AlreadyExistsObject {
                    bail!(err)
                } else {
                    // Already existed tree
                    // if m.0 == repository.path() {
                    //     bail!(err)
                    // }
                }
            } else {
                bail!(err)
            }
        }

        if m.0 == repository.path() {
            repo_tree_hash = hash
        }
    }

    Ok(repo_tree_hash)
}

fn tree_map(index: Index) -> Result<Vec<(PathBuf, Vec<PathBuf>)>> {
    let mut file_paths: Vec<PathBuf> = vec![];
    let mut dir_paths: Vec<PathBuf> = vec![];
    for filemeta in index.filemetas {
        let repo_path = file_system::exists_repo::<PathBuf>(None)?;
        let file_path = repo_path.join(filemeta.filename);
        let mut dir_name = file_path.parent().unwrap().to_path_buf();

        file_paths.push(repo_path.join(file_path));
        dir_paths.push(repo_path.clone());

        while dir_name != repo_path {
            dir_paths.push(repo_path.join(dir_name.clone()));

            dir_name = dir_name.parent().unwrap().to_path_buf();
        }
    }
    dir_paths.sort();
    dir_paths.dedup();

    let mut temp_map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for dir in &dir_paths {
        temp_map.insert(dir.to_path_buf(), vec![]);

        for file in &file_paths {
            if dir == &file.parent().unwrap().to_path_buf() {
                temp_map
                    .get_mut(&dir.to_path_buf())
                    .unwrap()
                    .push(file.to_path_buf())
            }
        }

        for sub_dir in &dir_paths {
            if dir == &sub_dir.parent().unwrap().to_path_buf() {
                temp_map
                    .get_mut(&dir.to_path_buf())
                    .unwrap()
                    .push(sub_dir.to_path_buf())
            }
        }
    }

    let mut tmp: Vec<(&PathBuf, &Vec<PathBuf>)> = temp_map.iter().collect();
    tmp.sort_by(|b, a| {
        let comp_a: Vec<&std::ffi::OsStr> = a.0.iter().collect();
        let comp_b: Vec<&std::ffi::OsStr> = b.0.iter().collect();
        if comp_a.len() >= comp_b.len() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    let mut tree_dir: Vec<(PathBuf, Vec<PathBuf>)> = vec![];
    for t in tmp {
        tree_dir.push((t.0.to_owned(), t.1.to_vec()))
    }

    Ok(tree_dir)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {
        let a = format!("{}\0 {}", "test", "a");
        let b: Vec<&str> = a.split("\0 ").collect();
        println!("{:?}", b);
    }

    #[test]
    fn test_read_head() {}

    #[test]
    fn test_head_hash() {}

    #[test]
    fn test_write_tree() {}

    #[test]
    fn test_tree_map() {}
}
