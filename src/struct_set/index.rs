// Std
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

// External
use anyhow::{Result, bail};
use byteorder::{ByteOrder, BigEndian};

// Internal
use super::FileMeta;
use crate::util::gadget;

/// TODO: Documentation
#[derive(Debug, Clone)]
pub struct Index {
    pub filemetas: Vec<FileMeta>,
}

impl Index {
    pub fn new() -> Result<Self> {
        let repo_path = gadget::get_repo_path()?;
        let mut all_paths = gadget::get_all_paths_ignore(&repo_path);
        all_paths.sort();

        let filemetas = all_paths.iter()
            .map(|path| FileMeta::new(path).unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            filemetas
        })
    }

    pub fn from_path(file_path: &str) -> Result<Self> {
        let repo_path = gadget::get_repo_path()?;
        let path = repo_path.join(file_path);

        let filemeta = FileMeta::new(&path)?;

        Ok(Self {
            filemetas: vec![filemeta]
        })
    }

    pub fn from_rawindex() -> Result<Self> {
        let index_path = gadget::get_index_path()?;
    
        let mut file = File::open(index_path)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;

        if buf == vec![] {
            bail!("First index");
        }

        let entry_num = BigEndian::read_u32(&buf[8..12]) as usize;
        let mut start_size = 12 as usize;
        let mut filemetas: Vec<FileMeta> = vec![];
        for _ in 0..entry_num {
            let name_size = BigEndian::read_u16(&buf[(start_size+60)..(start_size+62)]) as usize;
            filemetas.push(FileMeta::from_rawindex(&buf[(start_size)..(start_size+62+name_size)]));

            let padding_size = padding(name_size);
            start_size = start_size + 62 + name_size + padding_size;   
        }

        Ok(Self {
            filemetas: filemetas
        })
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
        let index_version = 1 as u32;
        let entry_num = self.filemetas.len() as u32; 
        let header = [*index_header, index_version.to_be_bytes(), entry_num.to_be_bytes()].concat();      
    
        let mut filemetas_vec:Vec<Vec<u8>> = vec![];
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
    let padding = target - size;

    padding
}

pub struct IndexDirectory {
    pub file_paths: Vec<PathBuf>,
    pub dir_paths: Vec<PathBuf>,
}

impl IndexDirectory {
    pub fn new(index: Index) -> Result<Self> {
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
        dir_paths.reverse();

        file_paths.sort();
        file_paths.reverse();

        println!("{:?}", dir_paths);
        println!("{:?}", file_paths);

        Ok(Self { 
            file_paths,
            dir_paths 
        })
    }
}
