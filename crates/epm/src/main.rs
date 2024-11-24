use std::process::ExitCode;

mod cli_options;

fn main() -> ExitCode {
    println!("{:?}", cli_options::get_options().run());
    ExitCode::SUCCESS
}
