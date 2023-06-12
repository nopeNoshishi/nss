//! **Parser command** ... Base command: `git ls-files`

// External
use anyhow::Result;

// Internal
use nss_core::repository::NssRepository;

pub fn run(repository: &NssRepository) -> Result<()> {
    let index = repository.read_index()?;

    for filemeta in index.filemetas {
        println!("{}", filemeta.filename.to_str().unwrap());
    }

    Ok(())
}

pub fn run_option_s(repository: &NssRepository) -> Result<()> {
    let index = repository.read_index()?;

    for filemeta in index.filemetas {
        println!(
            "{:0>6o} {} 0\t{}",
            filemeta.mode,
            hex::encode(filemeta.hash),
            filemeta.filename.to_str().unwrap()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_run_option_s() {}
}
