mod commands;
mod util;

use util::shnippet_name;
use clap::{Command, builder::Str};
use std::{process::exit, sync::Arc};

fn main() {
    let mut data = util::setup();

    let shnippets = shnippet_subCommands(&data, Option::None);

    
    let matches = Command::new("Shnippet")
        .version("0.1.0")
        .author("YJDoc2")
        .about("Commandline snippet manager")
        .subcommand_required(true)
        .subcommand(Command::new("list").about("List all shnippets"))
        .subcommand(Command::new("new").about("Add new shnippet"))
        .subcommand(
            Command::new("delete")
                .about("Delete an existing shnippet")
                .subcommand_required(true)
                .subcommands(shnippets.clone()),
        )
        .subcommand(
            Command::new("edit")
                .about("Edit an existing shnippet")
                .subcommands(shnippets.clone()).subcommand_required(true),
        )
        .subcommand(
            Command::new("exec")
                .about("Run a shnippet in shell")
                .subcommands(shnippets),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("list", _))=> commands::list(&data),
        Some(("delete", sub)) => {
            let opt_name = shnippet_name(sub);
            match(opt_name){
                Some(name) => commands::delete(&mut data, name),
                None => {
                    eprintln!("Unknown subcommand, try -h for help.");
                    eprintln!("Exiting...");
                    exit(1);
                }
            }
        }
        Some(("exec", sub)) => {
            let opt_name = shnippet_name(sub);
            match(opt_name){
                Some(name) => commands::exec(name),
                None => {
                    eprintln!("Unknown subcommand, try -h for help.");
                    eprintln!("Exiting...");
                    exit(1);
                }
            }
        }
        Some(("new", _)) => {
            commands::new(&mut data);
        }
        Some(("edit", sub)) => {
            let opt_name = shnippet_name(sub);
            match(opt_name){
                Some(name) => commands::edit(&data, name),
                None => {
                    eprintln!("Unknown subcommand, try -h for help.");
                    eprintln!("Exiting...");
                    exit(1);
                }
            }
        }
        Some((name, _)) => {
            eprintln!("Unknown subcommand {}, try -h for help.", name);
            eprintln!("Exiting...");
            exit(1);
        }
        None => {
            eprintln!("No subcommand given, try -h for help.");
            eprintln!("Exiting...");
            exit(1);
        }
    }
}

fn shnippet_sub_commands<'a>(data: &util::Data, template: Option<String>) -> Vec<Command> {

    let template_ref = template.as_ref();


    let shnippets: Vec<Command> = data.commands.iter().map(|(name, description)| {
        let about: String = template_ref.map(|help| help.replace("{name}", name).replace("{description}", description).to_owned()).unwrap_or(description.clone());
        Command::new(name.clone()).about(about)
    }).collect();
    shnippets
}