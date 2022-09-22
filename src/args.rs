use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(name = "vyper-cli")]
#[clap(author, version, about)]
pub struct VyperCliArgs {

    #[clap(subcommand)]
    pub vyper: Vyper
}

#[derive(Debug, Subcommand)]
pub enum Vyper {
    Core(CoreCommand),
    // Plugin(PluginCommand)
}

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
    pub tranche_id: u32
}