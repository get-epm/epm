use bpaf::{construct, OptionParser, Parser};

#[derive(Debug, Clone)]
pub(crate) enum EpmCommand {
    Add {},
    Config {},
    Init {},
    Install {},
    Remove {},
    Run {},
    Upgrade {},
}

fn get_add_cmd_parser() -> impl Parser<EpmCommand> {
    construct!(EpmCommand::Add {})
}

fn get_config_cmd_parser() -> impl Parser<EpmCommand> {
    construct!(EpmCommand::Config {})
}

fn get_install_cmd_parser() -> impl Parser<EpmCommand> {
    construct!(EpmCommand::Install {})
}

fn get_remove_cmd_parser() -> impl Parser<EpmCommand> {
    construct!(EpmCommand::Remove {})
}

fn get_run_cmd_parser() -> impl Parser<EpmCommand> {
    construct!(EpmCommand::Run {})
}

fn get_upgrade_cmd_parser() -> impl Parser<EpmCommand> {
    construct!(EpmCommand::Upgrade {})
}

pub(crate) fn get_epm_options() -> OptionParser<EpmCommand> {
    let add_command = get_add_cmd_parser().to_options().command("add");
    let config_command = get_config_cmd_parser().to_options().command("config");
    let init_command = construct!(EpmCommand::Init {}).to_options().command("init");
    let install_command = get_install_cmd_parser().to_options().command("install");
    let remove_command = get_remove_cmd_parser().to_options().command("remove");
    let run_command = get_run_cmd_parser().to_options().command("run");
    let upgrade_command = get_upgrade_cmd_parser().to_options().command("upgrade");

    construct!([
        add_command,
        config_command,
        init_command,
        install_command,
        remove_command,
        run_command,
        upgrade_command
    ])
    .to_options()
}
