pub mod core_args;
pub mod config_args;
use core_args::CoreCommand;
use config_args::ConfigOptions;

use clap::{
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(name = "vyper-cli")]
#[clap(author, version, about)]
pub struct VyperCliArgs {

    #[clap(subcommand)]
    pub vyper: Vyper,

    #[clap(flatten)]
    pub config_override: ConfigOptions,
}

#[derive(Debug, Subcommand)]
pub enum Vyper {
    
    /// Used to access vyper core commands
    Core(CoreCommand),
    // Plugin(PluginCommand)
}

