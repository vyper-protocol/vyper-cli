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
pub struct RateSwitchboardCommand {
    #[clap(subcommand)]
    pub command : RateSwitchboardSubcommand
}

#[derive(Debug, Subcommand)]
pub enum RateSwitchboardSubcommand {
    /// Gets the state of rate switchboard plugin from the given public key.
    Fetch(FetchState),
    /// Creates rate switchboard plugin with suitable aggregators.
    Create
}

#[derive(Debug, Args)]
pub struct FetchState {
    /// Public key of state of plugin.
    pub state_id: Pubkey
}