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
pub struct RedeemLogicDigitalCommand {
    #[clap(subcommand)]
    pub command : RedeemLogicDigitalSubcommand
}

#[derive(Debug, Subcommand)]
pub enum RedeemLogicDigitalSubcommand {
    /// Gets the state of redeem logic digital plugin from the given public key.
    Fetch(FetchState),
    /// Creates a redeem logic digital state with given configuration
    Create(PluginState)
}

#[derive(Debug, Args)]
pub struct FetchState {
    /// Public key of state of plugin.
    pub state_id: Pubkey
}

#[derive(Debug, Args)]
pub struct PluginState {
    /// strike value for plugin
    #[clap(long)]
    pub strike: f64,
    /// call value for plugin
    #[clap(long="call", parse(try_from_str))]
    pub is_call: bool,
    
}