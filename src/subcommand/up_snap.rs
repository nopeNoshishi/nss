//! **Update-index command** Base command: `git update-index

// Std
use std::collections::HashSet;
use std::path::PathBuf;

// External
use anyhow::Result;
use colored::*;

// Internal
use nss_core::repository::NssRepository;
use nss_core::struct_set::Index;

pub fn run(repository: &NssRepository, file_path: &str) -> Result<()> {
    let mut index = match repository.read_index() {
        Ok(index) => index,
        Err(e) => {
            println!("{}", e);
            Index::empty()
        }
    };

    index.add(repository, &repository.path().join(file_path), None)?;

    repository.write_index(index)?;

    Ok(())
}

pub fn run_all(repository: &NssRepository) -> Result<()> {
    let index = Index::new_all(repository)?;

    repository.write_index(index)?;

    Ok(())
}

pub fn run_option_v(repository: &NssRepository) -> Result<()> {
    let tracked_files = repository
        .read_index()?
        .filemetas
        .iter()
        .map(|f| PathBuf::from(&f.filename))
        .collect::<Vec<PathBuf>>();
    let mut all_paths = repository
        .get_all_paths_ignore(&repository.path())
        .iter()
        .map(|p| p.strip_prefix(&repository.path()).unwrap().to_path_buf())
        .collect::<Vec<PathBuf>>();
    all_paths.sort();

    println!("[List of files tracked]");
    for path in tracked_files.clone() {
        println!("{}", path.display())
    }

    let set1: HashSet<_> = tracked_files.iter().collect();
    let set2: HashSet<_> = all_paths.iter().collect();
    let no_tracked_files: Vec<_> = set2.difference(&set1).cloned().collect();

    println!("{}", "\n[List of files to br tracked]".red());
    for path in no_tracked_files {
        println!("{}", path.display())
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_run_all() {}

    #[test]
    fn test_run_option_w() {}
}
