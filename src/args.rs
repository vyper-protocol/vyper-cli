pub mod core_args;

use core_args::CoreCommand;
use clap::{
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(name = "vyper-cli")]
#[clap(author, version, about)]
pub struct VyperCliArgs {

    #[clap(subcommand)]
    pub vyper: Vyper
}

#[derive(Debug, Subcommand)]
pub enum Vyper {
    Core(CoreCommand),
    // Plugin(PluginCommand)
}

