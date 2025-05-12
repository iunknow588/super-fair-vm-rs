use clap::{arg, Command};

pub const NAME: &str = "custom-vm-id";

#[must_use]
pub fn command() -> Command {
    Command::new(NAME)
        .about("Sets a custom VM ID")
        .arg(arg!(<VM_ID> "A custom VM ID"))
        .arg_required_else_help(true)
}
