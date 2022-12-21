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
pub struct OtcCommand {
    #[clap(subcommand)]
    pub command : OtcSubcommand
}

#[derive(Debug, Subcommand)]
pub enum OtcSubcommand {
    /// Gets the configuration of otc from given public key.
    Fetch(FetchOtc),
    /// Creates the otc with given input configuration
    Create
}

#[derive(Debug, Args)]
pub struct FetchOtc {
    /// Public key of otc state.
    pub state_id: Pubkey
}