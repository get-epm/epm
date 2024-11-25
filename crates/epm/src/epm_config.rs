use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum CommandsMode {
    Composer,
    Epm,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct EpmConfig {
    pub(crate) commands_mode: CommandsMode,
    pub(crate) cache_dir: PathBuf,
}

pub(crate) fn get_epm_config() -> Result<EpmConfig, ()> {
    let project_dirs = ProjectDirs::from("dev", "php-epm", "epm");
    let config_path_envvar_result = env::var("EPM_CONFIG_PATH");

    let config_path = match config_path_envvar_result {
        Ok(config_path_envvar) => PathBuf::from(config_path_envvar),
        Err(_) => {
            match &project_dirs {
                Some(project_dirs) => project_dirs.config_dir().join("config.json"),
                None => return Err(()), // TODO: Return proper error
            }
        }
    };

    let mut should_persist = false;
    let mut config = if config_path.exists() {
        if !config_path.is_file() {
            return Err(()); // TODO: Return proper error
        }

        let file = match File::open(&config_path) {
            Ok(file) => file,
            Err(_) => return Err(()), // TODO: Return proper error
        };

        let reader = BufReader::new(file);
        match from_reader(reader) {
            Ok(config) => config,
            Err(_) => return Err(()), // TODO: Return proper error
        }
    } else {
        let cfg = EpmConfig {
            commands_mode: CommandsMode::Epm,
            cache_dir: match env::var("EPM_CACHE_DIR") {
                Ok(cache_dir_envvar) => PathBuf::from(cache_dir_envvar),
                Err(_) => match &project_dirs {
                    Some(project_dirs) => project_dirs.cache_dir().to_path_buf(),
                    None => return Err(()), // TODO: Return proper error
                },
            },
        };

        should_persist = true;
        cfg
    };

    patch_config_with_envvars(&mut config);

    if should_persist {
        if let Err(value) = persist_config(&config_path, &config) {
            return Err(value);
        }
    }

    Ok(config)
}

fn patch_config_with_envvars(config: &mut EpmConfig) {
		if let Ok(commands_mode_str) = env::var("EPM_COMMANDS_MODE") {
				match commands_mode_str.as_str() {
						// TODO: verify that this is the correct serialization (or use some sort of const fn)
						"Composer" => config.commands_mode = CommandsMode::Composer,
						"Epm" => config.commands_mode = CommandsMode::Epm,
						_ => {} // TODO: Log proper warning
				}
		}
}

fn persist_config(config_path: &Path, config: &EpmConfig) -> Result<(), ()> {
    let parent_directory = config_path.parent().unwrap(); // TODO: Handle error
    if !parent_directory.exists() {
        if let Err(_) = std::fs::create_dir_all(parent_directory) {
            return Err(()); // TODO: Return proper error
        }
    }

    let file = match File::create(&config_path) {
        Ok(file) => file,
        Err(_) => return Err(()), // TODO: Return proper error
    };

    match serde_json::to_writer_pretty(file, config) {
        Ok(_) => (),
        Err(_) => return Err(()), // TODO: Return proper error
    }

    Ok(())
}
