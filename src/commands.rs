use crate::util::{get_dir_path, store_data, Data};
use console::style;
use dialoguer::{Editor, Input};
use std::fs;
use std::process::{exit, Command};

pub fn list(data: &Data) {
    println!();
    for (name, description) in data.commands.iter() {
        println!("{} : ", style(name).bold());
        println!("{}", style(description).italic());
        println!();
    }
}

pub fn delete(data: &mut Data, name: &str) {
    if !data.commands.contains_key(name) {
        eprintln!(
            "{}",
            style(format!("No shnippet named {} found", name))
                .red()
                .bold()
        );
        exit(1);
    } else {
        data.commands.remove(name);
        store_data(&data);
        match fs::remove_file(get_dir_path(format!("{}.sh", name))) {
            Ok(_) => {
                println!("Removed {} successfully", name);
                exit(0);
            }
            Err(e) => {
                eprintln!("{}", style("Error in removing shnippet : ").red());
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
}

pub fn exec(name: &str) {
    let output = Command::new("sh")
        .arg(get_dir_path(format!("{}.sh", name)))
        .output()
        .expect("Error while executing shnippet");

    println!("Shnippet executed with status {}", output.status);
    println!("---");
    println!("{}", style("stdout : ").bold().italic());
    println!(
        "{}",
        String::from_utf8(output.stdout).unwrap_or_else(|_| "---???---".to_owned())
    );
    println!("---");
    println!("{}", style("stderr : ").bold().italic());
    println!(
        "{}",
        String::from_utf8(output.stderr).unwrap_or_else(|_| "---???---".to_owned())
    );
}

pub fn new(data: &mut Data) {
    let name: String = match Input::<String>::new().with_prompt("Name ").interact_text() {
        Ok(n) => {
            // make sure the name is file-system friendly
            let options = sanitize_filename::Options {
                replacement: "-",
                ..Default::default()
            };
            sanitize_filename::sanitize_with_options(n, options)
        }
        Err(e) => {
            eprintln!("Error : {}", e);
            eprintln!("Exiting...");
            exit(1);
        }
    };
    // shnippet already exists
    if let Some(v) = data.commands.get(&name) {
        eprintln!(
            "Shnippet {} already exists : \n {} {}",
            name,
            style("Description :").italic(),
            v
        );
        exit(1);
    };
    let desc: String = match Input::new().with_prompt("Desciption ").interact_text() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error : {}", e);
            eprintln!("Exiting...");
            exit(1);
        }
    };
    if let Some(command) = Editor::new().edit("").unwrap() {
        //store shnippet on filesystem
        match std::fs::write(get_dir_path(format!("{}.sh", name)), command) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error : {}", e);
                exit(1);
            }
        };
    } else {
        eprintln!("Aborting!");
        exit(1);
    }
    // save shnippet in data
    data.commands.insert(name, desc);
    store_data(&data);
}

pub fn edit(data: &Data, name: &str) {
    if !data.commands.contains_key(name) {
        eprintln!(
            "Shnippet {} not found, try creating one with shnippet new",
            name
        );
        exit(1);
    }

    let command = match std::fs::read_to_string(get_dir_path(format!("{}.sh", name))) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error : {}", e);
            exit(1);
        }
    };

    if let Some(new_command) = Editor::new().edit(&command).unwrap() {
        match std::fs::write(get_dir_path(format!("{}.sh", name)), new_command) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error : {}", e);
                exit(1);
            }
        };
    } else {
        eprintln!("Aborting!");
        exit(1);
    }
}
