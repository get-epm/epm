mod epm_config;
mod epm_options;
mod files_composer;
mod files_epm;
mod project_context;

use std::process::ExitCode;

use epm_config::CommandsMode;
use epm_options::get_epm_options;

fn main() -> Result<ExitCode, ExitCode> {
    let project_context = project_context::get_project_context().unwrap(); // TODO: handle error

    match project_context.config.commands_mode {
        CommandsMode::Composer => {
            eprintln!("Composer mode not yet implemented");
            Err(ExitCode::FAILURE)
        }
        CommandsMode::Epm => {
            let subcommand = get_epm_options().fallback_to_usage().run();
            println!("{:?}", &subcommand);
            Ok(ExitCode::SUCCESS)
        }
    }
}
