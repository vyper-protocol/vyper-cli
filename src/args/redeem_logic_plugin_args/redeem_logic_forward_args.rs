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
pub struct RedeemLogicForwardCommand {
    #[clap(subcommand)]
    pub command : RedeemLogicForwardSubcommand
}

#[derive(Debug, Subcommand)]
pub enum RedeemLogicForwardSubcommand {
    /// Gets the state of redeem logic forward plugin from the given public key.
    Fetch(FetchState),
    /// Creates a redeem logic forward state with given configuration
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
    /// notional value for plugin 
    pub notional: u64,
    /// strike value for plugin
    #[clap(long)]
    pub strike: f64,
    /// linear value for plugin
    #[clap(long="linear", parse(try_from_str))]
    pub is_linear: bool,
    
}