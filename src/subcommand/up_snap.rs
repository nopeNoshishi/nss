//! **Update-index command** Base command: `git update-index
//! /// TODO: Documentation`

// Std
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

// External
use anyhow::Result;

// Internal
use crate::struct_set::Index;
use crate::repo::NssRepository;

pub fn run(repository: NssRepository, file_path: &str) -> Result<()> {
    let mut index = match repository.read_index() {
        Ok(index) => index,
        Err(e) => {
            println!("{}", e);
            Index::empty()
        }
    };

    let file_path = PathBuf::from(file_path);
    index.add(&repository.path(), &file_path)?;

    let mut file = File::create(repository.index_path())?;
    file.write_all(&index.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn run_all(repository: NssRepository) -> Result<()> {
    let index = Index::new_all(&repository)?;

    let mut file = File::create(repository.index_path())?;
    file.write_all(&index.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn run_option_w(repository: NssRepository) -> Result<()> {
    let mut all_paths = repository.get_all_paths_ignore(&repository.path());
    all_paths.sort();

    println!("[List of files to be tracked]");
    for path in all_paths {
        println!(
            "{}",
            path.strip_prefix(&repository.path())
                .unwrap()
                .to_str()
                .unwrap()
        )
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
