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
}

#[derive(Debug, Args)]
pub struct FetchState {
    /// Public key of state of plugin.
    pub state_id: Pubkey
}