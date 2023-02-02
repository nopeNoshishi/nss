#[path = "src/cli/cli.rs"]
mod cli;
use cli::nss_command;

use::clap_mangen;

fn main() -> std::io::Result<()>  {
    let cmd = nss_command();

    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?);

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("nss.1"), buffer)?;
    Ok(())
}