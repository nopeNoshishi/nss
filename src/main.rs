// External
use anyhow::{bail, Result};
use std::path::PathBuf;

// Internal
pub mod cli;
pub mod struct_set;
pub mod subcommand;
pub mod util;

use cli::*;
use subcommand::*;
use util::file_system;
use util::gadget::NssRepository;

/// Parse argument and run commnad  
fn main() -> Result<()> {
    let cmd = nss_command();

    match file_system::exists_repo::<PathBuf>(None) {
        Ok(repo_path) => {
            match cmd.get_matches().subcommand() {
                // hasher
                Some(("hasher", sub_m)) => {
                    let path = sub_m.get_one::<std::path::PathBuf>("file").unwrap();

                    match sub_m.get_flag("write") {
                        true => hasher::run_option_w(NssRepository::new(repo_path), path)?,
                        _ => hasher::run(NssRepository::new(repo_path), path)?,
                    }
                }

                // ocat
                Some(("ocat", sub_m)) => {
                    let hash_path: &String = sub_m.get_one("hash").unwrap();

                    if sub_m.get_flag("pretty-print") ^ sub_m.get_flag("type") {
                        if sub_m.get_flag("pretty-print") {
                            ocat::run_option_p(NssRepository::new(repo_path), hash_path)?
                        } else {
                            ocat::run_option_t(NssRepository::new(repo_path), hash_path)?
                        }
                    } else {
                        bail!("Required one option (--type or --pprint)");
                    }
                }

                // look snap
                Some(("lk-snap", sub_m)) => match sub_m.get_flag("stage") {
                    true => lk_snap::run_option_s()?,
                    _ => lk_snap::run()?,
                },

                // update snapshot
                Some(("up-snap", sub_m)) => match sub_m.get_flag("working") {
                    true => up_snap::run_option_w(NssRepository::new(repo_path))?,
                    _ => {
                        let file_path: Option<&String> = sub_m.get_one("path");
                        match file_path {
                            Some(f) => up_snap::run(NssRepository::new(repo_path), f)?,
                            None => bail!("Required file path"),
                        }
                    }
                },

                Some(("write-tree", _)) => write_tree::run(NssRepository::new(repo_path))?,

                Some(("voyage", _)) => voyage::run(std::env::current_dir()?)?,

                Some(("snap", sub_m)) => match sub_m.get_flag("all") {
                    true => snap::shot_all(NssRepository::new(repo_path))?,
                    _ => {
                        let file_path: Option<&String> = sub_m.get_one("file");
                        match file_path {
                            Some(f) => snap::shot(NssRepository::new(repo_path), f)?,
                            None => {
                                bail!("Required file path")
                            }
                        }
                    }
                },

                Some(("reg", sub_m)) => {
                    let message: &String = sub_m.get_one("message").unwrap();
                    reg::run(NssRepository::new(repo_path), message)?
                }

                Some(("bookmark", sub_m)) => {
                    let book_name: &String = sub_m.get_one("bookmarker").unwrap();
                    let hash: Option<&String> = sub_m.get_one("hash");
                    if sub_m.get_flag("replace") {
                        bookmark::run_option_r(
                            NssRepository::new(repo_path),
                            book_name,
                            hash.unwrap(),
                        )?
                    } else {
                        bookmark::run(NssRepository::new(repo_path), book_name, hash)?
                    }
                }

                Some(("update-ref", sub_m)) => {
                    let new_commit: Option<&String> = sub_m.get_one("hash");
                    update_ref::run(NssRepository::new(repo_path), new_commit.unwrap())?
                }

                Some(("story", sub_m)) => {
                    if sub_m.get_flag("short") {
                        history::run_option_s(NssRepository::new(repo_path))?
                    } else {
                        history::run(NssRepository::new(repo_path))?
                    }
                }

                Some(("go-to", sub_m)) => {
                    let target: Option<&String> = sub_m.get_one("hash");
                    go_to::run(NssRepository::new(repo_path), target.unwrap())?
                }

                Some(("debug", _sub_m)) => {
                    use std::os::unix::fs::MetadataExt;

                    let path = repo_path.join("test");
                    let meta = path.metadata()?;
                    println!("{:b}", meta.mode());
                }

                _ => bail!("No such a commnad. Please `nss -h` to watch help."),
            }
        }

        // No repository
        Err(..) => {
            match cmd.get_matches().subcommand() {
                // voyage
                Some(("voyage", _)) => voyage::run(std::env::current_dir()?)?,
                _ => bail!("No Repository. You may start nssi voyage!"),
            }
        }
    }

    Ok(())
}
