use {
    std::process::exit,
    crate::args::redeem_logic_plugin_args,
    redeem_logic_plugin_args::redeem_logic_forward_args:: {
        RedeemLogicForwardCommand,
        RedeemLogicForwardSubcommand
    },
    anchor_client::{
        Program,
        ClientError,
        solana_sdk:: {
            signer::keypair::Keypair,
            signer::Signer,
            system_program
        }
    },
    redeem_logic_forward:: {
        RedeemLogicConfig,
        accounts::InitializeContext,
        instruction::Initialize,
    },
    crate::utils:: {
        println_name_value,
        println_error
    },
    rust_decimal::{
        Decimal
    },
    console::style
};



pub fn handle_redeem_logic_forward_command(redeem_logic_command: RedeemLogicForwardCommand, program: &Program) {
    let command = redeem_logic_command.command;
    match command {
        RedeemLogicForwardSubcommand::Fetch(fetch_state) => {
            let account:Result<RedeemLogicConfig,ClientError> = program.account(fetch_state.state_id);
            let account = match account {
                Ok(redeem_config) => redeem_config,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find a redeem logic forward plugin state with given public key"),
                        ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::LogParseError(err)=> println_error(&err)
                    }
                    exit(1);
                }   
            };
            println_name_value("notional", &account.notional);
            println_name_value("is_linear", &account.is_linear);
            println_name_value("strike",&Decimal::deserialize(account.strike));
        },
        RedeemLogicForwardSubcommand::Create(plugin_state) => {
            let plugin_config =  Keypair::new();
            let authority = program.payer();
            let signature = program.request()
                .signer(&plugin_config)
                .accounts(InitializeContext {
                    redeem_logic_config: plugin_config.pubkey(),
                    payer: authority,
                    system_program: system_program::ID,
                })
                .args(Initialize { notional: plugin_state.notional,is_linear:plugin_state.is_linear, strike:plugin_state.strike})
                .send(); 
            let signature = match signature {
                Ok(transaction) => transaction,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find a state with given public key"),
                        ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::LogParseError(err)=> println_error(&err)
                    }
                    exit(1);
                }
            };
            println_name_value("Redeem Logic Forward State successfully create at", &plugin_config.pubkey());
            println_name_value("Transaction Id", &signature);                                       
        }
    }
}