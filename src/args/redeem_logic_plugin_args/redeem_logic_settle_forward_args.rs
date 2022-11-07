use {
    clap::{
        Args,
        Subcommand
    },
    anchor_client:: {
        solana_sdk::{
            pubkey::Pubkey
        }
    }
};



#[derive(Debug, Args)]
pub struct RedeemLogicSettleForwardCommand {
    #[clap(subcommand)]
    pub command : RedeemLogicSettleForwardSubcommand
}

#[derive(Debug, Subcommand)]
pub enum RedeemLogicSettleForwardSubcommand {
    /// Gets the state of redeem logic settle forward plugin from the given public key.
    Fetch(FetchState),
    /// Creates a redeem logic settle forward state with given configuration
    Create(PluginState)
}

#[derive(Debug, Args)]
pub struct FetchState {
    /// Public key of state of plugin.
    pub state_id: Pubkey
}

#[derive(Debug, Args)]
pub struct PluginState {
    #[clap(long)]
    /// notinal value for plugin
    pub notional: u64,
    /// strike value for plugin
    #[clap(long)]
    pub strike: f64,
    /// linear value for plugin
    #[clap(long="linear", parse(try_from_str))]
    pub is_linear: bool,
    /// standard value for plugin
    #[clap(long="standard", parse(try_from_str))]
    pub is_standard: bool,
    
}