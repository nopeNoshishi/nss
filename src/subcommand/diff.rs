// External
use anyhow::{bail, Result};

// Internal
use nss_core::repository::NssRepository;
use nss_core::struct_set::{DIffTag, Diff, Index, Object, Tree};

pub fn run(repository: &NssRepository, target: &str, another: &str) -> Result<()> {
    // Get target index
    let tree = to_base_tree(repository, target)?;
    let target_index = Index::try_from_tree(repository, tree)?;

    // Get another index
    let tree = to_base_tree(repository, another)?;
    let another_index = Index::try_from_tree(repository, tree)?;

    let diff = target_index.diff(another_index);

    for (difftype, filename) in diff {
        match difftype {
            DIffTag::Delete => println!("D: {}", filename.to_str().unwrap()),
            DIffTag::Insert => println!("U: {}", filename.to_str().unwrap()),
            DIffTag::Replace => println!("M: {}", filename.to_str().unwrap()),
            _ => (),
        }
    }

    Ok(())
}

fn to_base_tree(repository: &NssRepository, target: &str) -> Result<Tree> {
    let commit = match repository.read_object(target)? {
        Object::Commit(c) => c,
        _ => bail!("{} is not commit hash", target),
    };

    // target commit hash needs to have tree hash
    match repository.read_object(&commit.tree_hash)? {
        Object::Tree(t) => Ok(t),
        _ => bail!("{} is not tree hash", &commit.tree_hash),
    }
}
