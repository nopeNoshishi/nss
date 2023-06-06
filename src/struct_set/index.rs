// Std
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

// External
use anyhow::{bail, Result};
use byteorder::{BigEndian, ByteOrder};

// Internal
use super::{FileMeta, Object, Tree};
use crate::util::{file_system, gadget};

/// TODO: Documentation
#[derive(Debug, Clone)]
pub struct Index {
    pub filemetas: Vec<FileMeta>,
}

impl Index {
    pub fn empty() -> Self {
        Self { filemetas: vec![] }
    }

    pub fn new_all() -> Result<Self> {
        let repo_path = gadget::get_repo_path()?;
        let mut all_paths = gadget::get_all_paths_ignore(repo_path);
        all_paths.sort();

        let filemetas = all_paths
            .iter()
            .map(|path| FileMeta::new(path).unwrap())
            .collect::<Vec<_>>();

        Ok(Self { filemetas })
    }

    pub fn from_rawindex() -> Result<Self> {
        let index_path = gadget::get_index_path()?;

        let mut file = File::open(index_path)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;

        if buf == Vec::<u8>::new() {
            bail!("First index");
        }

        let entry_num = BigEndian::read_u32(&buf[8..12]) as usize;
        let mut start_size = 12_usize;
        let mut filemetas: Vec<FileMeta> = vec![];
        for _ in 0..entry_num {
            let name_size =
                BigEndian::read_u16(&buf[(start_size + 60)..(start_size + 62)]) as usize;
            filemetas.push(FileMeta::from_rawindex(
                &buf[(start_size)..(start_size + 62 + name_size)],
            ));

            let padding_size = padding(name_size);
            start_size = start_size + 62 + name_size + padding_size;
        }

        Ok(Self { filemetas })
    }

    pub fn add(&mut self, file_path: &PathBuf) -> Result<()> {
        let repo_path = gadget::get_repo_path()?;
        let add_filemeta = FileMeta::new(&repo_path.join(file_path))?;

        let mut new_filemetas: Vec<FileMeta> = vec![];
        for filemeta in self.filemetas.clone() {
            if filemeta == add_filemeta {
                continue;
            } else {
                new_filemetas.push(filemeta);
            }
        }
        new_filemetas.push(add_filemeta);
        new_filemetas.sort_by(|a, b| b.filename.cmp(&a.filename));
        self.filemetas = new_filemetas;

        Ok(())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let index_header = b"DIRC";
        let index_version = 1_u32;
        let entry_num = self.filemetas.len() as u32;
        let header = [
            *index_header,
            index_version.to_be_bytes(),
            entry_num.to_be_bytes(),
        ]
        .concat();

        let mut filemetas_vec: Vec<Vec<u8>> = vec![];
        for filemeta in &self.filemetas {
            let len = 62 + filemeta.filename_size as usize;
            let padding = (0..(8 - len % 8)).map(|_| b'\0').collect::<Vec<u8>>();
            let filemeta_vec = [filemeta.as_bytes(), padding].concat();

            filemetas_vec.push(filemeta_vec)
        }

        [header, filemetas_vec.concat()].concat()
    }
}

fn padding(size: usize) -> usize {
    // calclate padding size
    let floor = (size - 2) / 8;
    let target = (floor + 1) * 8 + 2;

    target - size
}

impl TryFrom<Tree> for Index {
    type Error = anyhow::Error;

    fn try_from(tree: Tree) -> Result<Self, anyhow::Error> {
        let mut index = Index::empty();
        let mut paths: Vec<PathBuf> = vec![];

        let repo_path = gadget::get_repo_path()?;
        push_paths(&mut paths, tree, repo_path)?;

        for file_path in paths {
            index.add(&file_path)?
        }

        Ok(index)
    }
}

fn push_paths(paths: &mut Vec<PathBuf>, tree: Tree, base_path: PathBuf) -> Result<()> {
    for entry in tree.entries {
        let path = base_path.join(entry.name);
        if path.is_file() {
            paths.push(path);
        } else {
            let hash = hex::encode(entry.hash);
            let sub_tree = to_tree(&hash)?;

            push_paths(paths, sub_tree, path)?
        }
    }

    Ok(())
}

fn to_tree(hash: &str) -> Result<Tree, anyhow::Error> {
    let raw_content = file_system::read_object(hash)?;

    match Object::from_content(raw_content)? {
        Object::Tree(t) => Ok(t),
        _ => bail!("{} is not tree hash", hash),
    }
}
