//! **Snap command** Base command: `git add`

use std::path::{Path, PathBuf};

use super::up_snap;
use crate::struct_set::{Blob, Hashable};
use crate::util::{file_system, gadget};

use anyhow::Result;

pub fn shot(file_path: &str) -> Result<()> {
    write_blob(PathBuf::from(&file_path))?;
    up_snap::run(file_path)?;

    Ok(())
}

pub fn shot_all() -> Result<()> {
    let repo_path = gadget::get_repo_path()?;
    let all_files = gadget::get_all_paths_ignore(repo_path);

    for file_path in all_files {
        write_blob(file_path)?;
    }

    up_snap::run_all()?;

    Ok(())
}

fn write_blob<P: AsRef<Path>>(path: P) -> Result<()> {
    let blob = Blob::new(path.as_ref())?;

    let hash = hex::encode(blob.to_hash());
    file_system::write_blob(hash, blob)?;

    Ok(())
}
