use std::process::exit;

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
    utils::println_error
};


const VYPER_CORE_ID: &str = "vyPErCcGJKQQBeeQ59gXcWrDyU4vBrq8qQfacwmsAsp";

fn main() {

    // parsing arguments
    let args = VyperCliArgs::parse();

    // getting the config file and key-pair
    let config_file = CONFIG_FILE.as_ref();
    let config_file = match config_file {
        Some(file) => file,
        None => {
            println_error("Could not read the config file");
            exit(1);
        }
    };
    let cli_config = Config::load(&config_file);
    let cli_config = match cli_config {
        Ok(config) => config,
        Err(_) => {
            println_error("Could not load the config file");
            exit(1);
        }
    };

    let key_pair = read_keypair_file(&cli_config.keypair_path);
    let key_pair = match key_pair {
        Ok(keys) => keys,
        Err(_) => {
            println_error("Could not find keypair in the config file");
            exit(1);
        }
    };

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
