
// Std
use std::fs::File;
use std::io::prelude::*;

// External
use anyhow::{Result, bail};
use colored::*;

// Internal
use crate::struct_set::Object;
use crate::util::{gadget, file_system};

pub fn run() -> Result<()> {
    let head_hash = match read_head()? {
        Some(h) => h,
        _ => bail!("No history yet. You start new journey!")
    };

    go_back(&head_hash)?;

    Ok(())
}

pub fn run_option_s() -> Result<()> {
    let head_hash = match read_head()? {
        Some(h) => h,
        _ => bail!("No history yet. You start new journey!")
    };

    go_back_option_s(&head_hash)?;

    Ok(())
}

fn go_back(hash: &str) -> Result<()> {
    let raw_content = file_system::read_object(hash)?;
    let object: Object = Object::from_content(raw_content)?;

    let commit = match object {
        Object::Commit(c) => c,
        _ => todo!()
    };

    println!("{}\n{}\n\n\t{}\n",
        format!("commit: {}", hash).yellow(),
        format!("Author: {}", commit.author),
        format!("   {}", commit.message));
    
    if commit.parent != "None".to_string() {
        go_back(&commit.parent)?
    }

    Ok(())
}

fn go_back_option_s(hash: &str) -> Result<()> {
    let raw_content = file_system::read_object(hash)?;
    let object: Object = Object::from_content(raw_content)?;

    let commit = match object {
        Object::Commit(c) => c,
        _ => todo!()
    };

    println!("{} {}",
        format!("{}", &hash[0..7]).yellow(),
        format!("{}", commit.message));
    if commit.parent != "None".to_string() {
        go_back_option_s(&commit.parent)?
    }

    Ok(())
}

fn read_head() -> Result<Option<String>> {
    let head_path = gadget::get_head_path()?;

    let mut file = File::open(head_path).unwrap();
    let mut referece = String::new();
    file.read_to_string(&mut referece).unwrap();

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    if prefix_path[1].contains("/") {
        let bookmarker = prefix_path[1].split('/').collect::<Vec<&str>>()[2];
        let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

        let mut file = File::open(bookmark_path).unwrap();
        let mut hash = String::new();
        file.read_to_string(&mut hash).unwrap();

        return Ok(Some(hash))
    }

    Ok(Some(prefix_path[1].to_owned()))
}
