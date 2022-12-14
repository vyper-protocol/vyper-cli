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
pub struct RatePythCommand {
    #[clap(subcommand)]
    pub command : RatePythSubcommand
}

#[derive(Debug, Subcommand)]
pub enum RatePythSubcommand {
    /// Gets the state of rate pyth plugin from the given public key.
    Fetch(FetchState),
    /// Creates the rate-pyth plugin
    Create
}

#[derive(Debug, Args)]
pub struct FetchState {
    /// Public key of state of plugin.
    pub state_id: Pubkey
}