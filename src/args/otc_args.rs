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
}

#[derive(Debug, Args)]
pub struct FetchOtc {
    /// Public key of tranche configuration.
    pub state_id: Pubkey
}