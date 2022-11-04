mod commands;
mod constants;
mod util;

use clap::Command;
use constants::*;
use std::process::exit;
use util::shnippet_name;

fn main() {
    let mut data = util::setup();

    let matches = require_subcommands(
        Command::new("Shnippet")
            .version("0.1.0")
            .author("YJDoc2")
            .about("Commandline snippet manager")
            .subcommand(Command::new(LIST_COMMAND_NAME).about(LIST_COMMAND_DESCRIPTION))
            .subcommand(Command::new(NEW_COMMAND_NAME).about(NEW_COMMAND_DESCRIPTION))
            .subcommand(shnippet_list_command(
                DELETE_COMMAND_NAME,
                DESCRIPTION_COMMAND_NAME,
                &data,
                Some(DELETE_DESCRIPTION_TEMPLATE),
            ))
            .subcommand(shnippet_list_command(
                EDIT_COMMAND_NAME,
                EDIT_COMMAND_DESCRIPTION,
                &data,
                Some(EDIT_DESCRIPTION_TEMPLATE),
            ))
            .subcommand(shnippet_list_command(
                EXEC_COMMAND_NAME,
                EXEC_COMMAND_DESCRIPTION,
                &data,
                Option::None,
            )),
    )
    .get_matches();

    match matches.subcommand() {
        Some(("list", _)) => commands::list(&data),
        Some(("delete", sub)) => {
            let opt_name = shnippet_name(sub);
            match opt_name {
                Some(name) => commands::delete(&mut data, name),
                None => report_error_exit(),
            }
        }
        Some(("exec", sub)) => {
            let opt_name = shnippet_name(sub);
            match opt_name {
                Some(name) => commands::exec(name),
                None => report_error_exit(),
            }
        }
        Some(("new", _)) => {
            commands::new(&mut data);
        }
        Some(("edit", sub)) => {
            let opt_name = shnippet_name(sub);
            match opt_name {
                Some(name) => commands::edit(&data, name),
                None => report_error_exit(),
            }
        }
        Some((_, _)) => report_error_exit(),
        None => report_error_exit(),
    }
}

fn report_error_exit() {
    eprintln!("Unknown subcommand, try -h for help.");
    eprintln!("Exiting...");
    exit(1);
}

fn shnippet_list_command(
    name: &'static str,
    description: &'static str,
    data: &util::Data,
    description_template: Option<&str>,
) -> Command {
    require_subcommands(
        Command::new(name)
            .about(description)
            .subcommands(shnippet_sub_commands(data, description_template)),
    )
}

fn require_subcommands(command: Command) -> Command {
    return command
        .subcommand_required(true)
        .arg_required_else_help(true);
}

fn shnippet_sub_commands(data: &util::Data, template: Option<&str>) -> Vec<Command> {
    let shnippets: Vec<Command> = data
        .commands
        .iter()
        .map(|(name, description)| {
            let about: String = template
                .map(|help| {
                    help.replace(NAME_PLACEHOLDER, name)
                        .replace(DESCRIPTION_PLACEHOLDER, description)
                        .to_owned()
                })
                .unwrap_or(description.clone());
            Command::new(name.clone()).about(about)
        })
        .collect();
    shnippets
}
