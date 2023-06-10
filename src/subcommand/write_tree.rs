//! **Write-tree command** Base command: `git write-tree`

// Std
use std::collections::HashMap;
use std::path::PathBuf;

// External
use anyhow::Result;

// Internal
use crate::repo::NssRepository;
use crate::struct_set::{Entry, Hashable, Index, Object, Tree};

pub fn run(repository: &NssRepository) -> Result<()> {
    let index = repository.read_index()?;
    let tree_dir = tree_map(repository.path(), index)?;

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
        repository.write_object(tree)?;

        if m.0 == repository.path() {
            repo_tree_hash = hash
        }
    }

    println!("Tree hash: {}", repo_tree_hash);

    Ok(())
}

fn tree_map(repo_path: PathBuf, index: Index) -> Result<Vec<(PathBuf, Vec<PathBuf>)>> {
    let mut file_paths: Vec<PathBuf> = vec![];
    let mut dir_paths: Vec<PathBuf> = vec![];
    for filemeta in index.filemetas {
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
    fn test_run() {}

    #[test]
    fn test_tree_map() {}
}
