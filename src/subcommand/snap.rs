//! **Snap command** Base command: `git add`

use super::{up_snap, write_tree};

use anyhow::Result;

pub fn shot(file_path: &str) -> Result<()> {
    up_snap::run(file_path)?;
    write_tree::run()?;

    Ok(())
}

pub fn shot_all() -> Result<()> {
    up_snap::run_all()?;
    write_tree::run()?;

    Ok(())
}
