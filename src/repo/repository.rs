//! Repository addresser
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

// External
use anyhow::{bail, Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::struct_set::{Object, Index, Hashable};
use crate::nss_io::file_system;

// Manager for repository absolute path
#[derive(Debug, Clone)]
pub struct NssRepository {
    root: PathBuf,
}

impl NssRepository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn path(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn local_path(&self) -> PathBuf {
        self.root.clone().join(".nss")
    }

    pub fn objects_path<T: Into<String>>(&self, hash: T) -> PathBuf {
        let hash = hash.into();
        let (dir, file) = hash.split_at(2);

        self.root
            .clone()
            .join(".nss")
            .join("objects")
            .join(dir)
            .join(file)
    }

    pub fn bookmarks_path(&self, bookmarker: &str) -> PathBuf {
        self.root
            .clone()
            .join(".nss")
            .join("bookmarks")
            .join("local")
            .join(bookmarker)
    }

    pub fn config_path(&self) -> PathBuf {
        self.root.clone().join(".nss").join("config")
    }

    pub fn head_path(&self) -> PathBuf {
        self.root.clone().join(".nss").join("HEAD")
    }

    pub fn index_path(&self) -> PathBuf {
        self.root.clone().join(".nss").join("INDEX")
    }

    pub fn read_index(&self) -> Result<Index> {
        // read index
        let mut file = File::open(self.index_path())
            .with_context(|| "Index doesn't exit in this repository")?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .with_context(|| "Index can't be read")?;

        Index::from_rawindex(bytes)
    }

    pub fn write_object<H>(&self, object: H) -> Result<()> 
    where
        H: Hashable
    {
        let object_path = self.objects_path(hex::encode(object.to_hash()));
        file_system::create_dir(object_path.parent().unwrap())?;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&object.as_bytes())?;
        let compressed = encoder.finish()?;

        let mut file = File::create(object_path)?;
        file.write_all(&compressed)?;
        file.flush().unwrap();

        Ok(())
    }

    pub fn read_object<S>(&self, hash: S) -> Result<Object>
    where
        S: AsRef<str>,
    {
        let hash_path = self.try_get_objects_path(hash.as_ref())?;

        // read objectz
        let mut file = File::open(hash_path)
            .with_context(|| format!("{} doesn't exit in this repository", hash.as_ref()))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .with_context(|| format!("{} content can't read", &hash.as_ref()))?;

        // decode content
        let mut decoder = ZlibDecoder::new(&bytes[..]);
        let mut object_content: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut object_content)?;

        Object::from_content(object_content)
    }

    /// Return your object database **absolutely** path
    pub fn try_get_objects_path<T: Into<String>>(&self, hash: T) -> Result<PathBuf> {
        let hash = hash.into();

        if hash.len() < 6 {
            bail!("More hash value digit (less digit)")
        }

        let (dir, file) = hash.split_at(2);
        let object_dir = &self.path().join(".nss").join("objects").join(dir);

        let mut paths: Vec<PathBuf> = vec![];
        self.ext_paths(object_dir, &mut paths)?;

        let mut target_files: Vec<PathBuf> = vec![];
        for path in paths {
            if path.as_os_str().to_string_lossy().contains(file) {
                target_files.push(path)
            }
        }

        if target_files.len() > 2 {
            bail!("More hash value digit (nearly hash value exists)")
        } else if target_files.is_empty() {
            bail!("Doesn't exit in this repository")
        }

        Ok(object_dir.join(&target_files[0]))
    }

    pub fn ext_paths<P: AsRef<Path>>(&self, target: P, paths: &mut Vec<PathBuf>) -> Result<()> {
        // Print all files in target directory
        let files = target
            .as_ref()
            .read_dir()
            .with_context(|| format!("{:?} object database has no objects", target.as_ref()))?;

        for dir_entry in files {
            let path = dir_entry.unwrap().path();
            paths.push(path);
        }
        paths.sort();

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_all_paths(&self, target: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut paths = vec![];
        self.ext_paths(target, paths.as_mut())?;

        Ok(paths)
    }

    pub fn get_all_paths_ignore<P: AsRef<Path>>(&self, target: P) -> Vec<PathBuf> {
        let mut paths = vec![];
        self.ext_paths_ignore(target, paths.as_mut());

        paths
    }

    pub fn ext_paths_ignore<P: AsRef<Path>>(&self, target: P, paths: &mut Vec<PathBuf>) {
        // Print all files in target directory
        let files = target.as_ref().read_dir().unwrap();

        let mut ignore_paths: Vec<PathBuf> = vec![];

        // Check .nssignore file
        match fs::read_to_string(".nssignore") {
            Ok(content) => {
                let lines = content.lines();
                ignore_paths.extend(lines.into_iter().filter(|line| !line.contains('#') || line.is_empty()).map(|line| self.path().join(line)));
            },
            Err(..) => println!("You may set ignore file.")
        }
        
        // Program ignore folder
        ignore_paths.extend(vec![
            self.path().join(".git"),
            self.path().join(".nss"),
            self.path().join(".gitignore"),
            self.path().join(".nssignore"),
        ]);

        for dir_entry in files {
            let path = dir_entry.unwrap().path();

            let mut do_ignore = false;
            for ignore_path in ignore_paths.clone() {
                if path == ignore_path {
                    do_ignore = true
                }
            }

            if do_ignore {
                continue;
            }

            if path.is_dir() {
                self.ext_paths_ignore(&path, paths);
                continue;
            }
            paths.push(path);
        }
        paths.sort();
}

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_nss_repository() {
        let temp_dir = env::temp_dir();
        let repository = NssRepository::new(temp_dir.clone());

        assert_eq!(repository.path(), temp_dir);
        assert_eq!(repository.local_path(), temp_dir.join(".nss"));
        assert_eq!(
            repository.objects_path("3e46eb7dbc405630832193193cf17385f29cb243"),
            temp_dir
                .join(".nss")
                .join("objects")
                .join("3e")
                .join("46eb7dbc405630832193193cf17385f29cb243")
        );
        assert_eq!(
            repository.bookmarks_path("test"),
            temp_dir
                .join(".nss")
                .join("bookmarks")
                .join("local")
                .join("test")
        );
        assert_eq!(
            repository.config_path(),
            temp_dir.join(".nss").join("config")
        );
        assert_eq!(repository.head_path(), temp_dir.join(".nss").join("HEAD"));
        assert_eq!(repository.index_path(), temp_dir.join(".nss").join("INDEX"));
    }
}
