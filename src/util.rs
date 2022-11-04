use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    pub commands: HashMap<String, String>,
}

const DIR: &str = ".shnippet";

pub fn get_dir_path(name: String) -> PathBuf {
    let base_dir = dirs::home_dir().unwrap();
    let root_dir = Path::new(&base_dir).join(DIR).join(name);
    root_dir
}

pub fn setup() -> Data {
    let base_path = get_dir_path("".to_owned());
    if base_path.exists() {
        // do nothing
    } else {
        match std::fs::create_dir_all(&base_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while creating data directory : {}", e);
                eprintln!("Exiting...");
                exit(1);
            }
        }
    }

    let data_string =
        std::fs::read_to_string(base_path.join("data.json".to_owned())).unwrap_or_default();
    let data: Data = serde_json::from_str(&data_string).unwrap_or_default();
    data
}

pub fn store_data(data: &Data) {
    match std::fs::write(
        get_dir_path("data.json".to_owned()),
        serde_json::to_string(data).unwrap_or_default(),
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error : {}", e);
            exit(1);
        }
    };
}

pub fn shnippet_name<'a>(arg_matches: &'a ArgMatches) -> Option<&'a str> {
    arg_matches.subcommand().map(|sub|sub.0)
}