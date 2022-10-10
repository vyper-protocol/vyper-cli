pub mod core_args;
pub mod config_args;
pub mod redeem_plugin_args;
use {
    core_args::CoreCommand,
    config_args::ConfigOptions,
    redeem_plugin_args:: {
        redeem_logic_forward_args::RedeemLogicForwardCommand,
    },
    clap::{
        Parser,
        Subcommand
    }
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
    /// Used to access redeem logic forward commands
    RedeemLogicForward(RedeemLogicForwardCommand)
}

