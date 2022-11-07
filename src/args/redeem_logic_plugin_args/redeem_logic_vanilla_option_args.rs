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
pub struct RedeemLogicVanillaOptionCommand {
    #[clap(subcommand)]
    pub command : RedeemLogicVanillaOptionSubcommand
}

#[derive(Debug, Subcommand)]
pub enum RedeemLogicVanillaOptionSubcommand {
    /// Gets the state of redeem logic vanilla option plugin from the given public key.
    Fetch(FetchState),
    /// Creates a redeem logic vanilla option state with given configuration
    Create(PluginState)
}

#[derive(Debug, Args)]
pub struct FetchState {
    /// Public key of state of plugin.
    pub state_id: Pubkey
}

#[derive(Debug, Args)]
pub struct PluginState {
    /// strike value for plugim
    #[clap(long)]
    pub strike: f64,
    /// linear value for plugin
    #[clap(long="linear", parse(try_from_str))]
    pub is_linear: bool,
    /// call value for plugin
    #[clap(long="call", parse(try_from_str))]
    pub is_call: bool,
    
}