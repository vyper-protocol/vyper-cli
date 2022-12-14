use std::process::exit;
mod args;
mod ops;
mod utils;

use {
    args::VyperCliArgs,
    clap::Parser,
    args::Vyper,
    ops::core_ops::handle_core_command,
    ops::redeem_logic_plugin_ops::{
        redeem_logic_forward_ops::handle_redeem_logic_forward_command,
        redeem_logic_settle_forward_ops::handle_redeem_logic_settle_forward_command,
        redeem_logic_vanilla_option_ops::handle_redeem_logic_vanilla_option_command,
    },
    ops::rate_plugin_ops::{
        rate_switchboard_ops::handle_rate_switchboard_command,
        rate_pyth_ops::handle_rate_pyth_command
    },
    ops::otc_ops::handle_otc_command,
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
const RATE_SWITCHBOARD: &str  = "2hGXiH1oEQwjCXRx8bNdHTi49ScZp7Mj2bxcjxtULKe1";
const RATE_PYTH: &str = "3mxtC2cGVhHucUg4p58MVzVqUKLyiy1zWqRkRQdgUBPT";
const OTC: &str = "8aHSkExY28qCvg4gnTLU7y1Ev6HnpJ1NxuWb9XtEesVt";
const REDEEM_LOGIC_FORWARD: &str = "BrpV1re8MshA8qskKVxcEG8zXG3vf2uLX6myeTKAyhsK";
const REDEEM_LOGIC_SETTLE_FORWARD: &str = "6vBg1GMtKj7EYDLWWt6tkHoDWLAAksNPbKWiXMic99qU";
const REDEEM_LOGIC_VANILLA_OPTION: &str = "8fSeRtFseNrjdf8quE2YELhuzLkHV7WEGRPA9Jz8xEVe";

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
        },
        Vyper::RedeemLogicForward(redeem_logic_command) => {
            // redem logic forward program
            let redeem_logic_forward_program_id: Pubkey = Pubkey::new(&bs58::decode(&REDEEM_LOGIC_FORWARD).into_vec().expect("Invalid redeem logic forward program id"));
            let redeem_logic_forward_program = client.program(redeem_logic_forward_program_id);
            // command handler
            handle_redeem_logic_forward_command(redeem_logic_command, &redeem_logic_forward_program);
        },
        Vyper::RateSwitchboard(rate_switchboard_command) => {
             // rate switchboard program
             let rate_switchboard_program_id: Pubkey = Pubkey::new(&bs58::decode(&RATE_SWITCHBOARD).into_vec().expect("Invalid rate switchboard program id"));
             let rate_switchboard_program = client.program(rate_switchboard_program_id);
             // command handler
             handle_rate_switchboard_command(rate_switchboard_command, &rate_switchboard_program);
        },
        Vyper::RedeemLogicSettleForward(redeem_logic_command) => {
            // redem logic settle forward program
            let redeem_logic_settle_forward_program_id: Pubkey = Pubkey::new(&bs58::decode(&REDEEM_LOGIC_SETTLE_FORWARD).into_vec().expect("Invalid redeem logic forward program id"));
            let redeem_logic_settle_forward_program = client.program(redeem_logic_settle_forward_program_id);
            // command handler
            handle_redeem_logic_settle_forward_command(redeem_logic_command, &redeem_logic_settle_forward_program);
        },
        Vyper::RedeemLogicVanillaOption(redeem_logic_command) => {
            // redem logic vanilla option program
            let redeem_logic_vanilla_option_program_id: Pubkey = Pubkey::new(&bs58::decode(&REDEEM_LOGIC_VANILLA_OPTION).into_vec().expect("Invalid redeem logic forward program id"));
            let redeem_logic_vanilla_option_program = client.program(redeem_logic_vanilla_option_program_id);
            // command handler
            handle_redeem_logic_vanilla_option_command(redeem_logic_command, &redeem_logic_vanilla_option_program);
        },
        Vyper::Otc(otc_command) => {
            // otc program
            let otc_program_id: Pubkey = Pubkey::new(&bs58::decode(&OTC).into_vec().expect("Invalid otc program id"));
            let otc_program = client.program(otc_program_id);
            // command handler
            handle_otc_command(otc_command, &otc_program);
        },
        Vyper::RatePyth(rate_pyth_command) => {
            // rate switchboard program
            let rate_pyth_program_id: Pubkey = Pubkey::new(&bs58::decode(&RATE_PYTH).into_vec().expect("Invalid rate pyth program id"));
            let rate_pyth_program = client.program(rate_pyth_program_id);
            // command handler
            handle_rate_pyth_command(rate_pyth_command, &rate_pyth_program);
       },
    }
}
