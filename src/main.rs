mod args;
mod ops;
mod utils;

use {
    args::VyperCliArgs,
    clap::Parser,
    args::Vyper,
    ops::core_ops::handle_core_command,
    anchor_client::{
        Client,
        Cluster,
        solana_sdk::{
            signer::keypair::read_keypair_file,
            pubkey:: Pubkey
        },
    },
    solana_cli_config::{CONFIG_FILE, Config},
    std::rc::Rc,
};


const VYPER_CORE_ID: &str = "vyPErCcGJKQQBeeQ59gXcWrDyU4vBrq8qQfacwmsAsp";

fn main() {

    // parsing arguments
    let args = VyperCliArgs::parse();

    // getting the config file and key-pair
    let config_file = CONFIG_FILE.as_ref().expect("Could not read the config file");
    let cli_config = Config::load(&config_file).expect("Could not load the config file");
    let key_pair = read_keypair_file(&cli_config.keypair_path).expect("Could not find keypair in the config file");

    // setting the cluster and keypair
    let current_cluster = Cluster::Custom(cli_config.json_rpc_url,cli_config.websocket_url);
    let client = Client::new(current_cluster, Rc::new(key_pair));

    

    match args.vyper {
        Vyper::Core(core) => {
            // vyper core program
            let core_program_id: Pubkey = Pubkey::new(&bs58::decode(&VYPER_CORE_ID).into_vec().expect("Invalid vyper core program id"));
            let core_program = client.program(core_program_id);
            // core command handler
            handle_core_command(core,&core_program)
        }
    }
}
