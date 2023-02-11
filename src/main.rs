// External
use anyhow::{Result, bail};

// Internal
pub mod cli;
pub mod subcommand;
pub mod struct_set;
pub mod util;

use cli::*;
use subcommand::*;
use util::file_system;

/// Parse argument and run commnad  
fn main() -> Result<()> {

    let cmd = nss_command();    

    match cmd.get_matches().subcommand() {
        // hasher
        Some(("hasher", sub_m)) => {
            let path = sub_m.get_one::<std::path::PathBuf>("file").unwrap();
            
            match sub_m.get_flag("write") {
                true => { 
                    file_system::exists_repo(None)?;
                    hasher::run_option_w(path)?
                }
                _ => hasher::run(path)?
            }
        }

        // ocat
        Some(("ocat", sub_m)) => {
            file_system::exists_repo(None)?;
            let hash_path: &String = sub_m.get_one("hash").unwrap();

            if sub_m.get_flag("pretty-print") ^ sub_m.get_flag("type") {
                if sub_m.get_flag("pretty-print") {
                    ocat::run_option_p(hash_path)?
                } else {
                    ocat::run_option_t(hash_path)?
                }
            } else {
                bail!("Required one option (--type or --pprint)");
            }
        }

        // look snap
        Some(("lk-snap", sub_m)) => {
            file_system::exists_repo(None)?;
            match sub_m.get_flag("stage") {
                true => lk_snap::run_option_s()?,
                _ => lk_snap::run()?
            }
        }

        // update snapshot
        Some(("up-snap", sub_m)) => {
            file_system::exists_repo(None)?;
            match sub_m.get_flag("working") {
                true => up_snap::run_option_w()?,
                _ => {
                    let file_path: Option<&String>  = sub_m.get_one("path");
                    match file_path {
                        Some(f) => up_snap::run(f)?,
                        None => bail!("Required file path")
                    }
                }
            }
        }

        Some(("write-tree", _)) => {
            file_system::exists_repo(None)?;
            write_tree::run()?
        }

        Some(("voyage", _)) => {
            voyage::run()?
        }

        Some(("snap", sub_m)) => {
            file_system::exists_repo(None)?;
            match sub_m.get_flag("all") {
                true => snap::shot_all()?,
                _ => {
                    let file_path: Option<&String>  = sub_m.get_one("file");
                    match file_path {
                        Some(f) => snap::shot(f)?,
                        None => {
                            bail!("Required file path") }
                    }
                }
            }
        }

        Some(("reg", sub_m)) => {
            file_system::exists_repo(None)?;
            let message: &String = sub_m.get_one("message").unwrap();
            reg::run(message)?
        }

        Some(("bookmark", sub_m)) => {
            file_system::exists_repo(None)?;
            let book_name:&String = sub_m.get_one("bookmarker").unwrap();
            let hash: Option<&String> = sub_m.get_one("hash");
            if sub_m.get_flag("replace") {
                bookmark::run_option_r(book_name, hash.unwrap())?
            } else {
                bookmark::run(book_name, hash)?
            }
        }

        Some(("update-ref", sub_m)) => {
            file_system::exists_repo(None)?;
            let new_commit: Option<&String> = sub_m.get_one("hash");
            update_ref::run(new_commit.unwrap())?
        }

        Some(("story", sub_m)) => {
            file_system::exists_repo(None)?;
            if sub_m.get_flag("short") {
                history::run_option_s()?
            } else {
                history::run()?
            }
        }

        Some(("go-to", sub_m)) => {
            file_system::exists_repo(None)?;
            let target: Option<&String> = sub_m.get_one("hash");
            go_to::run(target.unwrap())?
        }

        Some(("debug", _sub_m)) => {
            use std::os::unix::fs::MetadataExt;
            use crate::util::gadget;

            let repopath = gadget::get_repo_path()?;
            let path = repopath.join("test");
            let meta = path.metadata()?;
            println!("{:b}", meta.mode());
        }

        _ => bail!("No such a commnad. Please `nss -h` to watch help."),
    }

    Ok(())
}
