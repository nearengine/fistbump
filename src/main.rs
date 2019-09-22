#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::Env;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
struct File {
    path: String,
    search: Option<String>,
    replace: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    current_version: String,
    search: Option<String>,
    replace: Option<String>,
    files: Vec<File>,
}

#[derive(StructOpt)]
struct CliArgs {
    version: String,
}

const CONFIG_PATH: &str = ".fistbumprc.json";

fn fistbump() -> Result<(), String> {
    info!("Initialized");

    let args = CliArgs::from_args();

    info!("Reading config from file {}", CONFIG_PATH);
    let config_data = match fs::read_to_string(CONFIG_PATH) {
        Ok(n) => n,
        Err(e) => return Err(format!("Couldn't read config file {}: {}", CONFIG_PATH, e)),
    };
    debug!("Got config `{}`", config_data);

    info!("Parsing config from JSON");
    let config: Config = match serde_json::from_str(&config_data) {
        Ok(n) => n,
        Err(e) => {
            return Err(format!(
                "Invalid JSON in config file {}: {}",
                CONFIG_PATH, e
            ))
        }
    };

    let current_version = config.current_version;
    let new_version = &args.version;
    let default_search = config.search.unwrap_or("{current_version}".to_string());
    let default_replace = config.replace.unwrap_or("{new_version}".to_string());

    for file in config.files.iter() {
        info!("Processing file {}", &file.path);
        let file_search = file
            .search
            .as_ref()
            .unwrap_or(&default_search)
            .replace("{current_version}", &current_version);
        debug!("Searching for `{}`", file_search);

        let file_replace = file
            .replace
            .as_ref()
            .unwrap_or(&default_replace)
            .replace("{new_version}", new_version);
        debug!("Replacing with `{}`", file_replace);

        let file_contents = match fs::read_to_string(&file.path) {
            Ok(n) => n,
            Err(e) => return Err(format!("Couldn't open file {}: {}", &file.path, e)),
        };

        let re = match Regex::new(&file_search) {
            Ok(n) => n,
            Err(e) => return Err(format!("Regex error: {}", e)),
        };

        let updated_file_contents = re.replace_all(&file_contents, file_replace.as_str());

        match fs::write(&file.path, updated_file_contents.as_ref()) {
            Ok(n) => n,
            Err(e) => return Err(format!("Couldn't write file {}: {}", &file.path, e)),
        };
    }

    let updated_config_data = config_data.replace(
        format!(r#""current_version": "{}""#, &current_version).as_str(),
        format!(r#""current_version": "{}""#, &new_version).as_str(),
    );
    debug!("Updating config `{}`", updated_config_data);

    match fs::write(CONFIG_PATH, updated_config_data) {
        Ok(n) => n,
        Err(e) => return Err(format!("Couldn't write config file: {}", e)),
    };

    info!("Finished");
    return Ok(());
}

fn main() {
    let env = Env::default().filter_or("FISTBUMP_LOG_LEVEL", "error");

    env_logger::init_from_env(env);

    if let Err(e) = fistbump() {
        error!("{}", e);
        ::std::process::exit(1);
    }
}
