mod commands;
mod util;
use clap::{App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new("Shnippet")
        .version("0.1.0")
        .author("YJDoc2")
        .about("Commandline snippet manager")
        .subcommand(SubCommand::with_name("list").about("List all shnippets"))
        .subcommand(SubCommand::with_name("new").about("Add new shnippet"))
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete an existing shnippet")
                .arg(Arg::with_name("name").help("Name of shnippet to delete")),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit an existing shnippet")
                .arg(Arg::with_name("name").help("Name of shnippet to edit")),
        )
        .subcommand(
            SubCommand::with_name("exec")
                .about("Run a shnippet in shell")
                .arg(Arg::with_name("name").help("Name of shnippet to run")),
        )
        .get_matches();

    let mut data = util::setup();

    match matches.subcommand() {
        ("list", _) => commands::list(&data),
        ("delete", Some(sub)) => {
            let name = sub.value_of("name").unwrap_or_default();
            commands::delete(&mut data, &name);
        }
        ("exec", Some(sub)) => {
            let name = sub.value_of("name").unwrap_or_default();
            commands::exec(&name);
        }
        ("new", _) => {
            commands::new(&mut data);
        }
        ("edit", Some(sub)) => {
            let name = sub.value_of("name").unwrap_or_default();
            commands::edit(&data, &name);
        }
        (name, _) => {
            eprintln!("Unknown subcommand {}, try -h for help.", name);
            eprintln!("Exiting...");
            exit(1);
        }
    }
}
