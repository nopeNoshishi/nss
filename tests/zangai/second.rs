fn main() -> std::io::Result<()> {
    let cmd = Command::new("ngit")
        .about("This is Original Git") 
        .version("0.1.0")
        .author("Noshishi. <noshishi@noshishi.com>")
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommand(Command::new("add")
            .about("Snapshot latest working directory")
            .arg(Arg::new("file")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(false)
            .value_name("file")))
        .subcommand(Command::new("commit")
            .about("Register snapshot(tree object) as commit object in local repository")
            .arg(Arg::new("message")
            .short('m')
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .help("Add message to commit object ")
            .required(true)));

    match cmd.get_matches().subcommand() {
        Some(("add", sub_m)) => {
            let filename: Option<&String>  = sub_m.get_one("file");
            match filename {
                Some(f) => add(f)?,
                None => panic!("Required file path"),
            }
        },
        Some(("commit", sub_m)) => {
            let message: Option<&String> = sub_m.get_one("message");
            match message {
                Some(m) => commit(m)?,
                None => panic!("Required message"),
            }
        },
        _ => {},
    }
    
    Ok(())
}