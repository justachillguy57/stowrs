use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn get_cmd() -> ArgMatches {
    let cmd = Command::new("stowrs")
        .arg(
            Arg::new("pth")
                .help("Path argument")
                .value_name("PATH")
                .required(true),
        )
        .arg(
            Arg::new("in-dir")
                .help("where to search for the file mentioned [DEFAULT: `$HOME`]")
                .value_name("DIR")
                .required(false)
                .short('i')
                .long("in-dir"),
        )
        .arg(
            Arg::new("check")
                .help("check if the directories are able to be linked properly")
                .required(false)
                .short('C')
                .long("check"),
        )
        .arg(
            Arg::new("adopt")
                .help("adopt existing files")
                .required(false)
                .short('o')
                .long("adopt")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .help("enable verbose mode")
                .required(false)
                .short('V')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    cmd
}
