//! **Snap command** Base command: `git add`

// External
use anyhow::Result;

// Internel
use super::up_snap;
use crate::repo::NssRepository;
use crate::struct_set::Blob;

pub fn shot(repository: &NssRepository, file_path: &str) -> Result<()> {
    let blob = Blob::new(file_path)?;
    repository.write_object(blob)?;
    up_snap::run(repository, file_path)?;

    Ok(())
}

pub fn shot_all(repository: &NssRepository) -> Result<()> {
    let all_files = repository.get_all_paths_ignore(repository.path());

    for file_path in all_files {
        let blob = Blob::new(file_path)?;
        repository.write_object(blob)?;
    }

    up_snap::run_all(repository)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_shot() {}

    #[test]
    fn test_shot_all() {}

    #[test]
    fn test_write_blob() {}
}
