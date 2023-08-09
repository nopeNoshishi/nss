// External
use anyhow::Result;
use chrono::prelude::{Datelike, Local};
use chrono::{Month, TimeZone};
use colored::*;

// Internal
use nss_core::repository::NssRepository;

pub fn run(repository: &NssRepository) -> Result<()> {
    let head_hash = repository.read_head()?;

    go_back(repository, &head_hash)?;

    Ok(())
}

pub fn run_option_s(repository: &NssRepository) -> Result<()> {
    let head_hash = repository.read_head()?;

    go_back_option_s(repository, &head_hash)?;

    Ok(())
}

#[allow(clippy::format_in_format_args, unused_must_use)]
fn go_back(repository: &NssRepository, hash: &str) -> Result<()> {
    let commit = repository.read_commit(hash)?;
    let bookmarker = repository
        .read_head_base()
        .map(|x| x.bright_green().bold())
        .unwrap_or("DetachHead".red().bold());

    let hash = format!("Commit: {}", hash).yellow();
    let branch = format!("({}{})", "HEAD -> ".bright_cyan().bold(), bookmarker);

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

    if !commit.parents.is_empty() {
        commit.parents.iter().for_each(|c| {
            _go_back(repository, c);
        })
    }

    Ok(())
}

fn _go_back(repository: &NssRepository, hash: &str) -> Result<()> {
    let commit = repository.read_commit(hash)?;

    let hash = format!("Commit: {}", hash).yellow();

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
    println!("{}\n{}\n{}\n\n{}\n", hash, author, date, message);

    if !commit.parents.is_empty() {
        commit.parents.iter().for_each(|c| {
            _go_back(repository, c).expect("todo!");
        })
    }

    Ok(())
}

#[allow(clippy::format_in_format_args)]
fn go_back_option_s(repository: &NssRepository, hash: &str) -> Result<()> {
    let commit = repository.read_commit(hash)?;

    println!(
        "{} {}",
        format!("{:?}", &hash[0..7]).yellow(),
        format!("{}", commit.message)
    );

    if !commit.parents.is_empty() {
        commit.parents.iter().for_each(|c| {
            go_back_option_s(repository, c).expect("todo!");
        })
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

    #[test]
    fn test_go_back() {}

    #[test]
    fn test_go_back_option_s() {}
}
