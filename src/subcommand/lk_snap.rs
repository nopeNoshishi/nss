//! **Parser command** ... Base command: `git ls-files`


// External
use  anyhow::Result;

// Internal
use crate::struct_set::Index;



pub fn run() -> Result<()>{
    let index = Index::from_rawindex()?;

    for filemeta in index.filemetas {
        println!("{}", filemeta.filename);
    }

    Ok(())
}

pub fn run_option_s() -> Result<()> {
    let index = Index::from_rawindex()?;

    for filemeta in index.filemetas {
        println!("{:0>6o} {} 0\t{}", filemeta.mode, hex::encode(filemeta.hash), filemeta.filename);
    }

    Ok(())
}
