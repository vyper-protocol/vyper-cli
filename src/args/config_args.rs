use {
    anchor_client::Cluster,
    clap::{
        Parser
    },
};

#[derive(Default, Debug, Parser)]
pub struct ConfigOptions {
    /// Cluster override.
    #[clap(global = true, long = "client.cluster")]
    pub cluster: Option<Cluster>,
    /// Wallet override.
    #[clap(global = true, long = "client.wallet")]
    pub wallet: Option<String>,
}