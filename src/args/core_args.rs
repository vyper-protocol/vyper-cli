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
    Fetch(FetchTranche),
}

#[derive(Debug, Args)]
pub struct FetchTranche {
    pub tranche_id: Pubkey
}