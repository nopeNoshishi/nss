//! **Snap command** Base command: `git add`

use std::path::{Path, PathBuf};

use super::up_snap;
use crate::struct_set::{Blob, Hashable};
use crate::util::file_system;
use crate::util::gadget::NssRepository;

use anyhow::Result;

pub fn shot(repository: NssRepository, file_path: &str) -> Result<()> {
    write_blob(repository.clone(), PathBuf::from(&file_path))?;
    up_snap::run(repository, file_path)?;

    Ok(())
}

pub fn shot_all(repository: NssRepository) -> Result<()> {
    let all_files = file_system::get_all_paths_ignore(repository.path(), &repository.path());

    for file_path in all_files {
        write_blob(repository.clone(), file_path)?;
    }

    up_snap::run_all(repository)?;

    Ok(())
}

fn write_blob<P: AsRef<Path>>(repository: NssRepository, path: P) -> Result<()> {
    let blob = Blob::new(path.as_ref())?;

    let hash = hex::encode(blob.to_hash());
    file_system::write_object(repository.objects_path(hash), blob)?;

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
