use clap::{Arg, ArgAction, Command};

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

    Command::new("nssi")
        .about(
            "This is Original Version Management System.\nLearn git and rust for good developer.",
        )
        .version(env!("CARGO_PKG_VERSION"))
        .author("Noshishi. <noshishi@noshishi.com>")
        .override_help(help)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .subcommands(vec![
            //main command
            voyage_command(),
            snap_command(),
            reg_command(),
            bookemark_command(),
            history_command(),
            goto_command(),
        ])
        .subcommands(vec![
            // deep command
            hasher_command(),
            ocat_command(),
            look_command(),
            index_command(),
            write_command(),
            ref_command(),
        ])
        .subcommands(vec![
            // development command
            debug_command(),
        ])
}

fn hasher_command() -> clap::Command {
    Command::new("hasher")
        .about("Calclate object hash or Create object")
        .arg(
            Arg::new("write")
                .short('w')
                .long("write")
                .action(ArgAction::SetTrue)
                .help("file to blob Object"),
        )
        .arg(
            Arg::new("file")
                .value_parser(clap::builder::PathBufValueParser::new())
                .help("..file relative path against repo")
                .value_name("file")
                .required(true),
        )
}

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
        .arg(
            Arg::new("stage")
                .short('s')
                .long("stage")
                .action(ArgAction::SetTrue)
                .help("Show detail data"),
        )
}

fn index_command() -> clap::Command {
    Command::new("up-sanp")
        .about("Register file contents in the working diretory to the index")
        .arg(
            Arg::new("working")
                .short('v')
                .long("view")
                .action(ArgAction::SetTrue)
                .help("Only view list of files to be tracked"),
        )
        .arg(
            Arg::new("path")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .value_name("path"),
        )
}

fn write_command() -> clap::Command {
    Command::new("write-tree").about("Create main tree object by index")
}

fn voyage_command() -> clap::Command {
    Command::new("voyage").about("Create your current dirctory into nss repository")
}

fn snap_command() -> clap::Command {
    Command::new("snap")
        .about("Snapshot latest working directory")
        .arg(
            Arg::new("all")
                .short('A')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Snapshot all tracking files"),
        )
        .arg(
            Arg::new("file")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(false)
                .value_name("file"),
        )
}

fn reg_command() -> clap::Command {
    Command::new("reg")
        .about("Register snapshot(tree object) as commit object in local repository")
        .arg(
            Arg::new("message")
                .short('m')
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Add message to commit object ")
                .required(true),
        )
}

fn bookemark_command() -> clap::Command {
    Command::new("bookmark")
        .about("Cretae or replace bookmarker for specific commit")
        .arg(
            Arg::new("replace")
                .short('r')
                .long("replace")
                .action(ArgAction::SetTrue)
                .help("Replace already existing bookmarks to another commit"),
        )
        .arg(
            Arg::new("bookmarker")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("... name (bookmarker name) to identify the commit")
                .value_name("booknaker")
                .required(true),
        )
        .arg(
            Arg::new("hash")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("... commit to leaving a bookmark")
                .value_name("commit hash")
                .required(false),
        )
}

fn ref_command() -> clap::Command {
    Command::new("update-ref")
        .about("Move HEAD pointer to the commit")
        .arg(
            Arg::new("hash")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("...This object must be stored in the repository as commit object")
                .required(true)
                .value_name("hash value"),
        )
}

fn history_command() -> clap::Command {
    Command::new("story")
        .about("View commit history from a cuurent commit")
        .arg(
            Arg::new("short")
                .short('s')
                .long("short")
                .action(ArgAction::SetTrue)
                .help("Short history"),
        )
}

fn goto_command() -> clap::Command {
    Command::new("go-to")
        .about("Go to the commit and change the working directory and index")
        .arg(
            Arg::new("hash")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("...This object must be stored in the repository as commit object")
                .required(true)
                .value_name("hash value"),
        )
}

fn debug_command() -> clap::Command {
    Command::new("debug").about("Try debug")
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::*;
    use std::path::PathBuf;

    #[test]
    fn test_nss_command() {
        let mut cmd = nss_command();

        let res = cmd.try_get_matches_from_mut(vec!["nssi"]);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().kind(),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );

        let res = cmd.try_get_matches_from_mut(vec!["nssi", "test"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidSubcommand);

        let res = cmd.try_get_matches_from_mut(vec!["nssi", "debug"]);
        assert!(res.is_ok());

        let res = cmd.try_get_matches_from_mut(vec!["nssi", "-h"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::DisplayHelp);

        let res = cmd.try_get_matches_from_mut(vec!["nssi", "-V"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::DisplayVersion);
    }

    #[test]
    fn test_hasher_command() {
        let mut cmd = hasher_command();

        let res = cmd.try_get_matches_from_mut(vec!["hasher"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

        let mut res = cmd.try_get_matches_from_mut(vec!["hasher", "first.txt"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut().unwrap().get_one::<PathBuf>("file").unwrap(),
            &PathBuf::from("first.txt")
        );
        assert_ne!(
            res.as_mut().unwrap().get_one::<PathBuf>("file").unwrap(),
            &PathBuf::from("second.txt")
        );
        assert!(res
            .as_mut()
            .unwrap()
            .try_get_one::<String>("value")
            .is_err());

        assert!(!res.as_mut().unwrap().get_flag("write"));

        let mut res = cmd.try_get_matches_from_mut(vec!["hasher", "-w", "first.txt"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut().unwrap().get_one::<PathBuf>("file").unwrap(),
            &PathBuf::from("first.txt")
        );
        assert_ne!(
            res.as_mut().unwrap().get_one::<PathBuf>("file").unwrap(),
            &PathBuf::from("second.txt")
        );
        assert!(res
            .as_mut()
            .unwrap()
            .try_get_one::<String>("value")
            .is_err());
        assert!(res.as_mut().unwrap().get_flag("write"));
    }

    #[test]
    fn test_ocat_command() {
        let mut cmd = ocat_command();

        // No hash value
        let res = cmd.try_get_matches_from_mut(vec!["ocat"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

        // Get hash value
        let mut res = cmd.try_get_matches_from_mut(vec!["ocat", "jfaf7GATG7ya"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "jfaf7GATG7ya"
        );
        assert_ne!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "6fawfwK234412"
        );

        // // Get no setup value
        assert!(res.as_mut().unwrap().try_get_one::<String>("test").is_err());

        // Get hash value with -p option
        let mut res = cmd.try_get_matches_from_mut(vec!["ocat", "jfaf7GATG7ya", "-p"]);

        assert!(res.is_ok());
        assert!(res.as_mut().unwrap().get_flag("pretty-print"));

        // Get hash value with -t option
        let mut res = cmd.try_get_matches_from_mut(vec!["ocat", "jfaf7GATG7ya", "-t"]);

        assert!(res.is_ok());
        assert!(res.as_mut().unwrap().get_flag("type"));
    }

    #[test]
    fn test_look_command() {
        let mut cmd = look_command();

        // No option
        let res = cmd.try_get_matches_from_mut(vec!["lk-snap"]);
        assert!(res.is_ok());

        // Not exepected value
        let res = cmd.try_get_matches_from_mut(vec!["lk-snap", "jfaf7GATG7ya"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);

        // Run with -s option
        let mut res = cmd.try_get_matches_from_mut(vec!["lk-snap", "-s"]);

        assert!(res.is_ok());
        assert!(res.as_mut().unwrap().get_flag("stage"));
    }

    #[test]
    fn test_index_command() {
        let mut cmd = index_command();

        let res = cmd.try_get_matches_from_mut(vec!["up-snap"]);
        assert!(res.is_ok());

        let mut res = cmd.try_get_matches_from_mut(vec!["up-snap", "first.txt"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut().unwrap().get_one::<String>("path").unwrap(),
            "first.txt"
        );
        assert_ne!(
            res.as_mut().unwrap().get_one::<String>("path").unwrap(),
            "6fawfwK234412"
        );
        assert!(res.as_mut().unwrap().try_get_one::<String>("test").is_err());

        // Run with -v option
        let mut res = cmd.try_get_matches_from_mut(vec!["lk-snap", "-v"]);

        assert!(res.is_ok());
        assert!(res.as_mut().unwrap().get_flag("working"));
    }

    #[test]
    fn test_write_command() {
        let mut cmd = write_command();

        // No option
        let res = cmd.try_get_matches_from_mut(vec!["write-tree"]);
        assert!(res.is_ok());

        // Not exepected value
        let res = cmd.try_get_matches_from_mut(vec!["write-tree", "jfaf7GATG7ya"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_voyage_command() {
        let mut cmd = voyage_command();

        let res = cmd.try_get_matches_from_mut(vec!["voyage"]);
        assert!(res.is_ok());

        let res = cmd.try_get_matches_from_mut(vec!["voyage", "first.txt"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_snap_command() {
        let mut cmd = snap_command();

        // No option
        let res = cmd.try_get_matches_from_mut(vec!["snap"]);
        assert!(res.is_ok());

        // Not exepected value
        let mut res = cmd.try_get_matches_from_mut(vec!["snap", "first.txt"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut().unwrap().get_one::<String>("file").unwrap(),
            "first.txt"
        );
        assert_ne!(
            res.as_mut().unwrap().get_one::<String>("file").unwrap(),
            "6fawfwK234412"
        );
        assert!(res.as_mut().unwrap().try_get_one::<String>("test").is_err());

        // Run with -a option
        let mut res = cmd.try_get_matches_from_mut(vec!["snap", "-A"]);

        assert!(res.is_ok());
        assert!(res.as_mut().unwrap().get_flag("all"));
    }

    #[test]
    fn test_reg_command() {
        let mut cmd = reg_command();

        let res = cmd.try_get_matches_from_mut(vec!["reg"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

        let mut res = cmd.try_get_matches_from_mut(vec!["reg", "-m", "initial"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut().unwrap().get_one::<String>("message").unwrap(),
            "initial"
        );
        assert_ne!(
            res.as_mut().unwrap().get_one::<String>("message").unwrap(),
            "6fawfwK234412"
        );
        assert!(res.as_mut().unwrap().try_get_one::<String>("test").is_err());
    }

    #[test]
    fn test_bookemark_command() {
        let mut cmd = bookemark_command();

        // No option
        let res = cmd.try_get_matches_from_mut(vec!["bookmark"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

        // Get message value
        let mut res = cmd.try_get_matches_from_mut(vec!["bookmark", "develop"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut()
                .unwrap()
                .get_one::<String>("bookmarker")
                .unwrap(),
            "develop"
        );
        assert_ne!(
            res.as_mut()
                .unwrap()
                .get_one::<String>("bookmarker")
                .unwrap(),
            "6fawfwK234412"
        );
        assert!(res.as_mut().unwrap().try_get_one::<String>("test").is_err());

        // Get hash value
        let mut res = cmd.try_get_matches_from_mut(vec!["bookmark", "develop", "6fawfwK234412"]);
        assert!(res.is_ok());
        assert_ne!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "develop"
        );
        assert_eq!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "6fawfwK234412"
        );

        // Run with -r option
        let mut res = cmd.try_get_matches_from_mut(vec!["bookmark", "-r", "develop"]);

        assert!(res.is_ok());
        assert!(res.as_mut().unwrap().get_flag("replace"));
    }

    #[test]
    fn test_ref_command() {
        let mut cmd = ref_command();

        // No option
        let res = cmd.try_get_matches_from_mut(vec!["update-ref"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

        // Get hash value
        let mut res = cmd.try_get_matches_from_mut(vec!["update-ref", "6fawfwK234412"]);
        assert!(res.is_ok());
        assert_ne!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "first.txt"
        );
        assert_eq!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "6fawfwK234412"
        );
        assert!(res.as_mut().unwrap().try_get_one::<String>("test").is_err());
    }

    #[test]
    fn test_history_command() {
        let mut cmd = history_command();

        // No option
        let res = cmd.try_get_matches_from_mut(vec!["story"]);
        assert!(res.is_ok());

        // Run with -s option
        let mut res = cmd.try_get_matches_from_mut(vec!["story", "-s"]);

        assert!(res.is_ok());
        assert!(res.as_mut().unwrap().get_flag("short"));
    }

    #[test]
    fn test_goto_command() {
        let mut cmd = goto_command();

        let res = cmd.try_get_matches_from_mut(vec!["go-to"]);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);

        let mut res = cmd.try_get_matches_from_mut(vec!["go-to", "jfaf7GATG7ya"]);
        assert!(res.is_ok());
        assert_eq!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "jfaf7GATG7ya"
        );
        assert_ne!(
            res.as_mut().unwrap().get_one::<String>("hash").unwrap(),
            "6fawfwK234412"
        );
        assert!(res
            .as_mut()
            .unwrap()
            .try_get_one::<String>("value")
            .is_err());
    }
}
