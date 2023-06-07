//! Repository addresser

use std::path::PathBuf;

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
