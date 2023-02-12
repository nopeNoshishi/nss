//! Useful method package
//! 
//! Support coding
//! - create_dir
//! - get_all_paths     Under parameter directory path
//! 
//! Repository addresser
//! - get_repo_path
//! - get_objcts_path
//! - get_refs_path
//! - get_config_path
//! - get_head_path
//! - get_index_path
//! - get_repo_path
//! 



use std::io;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use anyhow::{Result, bail};

use super::file_system;


pub fn create_dir<P: AsRef<Path>> (dir_path: P) -> io::Result<()>{

    fs::create_dir_all(dir_path.as_ref())?;

    Ok(())
}

#[allow(dead_code)]
pub fn get_all_paths(target: &PathBuf) -> Result<Vec<PathBuf>> {

    let mut paths = vec![];
    ext_paths(target, paths.as_mut())?;

    Ok(paths)
}

pub fn get_all_paths_ignore<P: AsRef<Path>> (target: P) -> Vec<PathBuf> {
    let mut paths = vec![];
    ext_paths_ignore(target, paths.as_mut());

    paths
}

#[allow(dead_code)]
fn ext_paths<P: AsRef<Path>> (target: P, paths: &mut Vec<PathBuf>) -> Result<()> {
    // Print all files in target directory
    let files = target.as_ref()
        .read_dir()
        .with_context(|| format!("{:?} object database has no objects", target.as_ref()))?;

    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        paths.push(path);
    }
    paths.sort();

    Ok(())
}

pub fn ext_paths_ignore<P: AsRef<Path>> (target: P, paths: &mut Vec<PathBuf>) {
    // Print all files in target directory
    let files = target.as_ref().read_dir().unwrap();

    let repo_path = get_repo_path().unwrap();
    let binding = fs::read_to_string(".nssignore").unwrap();
    let lines = binding.lines();
    let mut ignore_paths: Vec<PathBuf> = Vec::new();

    ignore_paths.push(PathBuf::from(repo_path.join(".git")));
    ignore_paths.push(PathBuf::from(repo_path.join(".nss")));
    ignore_paths.push(PathBuf::from(repo_path.join(".gitignore")));
    ignore_paths.push(PathBuf::from(repo_path.join(".nssignore")));

    for line in lines {
        let ignore_path = repo_path.join(line);
        ignore_paths.push(ignore_path)
    }

    for dir_entry in files {
        let path = dir_entry.unwrap().path();

        let mut do_ignore: bool = false;
        for ignore_path in ignore_paths.clone() {
            if path == ignore_path{
                do_ignore = true
            }
        }

        if do_ignore == true {
            continue;
        }

        if path.is_dir() {
            ext_paths_ignore(&path, paths);
            continue;
        }
        paths.push(path);
    }
    paths.sort();
}

/// Return your repository **absolutely** path
pub fn get_repo_path() -> Result<PathBuf> {

    let repo_path = file_system::exists_repo(None)?;

    Ok(repo_path)
}

/// new object path
pub fn get_new_objects_path<T: Into<String>>(hash: T) -> Result<PathBuf> {

    let hash = hash.into();

    let (dir, file) = hash.split_at(2);
    let repo_path = get_repo_path()?;
    let object_dir = &repo_path.join(".nss/objects").join(dir);

    Ok(object_dir.join(file))
}

/// Return your object database **absolutely** path
pub fn get_objects_path<T: Into<String>>(hash: T) -> Result<PathBuf> {

    let hash = hash.into();

    if hash.len() < 6 {
        bail!("More hash value digit (less digit)")
    }
    let (dir, file) = hash.split_at(2);
    let repo_path = get_repo_path()?;
    let object_dir = &repo_path.join(".nss/objects").join(dir);

    let mut paths: Vec<PathBuf> = vec![];
    ext_paths(object_dir, &mut paths)?;

    let mut target_files: Vec<PathBuf> = vec![];
    for path in paths {
        if path.as_os_str().to_string_lossy().contains(&file) {
            target_files.push(path)
        }
    }

    if target_files.len() > 2 {
        bail!("More hash value digit (nearly hash value exists)")
    } else if target_files.len() == 0 {
        bail!("Doesn't exit in this repository")
    }

    Ok(object_dir.join(&target_files[0]))
}

/// Return references(pointers) **absolutely** path
pub fn get_bookmarks_path(bookmarker: &str) -> Result<PathBuf> {
    
    let repo_path = get_repo_path()?;

    Ok(repo_path.join(".nss/bookmarks/local").join(bookmarker))
}

/// Return config file **absolutely** path
#[allow(dead_code)]
pub fn get_config_path() -> Result<PathBuf> {
   
    let repo_path = get_repo_path()?;

    Ok(repo_path.join(".nss/config"))
}

/// Return HEAD file **absolutely** path
pub fn get_head_path() -> Result<PathBuf> {
    
    let repo_path = get_repo_path()?;

    Ok(repo_path.join(".nss/HEAD"))
}

/// Return index(staging area) file **absolutely** path
pub fn get_index_path() -> Result<PathBuf> {
    
    let repo_path = get_repo_path()?;

    Ok(repo_path.join(".nss/INDEX"))
}

#[cfg(test)]
mod tests {
    // use crate::util::gadget::get_repo_path;

    #[test]
    fn test_get_repopath() {
        // use std::path::PathBuf;

        // let nss_dirs = [".nss", ".nss/bookmarks", ".nss/objects",
        //                            ".nss/bookmarks/local", ".nss/memo"];

        // for dir_path in nss_dirs {
        //     create_dir(&PathBuf::from(dir_path))?
        // }

        // let nss_path = ".nss/repo";
        // let mut file = File::create(nss_path)?;
        // let repo_location = std::env::current_dir()?;
        // file.write_all(repo_location.to_str().unwrap().as_bytes())?;

        // let pacakege_dir = std::env::current_dir().unwrap();
        // let repo_path = get_repo_path();
        


        // assert_eq!(pacakege_dir, repo_path)
    }
}







