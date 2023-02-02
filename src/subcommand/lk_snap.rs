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
        println!("{} {} 0\t{}", num_to_mode(filemeta.mode as u16), hex::encode(filemeta.hash), filemeta.filename);
    }

    Ok(())
}

pub fn num_to_mode(val: u16) -> String {
    // mode to display
    let file_type = val >> 13;
    let (user, group, other) = {
        let permission = val & 0x01ff;
        let user = (permission & 0x01c0) >> 6;
        let group = (permission & 0x0038) >> 3;
        let other = permission & 0x0007;

        (user, group, other)
    };

    format!("{:03b}{}{}{}", file_type, user, group, other)
}
