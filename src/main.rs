mod cli;

mod types;
pub use types::*;

mod config;
mod efi;
mod util;

mod command;

use clap::Parser;

use crate::cli::{Cli, Commands};

fn main() -> miette::Result<()> {
    let args = Cli::parse();

    util::check_privileges()?;

    let config_path = config::determine_config_path(args.config_path)?;

    match args.command {
        Commands::Boot { target, no_reboot } => command::boot::boot(config_path, target, no_reboot),
        Commands::Scan => command::scan::scan(config_path),
        Commands::Alias { action } => match action {
            cli::AliasCommands::List => command::alias::list(config_path),
            cli::AliasCommands::Add { name, id } => command::alias::add(config_path, &name, id),
            cli::AliasCommands::Remove { name } => command::alias::remove(config_path, &name),
            cli::AliasCommands::Clear { yes } => command::alias::clear(config_path, yes),
        },
        Commands::AllowNonAdmin { it_path } => command::allow_non_admin::allow_non_admin(it_path),
    }
}
