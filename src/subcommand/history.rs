// Std
use std::fs::File;
use std::io::prelude::*;

// External
use anyhow::{bail, Result};
use chrono::prelude::{Datelike, Local};
use chrono::{Month, TimeZone};
use colored::*;

// Internal
use crate::repo::NssRepository;
use crate::struct_set::{Hashable, Object};

pub fn run(repository: &NssRepository) -> Result<()> {
    let head_hash = match read_head(repository)? {
        Some(h) => h,
        _ => bail!("No history yet. You start new journey!"),
    };

    go_back(repository, &head_hash)?;

    Ok(())
}

pub fn run_option_s(repository: &NssRepository) -> Result<()> {
    let head_hash = match read_head(repository)? {
        Some(h) => h,
        _ => bail!("No history yet. You start new journey!"),
    };

    go_back_option_s(repository, &head_hash)?;

    Ok(())
}

#[allow(clippy::format_in_format_args)]
fn go_back(repository: &NssRepository, hash: &str) -> Result<()> {
    let object = repository.read_object(hash)?;

    let commit = match object {
        Object::Commit(commit) => commit,
        _ => bail!("Not commit hash ({})", hex::encode(object.to_hash())),
    };

    let hash = format!("Commit: {}", hash).yellow();
    let branch = format!("({}{})", "HEAD -> ".bright_cyan().bold(), "voyage".bright_green().bold());

    let author = format!("Author: {}", commit.author);
    let timestamp = Local.timestamp_opt(commit.date.timestamp(), 0).unwrap();
    let date = format!(
        "Date:   {} {:.3} {}",
        timestamp.weekday(),
        Month::try_from(timestamp.month() as u8).unwrap().name(),
        timestamp.format("%d %H:%M:%S %Y %z")
    );
    let message = format!("    {}", commit.message);

    // Output
    println!("{} {}\n{}\n{}\n\n{}\n", hash, branch, author, date, message);

    if commit.parent != *"None" {
        go_back(repository, &commit.parent)?
    }

    Ok(())
}

#[allow(clippy::format_in_format_args)]
fn go_back_option_s(repository: &NssRepository, hash: &str) -> Result<()> {
    let object = repository.read_object(hash)?;

    let commit = match object {
        Object::Commit(c) => c,
        _ => todo!(),
    };

    println!(
        "{} {}",
        format!("{:?}", &hash[0..7]).yellow(),
        format!("{}", commit.message)
    );
    if commit.parent != *"None" {
        go_back_option_s(repository, &commit.parent)?
    }

    Ok(())
}

fn read_head(repository: &NssRepository) -> Result<Option<String>> {
    let mut file = File::open(repository.head_path()).unwrap();
    let mut referece = String::new();
    file.read_to_string(&mut referece).unwrap();

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    if prefix_path[1].contains('/') {
        let bookmarker = prefix_path[1].split('/').collect::<Vec<&str>>()[2];

        let mut file = File::open(repository.bookmarks_path(bookmarker)).unwrap();
        let mut hash = String::new();
        file.read_to_string(&mut hash).unwrap();

        return Ok(Some(hash));
    }

    Ok(Some(prefix_path[1].to_owned()))
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_run() {}

    #[test]
    fn test_run_option_s() {}

    #[test]
    fn test_go_back() {}

    #[test]
    fn test_go_back_option_s() {}

    #[test]
    fn test_read_head() {}
}
