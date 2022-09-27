mod args;
mod ops;

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



fn main() {
    let args = VyperCliArgs::parse();
    let program_id: Pubkey = Pubkey::new(&bs58::decode("vyPErCcGJKQQBeeQ59gXcWrDyU4vBrq8qQfacwmsAsp").into_vec().unwrap());
    
    let config_file = CONFIG_FILE.as_ref().unwrap();
    let cli_config = Config::load(&config_file).unwrap();
    
    let key_pair = read_keypair_file(&cli_config.keypair_path).unwrap();
    println!("{:?}",cli_config);
    let client = Client::new(Cluster::Devnet, Rc::new(key_pair));
    let program = client.program(program_id);
    match args.vyper {
        Vyper::Core(core) => {
            handle_core_command(core,&program)
        }
    }
}
