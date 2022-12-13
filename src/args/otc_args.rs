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
    /// Creates the otc with input configuration
    Create(CreateOtc)
}

#[derive(Debug, Args)]
pub struct FetchOtc {
    /// Public key of otc state.
    pub state_id: Pubkey
}

#[derive(Debug, Args)]
pub struct CreateOtc {
    // /// Senior deposit amount
    // pub senior_deposit_amount: u64,
    // /// Junior deposit amount
    // pub junior_deposit_amount: u64,
    // /// Deposit end date and time
    // pub deposit_end: i64,
    // /// Settle start date and time
    // pub settle_start: i64,
    /// Rate plugin type
    pub rate_plugin_type: String,
    /// Redeem plugin type
    pub redeem_plugin_type: String,
    // /// Deposit start date and time
    // pub deposit_start: Option<i64>,
}