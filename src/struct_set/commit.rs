use anyhow::Result;

use super::object::Hashable;

/// **Commit Struct**
///
/// This struct represents ...
#[derive(Debug, Clone)]
pub struct Commit {
    pub tree_hash: String,
    pub parent: String,
    pub author: String,
    pub committer: String,
    pub message: String,
}

impl Commit {
    /// Create commit with the repo tree object, config infomation and message.
    ///
    /// This tree_hash must be in the database.
    pub fn new<S: Into<String>>(
        tree_hash: S,
        parent: S,
        author: S,
        committer: S,
        message: S,
    ) -> Result<Self> {
        Ok(Self {
            tree_hash: tree_hash.into(),
            parent: parent.into(),
            author: author.into(),
            committer: committer.into(),
            message: message.into(),
        })
    }

    pub fn from_rawobject(content: &[u8]) -> Result<Self> {
        let all_line = content
            .split(|&x| x == b'\n')
            .filter(|x| x != b"")
            .map(|x| String::from_utf8(x.to_vec()).unwrap())
            .collect::<Vec<String>>();

        // "tree tree_hash"
        let mut iter = all_line[0].split_whitespace();
        iter.next();
        let tree_hash = iter.next().unwrap().to_string();

        let mut iter = all_line[1].split_whitespace();
        iter.next();
        let parent = iter.next().unwrap().to_string();

        let mut iter = all_line[2].split_whitespace();
        iter.next();
        let author = iter.next().unwrap().to_string();

        let mut iter = all_line[3].split_whitespace();
        iter.next();
        let committer = iter.next().unwrap().to_string();

        let message = all_line[4].clone();

        Ok(Self {
            tree_hash,
            parent,
            author,
            committer,
            message,
        })
    }
}

impl std::fmt::Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tree = format!("tree {}", self.tree_hash);
        let parent = match self.parent.as_str() {
            "None" => "parent None\n".to_string(),
            _ => format!("parent {}\n", self.parent),
        };
        let author = format!("author {}", self.author);
        let committer = format!("committer {}", self.committer);

        write!(
            f,
            "{}\n{}{}\n{}\n\n{}\n",
            tree, parent, author, committer, self.message
        )
    }
}

impl Hashable for Commit {
    fn as_bytes(&self) -> Vec<u8> {
        let tree_hash = format!("tree {}", self.tree_hash);
        let parent = match self.parent.as_str() {
            "None" => "parent None\n".to_string(),
            _ => format!("parent {}\n", self.parent),
        };
        let author = format!("author {}", self.author);
        let committer = format!("committer {}", self.committer);
        let content = format!(
            "{}\n{}{}\n{}\n\n{}\n",
            tree_hash, parent, author, committer, self.message
        );
        let store = format!("commit {}\0{}", content.len(), content);

        Vec::from(store.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_commit_new() {}

    #[test]
    fn test_commit_from_rawobject() {}

    #[test]
    fn test_commit_as_bytes() {}

    #[test]
    fn test_commit_display() {}
}
