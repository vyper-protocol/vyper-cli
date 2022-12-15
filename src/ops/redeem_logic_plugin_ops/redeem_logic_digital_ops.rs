use {
    std::process::exit,
    crate::args::redeem_logic_plugin_args,
    redeem_logic_plugin_args::redeem_logic_digital_args:: {
        RedeemLogicDigitalCommand,
        RedeemLogicDigitalSubcommand
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
    redeem_logic_digital:: {
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



pub fn handle_redeem_logic_digital_command(redeem_logic_command: RedeemLogicDigitalCommand, program: &Program) {
    let command = redeem_logic_command.command;
    match command {
        RedeemLogicDigitalSubcommand::Fetch(fetch_state) => {
            let account:Result<RedeemLogicConfig,ClientError> = program.account(fetch_state.state_id);
            let account = match account {
                Ok(redeem_config) => redeem_config,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find a redeem logic digital plugin state with given public key"),
                        ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::LogParseError(err)=> println_error(&err)
                    }
                    exit(1);
                }   
            };
            println_name_value("strike",&Decimal::deserialize(account.strike));
            println_name_value("is_call", &account.is_call);
        },
        RedeemLogicDigitalSubcommand::Create(plugin_state) => {
            let plugin_config =  Keypair::new();
            let signature = program.request()
                .signer(&plugin_config)
                .accounts(InitializeContext {
                    redeem_logic_config: plugin_config.pubkey(),
                    payer: program.payer(),
                    system_program: system_program::ID,
                })
                .args(Initialize {is_call:plugin_state.is_call, strike:plugin_state.strike})
                .send(); 
            let signature = match signature {
                Ok(transaction) => transaction,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not create a state with given public key"),
                        ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::LogParseError(err)=> println_error(&err)
                    }
                    exit(1);
                }
            };
            println_name_value("Redeem Logic Digital State successfully create at", &plugin_config.pubkey());
            println_name_value("Transaction Id", &signature);                                       
        }
    }
}