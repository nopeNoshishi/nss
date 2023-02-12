//! **Reg command** ... Base command: `git commit` and `git commit-tree`

// Std
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::prelude::*;
use std::collections::HashMap;

// External
use anyhow::{Result, bail};
use colored::*;

// Internal
use crate::struct_set::{Index, Entry, Tree, Commit, Hashable};
use crate::util::{gadget, file_system};

pub fn run(massage: &str) -> Result<()> {

    // Create tree object from index
    let hash = write_tree()?;

    // Read head hash
    let head_hash = match head_hash()? {
        Some(h) => h,
        _ => "None".to_owned()
    };

    // Build commit object
    let commit = Commit::new(  
        hash,
        head_hash,
        "nopeNoshishi".to_string(),
        "nopeNoshishi@nope.noshishi".to_string(),
        massage.to_string()
    )?;

    // Write commit object
    let hash = hex::encode(commit.to_hash());
    file_system::write_commit(&hash, commit.clone())?;
    
    display_result(commit.parent.as_str(), hash.as_str())?;

    Ok(())
}

fn display_result(old_hash: &str, new_hash: &str) -> Result<()> {
    match old_hash {
        "None" => {
            println!("{}: {} --> {}: {}",
            format!("OLD").bright_blue(),
            &old_hash,
            format!("NEW").bright_yellow(),
            &new_hash[0..7]);

            let book_path = read_head()?;
            let bookmarker = book_path.split('/').collect::<Vec<&str>>()[2];
            update_bookmark(bookmarker, new_hash, None)?;
        },
        _ => {
            println!("{}: {} --> {}: {}",
            format!("OLD").bright_blue(),
            &old_hash[0..7],
            format!("NEW").bright_yellow(),
            &new_hash[0..7]);

            let book_path = read_head()?;
            let bookmarker = book_path.split('/').collect::<Vec<&str>>()[2];
            update_bookmark(bookmarker, new_hash, Some(old_hash))?;
        }
    }

    Ok(())
}

fn read_head() -> Result<String> {
    let head_path = gadget::get_head_path()?;

    let mut file = File::open(head_path)?;
    let mut referece = String::new();
    file.read_to_string(&mut referece).unwrap();

    let prefix_path = referece.split(' ').collect::<Vec<&str>>();

    return Ok(prefix_path[1].to_string())
}

fn head_hash() -> Result<Option<String>> {

    let head_item = read_head()?;
    if head_item.contains("/") {
        let bookmarker = head_item.split('/').collect::<Vec<&str>>()[2];
        let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

        let mut file = File::open(bookmark_path).unwrap();
        let mut hash = String::new();
        file.read_to_string(&mut hash).unwrap();

        if hash == "".to_string() {
            return Ok(None)
        }

        return Ok(Some(hash))
    }

    Ok(Some(head_item))
}

fn update_bookmark(bookmarker: &str, new_commit: &str, old_commit: Option<&str>) -> Result<()> {
    let bookmark_path = gadget::get_bookmarks_path(bookmarker)?;

    let raw_content = file_system::read_object(new_commit)?;
    if String::from_utf8(raw_content[0..1].to_vec()).unwrap() == String::from("c") {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(bookmark_path)?;
        let mut bookmark_hash = String::new();
        file.read_to_string(&mut bookmark_hash)?;

        if bookmark_hash == "".to_owned() {
            file.write_all(new_commit.as_bytes())?;
        } else if bookmark_hash == old_commit.unwrap().to_owned() {
            file.write_all(new_commit.as_bytes())?;
        } else {
            bail!("This bookmarker has the differnce old hash ({})", bookmark_hash)
        }
    } else {
        bail!("Not commit hash <new commit> ({})", new_commit)
    }

    Ok(())
}

fn write_tree() -> Result<String>{
    let index = Index::from_rawindex()?;
    let tree_dir = tree_map(index)?;

    let mut repo_tree_hash = String::new();
    let mut dir_entry_map: HashMap<PathBuf, Entry> = HashMap::new();
    for m in tree_dir {

        let mut entries: Vec<Entry> = vec![];


        for path in m.1 {
            if path.is_file() {
                let entry = Entry::new(path)?;
                entries.push(entry)
            } else {
                let entry = dir_entry_map.get(&path).unwrap().to_owned();
                entries.push(entry)
            }
        }

        let dir_entry = Entry::new_group(&m.0, entries.clone())?;
        dir_entry_map.insert(m.0.to_path_buf(), dir_entry);

        let tree = Tree::from_entries(entries);
        let hash = hex::encode(tree.to_hash());
        file_system::write_tree(&hash, tree)?;

        if m.0 == gadget::get_repo_path()? {
            repo_tree_hash = hash
        }
    }    

    Ok(repo_tree_hash)
}


fn tree_map(index: Index) -> Result<Vec<(PathBuf,Vec<PathBuf>)>> {
    let mut file_paths: Vec<PathBuf> = vec![];
    let mut dir_paths: Vec<PathBuf> = vec![];
    for filemeta in index.clone().filemetas {
        let repo_path = gadget::get_repo_path()?;
        let file_path = repo_path.join(filemeta.filename);
        let mut dir_name = file_path.parent().unwrap().to_path_buf();

        file_paths.push(repo_path.join(file_path));
        dir_paths.push(repo_path.clone());

        while dir_name != repo_path {
            dir_paths.push(repo_path.join(dir_name.clone()));

            dir_name = dir_name.parent().unwrap().to_path_buf();
        }
    };
    dir_paths.sort();
    dir_paths.dedup();


    let mut temp_map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    for dir in &dir_paths {
        temp_map.insert(dir.to_path_buf(), vec![]);

        for file in &file_paths {
            if dir == &file.parent().unwrap().to_path_buf() {
                temp_map.get_mut(&dir.to_path_buf()).unwrap().push(file.to_path_buf())
            } 
        }

        for sub_dir in &dir_paths {
            if dir == &sub_dir.parent().unwrap().to_path_buf() {

                temp_map.get_mut(&dir.to_path_buf()).unwrap().push(sub_dir.to_path_buf())
            }
        }
    }

    let mut tmp: Vec<(&PathBuf, &Vec<PathBuf>)> = temp_map.iter().collect();
    tmp.sort_by(|b, a| {
        let comp_a: Vec<&std::ffi::OsStr> = a.0.iter().collect();
        let comp_b: Vec<&std::ffi::OsStr> = b.0.iter().collect();
        if comp_a.len() >= comp_b.len() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    let mut tree_dir: Vec<(PathBuf, Vec<PathBuf>)> = vec![];
    for t in tmp {
        tree_dir.push((t.0.to_owned(), t.1.to_vec()))
    }

    Ok(tree_dir)
}