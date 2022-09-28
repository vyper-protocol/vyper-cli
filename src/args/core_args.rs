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
pub struct CoreCommand {
    #[clap(subcommand)]
    pub command : CoreSubcommand
}

#[derive(Debug, Subcommand)]
pub enum CoreSubcommand {
    /// Gets the configuration of tranche from given public key.
    Fetch(FetchTranche),
}

#[derive(Debug, Args)]
pub struct FetchTranche {
    /// Public key of tranche configuration.
    pub tranche_id: Pubkey
}