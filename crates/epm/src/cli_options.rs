use bpaf::{construct, OptionParser, Parser};

#[derive(Debug, Clone)]
pub enum CliCommand {
    AddCommand {},
    InstallCommand {},
    RemoveCommand {},
    RunCommand {},
}

fn get_add_cmd_parser() -> impl Parser<CliCommand> {
    construct!(CliCommand::AddCommand {})
}

fn get_install_cmd_parser() -> impl Parser<CliCommand> {
    construct!(CliCommand::InstallCommand {})
}

fn get_remove_cmd_parser() -> impl Parser<CliCommand> {
    construct!(CliCommand::RemoveCommand {})
}

fn get_run_cmd_parser() -> impl Parser<CliCommand> {
    construct!(CliCommand::RunCommand {})
}

pub fn get_options() -> OptionParser<CliCommand> {
    let add_command = get_add_cmd_parser().to_options().command("add");
    let install_command = get_install_cmd_parser().to_options().command("install");
    let remove_command = get_remove_cmd_parser().to_options().command("remove");
    let run_command = get_run_cmd_parser().to_options().command("run");

    construct!([add_command, install_command, remove_command, run_command,]).to_options()
}
