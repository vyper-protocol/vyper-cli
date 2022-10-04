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
    solana_cli_config::Config,
    std::rc::Rc,
    utils::{
        println_error,
        get_solana_config
    }
};


const VYPER_CORE_ID: &str = "vyPErCcGJKQQBeeQ59gXcWrDyU4vBrq8qQfacwmsAsp";

fn main() {

    // parsing arguments
    let args = VyperCliArgs::parse();

    // getting the config file
    let mut cli_config:Option<Config> = None;

    let current_cluster = match args.config_override.cluster {
        Some(cluster) => cluster,
        None => {
            cli_config = Some(get_solana_config());
            match &cli_config {
                Some(solana_config) => Cluster::Custom(solana_config.json_rpc_url.clone(),solana_config.websocket_url.clone()),
                None=> {
                    println_error("Could not find a config file or --cluster option");
                    exit(1);
                }
            }
        }
    };

    let keypair_path = match args.config_override.wallet {
        Some(keypair) => keypair,
        None => {
            match &cli_config {
                Some(solana_config)=> solana_config.keypair_path.clone(),
                None => {
                    get_solana_config().keypair_path
                }
            }
        }
    };

    let key_pair = read_keypair_file(&keypair_path);
    let key_pair = match key_pair {
        Ok(keys) => keys,
        Err(_) => {
            println_error("Could not find keypair in the config file");
            exit(1);
        }
    };


    let client = Client::new(current_cluster, Rc::new(key_pair));
    match args.vyper {
        Vyper::Core(core) => {
            // vyper core program
            let core_program_id: Pubkey = Pubkey::new(&bs58::decode(&VYPER_CORE_ID).into_vec().expect("Invalid vyper core program id"));
            let core_program = client.program(core_program_id);
            // core command handler
            handle_core_command(core,&core_program);
        }
    }
}
