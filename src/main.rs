mod commands;
mod util;

use util::shnippet_name;
use clap::{App, SubCommand, AppSettings};
use std::process::exit;

fn main() {
    let mut data = util::setup();

    let shnippets: Vec<App> = data.commands.iter().map(|(name, description)| {
        SubCommand::with_name(name).about(description.as_str())
    }).collect();

    let matches = App::new("Shnippet")
        .version("0.1.0")
        .author("YJDoc2")
        .about("Commandline snippet manager")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("list").about("List all shnippets"))
        .subcommand(SubCommand::with_name("new").about("Add new shnippet"))
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete an existing shnippet")
                .subcommands(shnippets.clone()).setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit an existing shnippet")
                .subcommands(shnippets.clone()).setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .subcommand(
            SubCommand::with_name("exec")
                .about("Run a shnippet in shell")
                .subcommands(shnippets).setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", _) => commands::list(&data),
        ("delete", Some(sub)) => {
            let name = shnippet_name(sub);
            commands::delete(&mut data, &name);
        }
        ("exec", Some(sub)) => {
            let name = shnippet_name(sub);
            commands::exec(&name);
        }
        ("new", _) => {
            commands::new(&mut data);
        }
        ("edit", Some(sub)) => {
            let name = shnippet_name(sub);
            commands::edit(&data, &name);
        }
        (name, _) => {
            eprintln!("Unknown subcommand {}, try -h for help.", name);
            eprintln!("Exiting...");
            exit(1);
        }
    }
}