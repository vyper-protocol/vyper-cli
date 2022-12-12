pub mod core_args;
pub mod config_args;
pub mod redeem_logic_plugin_args;
pub mod rate_plugin_args;
pub mod otc_args;
use {
    core_args::CoreCommand,
    config_args::ConfigOptions,
    redeem_logic_plugin_args:: {
        redeem_logic_forward_args::RedeemLogicForwardCommand,
        redeem_logic_settle_forward_args::RedeemLogicSettleForwardCommand,
        redeem_logic_vanilla_option_args::RedeemLogicVanillaOptionCommand,
    },
    rate_plugin_args::rate_switchboard_args::RateSwitchboardCommand,
    clap::{
        Parser,
        Subcommand
    },
    otc_args::OtcCommand
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
    RedeemLogicForward(RedeemLogicForwardCommand),
    /// Used to access rate-switchboard plugin
    RateSwitchboard(RateSwitchboardCommand),
    /// Used to access redeem logic settle forward plugin
    RedeemLogicSettleForward(RedeemLogicSettleForwardCommand),
    /// Used to access redeem logic vanilla option plugin
    RedeemLogicVanillaOption(RedeemLogicVanillaOptionCommand),,
    /// Used to access the vyper otc commands
    Otc(OtcCommand)
}

