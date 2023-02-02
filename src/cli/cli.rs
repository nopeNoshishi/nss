use clap::{Command, Arg, ArgAction};

pub fn nss_command() -> clap::Command {

    let help = "This is Original Version Management System.
Learn git and rust for good developer.

Usage: nssi [COMMAND]

Main Commands:
    voyage        Create your current dirctory into nss repository
    snap          Snapshot latest working directory
    reg           Register snapshot(tree object) as commit object in local repository
    bookmark      Cretae or replace bookmarker for specific commit
    story         View commit history from a cuurent commit
    go-to         Go to the commit and change the working directory and index

Deep Commnads:
    hasher        Calclate object hash or Create object
    ocat          Search the specified hash value in the repositor,
                    and if the hash value is available, display the result.
    lk-snap       Look the snapshot and display file mtadata
    up-snap       Register file contents in the working diretory to the index
    write-tree    Create main tree object by index
    update-ref    Move HEAD pointer to the commit

Develop Commands:
    debug         Try debug

Support Commands:
    help          Print this message or the help of the given subcommand(s)

Options:
    -h, --help     Print help information
    -V, --version  Print version information";

    Command::new("nss")
        .about("This is Original Version Management System.\nLearn git and rust for good developer.")
        .version("0.1.0")
        .author("Noshishi. <noshishi@noshishi.com>")
        .override_help(help)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommands(vec![
                                 //main command  
                                  voyage_command(), snap_command(),reg_command(),  bookemark_command(),
                                  history_command(), goto_command(),])
        .subcommands(vec![
                                  // deep command
                                  hasher_command(), ocat_command(), look_command(), index_command(),
                                  write_command(),  ref_command(),])
        .subcommands(vec![
                                  // development command
                                  debug_command()])
}

/// Usage:
/// 
/// ```
/// nss hasher <file>
/// ```
/// 
/// ```
/// nss hasher -w <file>
/// ```
fn hasher_command() -> clap::Command {
    Command::new("hasher")
            .about("Calclate object hash or Create object")
            .arg(Arg::new("write")
                .short('w')
                .long("write")
                .action(ArgAction::SetTrue)
                .help("file to blob Object"))
            .arg(Arg::new("file")
                .value_parser(clap::builder::PathBufValueParser::new())
                .help("..file relative path against repo")
                .value_name("file")
                .required(true)
                )
}


/// Usage:
/// ```
/// nss cat -p <object hash>
/// ```
/// 
/// ```
/// nss cat -t <object hash>
/// ```
fn ocat_command() -> clap::Command {
    Command::new("ocat")
        .about("Search the specified hash value in the repositor,\nand if the hash value is available, display the result.")
        .override_usage("\n\tnss ocat (-p | -t) <object hash>")
            .arg(Arg::new("pretty-print")
                .short('p')
                .long("pprint")
                .action(ArgAction::SetTrue)
                .help("Output the object content on command line"))
            .arg(Arg::new("type")
                .short('t')
                .long("type")
                .action(ArgAction::SetTrue)
                .help("Output the object type on command line"))
            .arg(Arg::new("hash")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("... This object must be stored in the repository")
                .required(true)
                .value_name("hash value"))
}


fn look_command() -> clap::Command {
    Command::new("lk-snap")
        .about("Look the snapshot and display file mtadata")
            .arg(Arg::new("stage")
                .short('s')
                .long("stage")
                .action(ArgAction::SetTrue)
                .help("Show detail data"))
}

fn index_command() -> clap::Command {
    Command::new("up-sanp")
        .about("Register file contents in the working diretory to the index")
            .arg(Arg::new("working")
                .short('v')
                .long("view")
                .action(ArgAction::SetTrue)
                .help("Only view list of files to be tracked"))
            .arg(Arg::new("path")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .value_name("path"))
}

fn write_command() -> clap::Command {
    Command::new("write-tree")
        .about("Create main tree object by index")
}

fn voyage_command() -> clap::Command {
    Command::new("voyage")
        .about("Create your current dirctory into nss repository")
}

fn snap_command() -> clap::Command {
    Command::new("snap")
        .about("Snapshot latest working directory")
            .arg(Arg::new("all")
                .short('A')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Snapshot all tracking files"))
            .arg(Arg::new("file")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .value_name("file"))
}

fn reg_command() -> clap::Command {
    Command::new("reg")
        .about("Register snapshot(tree object) as commit object in local repository")
            .arg(Arg::new("message")
                .short('m')
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Add message to commit object ")
                .required(true))
}

fn bookemark_command() -> clap::Command {
    Command::new("bookmark")
        .about("Cretae or replace bookmarker for specific commit")
            .arg(Arg::new("replace")
                .short('r')
                .long("replace")
                .action(ArgAction::SetTrue)
                .help("Replace already existing bookmarks to another commit"))
            .arg(Arg::new("bookmarker")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("... name (bookmarker name) to identify the commit")
                .value_name("booknaker")
                .required(true))
            .arg(Arg::new("hash")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("... commit to leaving a bookmark")
                .value_name("commit hash")
                .required(false))
}

fn ref_command() -> clap::Command {
    Command::new("update-ref")
        .about("Move HEAD pointer to the commit")
            .arg(Arg::new("hash")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("...This object must be stored in the repository as commit object")
                .required(true)
                .value_name("hash value"))
}

fn history_command() -> clap::Command {
    Command::new("story")
        .about("View commit history from a cuurent commit")
}

fn goto_command() -> clap::Command {
    Command::new("go-to")
        .about("Go to the commit and change the working directory and index")
            .arg(Arg::new("hash")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("...This object must be stored in the repository as commit object")
                .required(true)
                .value_name("hash value"))
}

fn debug_command() -> clap::Command {
    Command::new("debug")
        .about("Try debug")
}
