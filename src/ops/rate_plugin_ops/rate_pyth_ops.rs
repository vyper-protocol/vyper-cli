use {
    std::process::exit,
    crate::args::rate_plugin_args,
    rate_plugin_args::rate_pyth_args:: {
        RatePythCommand,
        RatePythSubcommand
    },
    rate_pyth::{
        RateState,
        accounts::InitializeContext,
        instruction::Initialize
    },
    anchor_client::{
        Program,
        ClientError,
        solana_sdk:: {
            signer::keypair::Keypair,
            signer::Signer,
            system_program,
        },
        anchor_lang::prelude::AccountMeta
    },
    crate::utils:: {
        println_name_value,
        println_fair_value,
        println_error,
        println_aggregators
    },
    console::style,
};



pub fn handle_rate_pyth_command(rate_pyth_command: RatePythCommand, program: &Program) {
    let command = rate_pyth_command.command;
    match command {
        RatePythSubcommand::Fetch(fetch_state) => {
            let account:Result<RateState,ClientError> = program.account(fetch_state.state_id);
            let account = match account {
                Ok(rate_state) => rate_state,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find a rate switchboard state with given public key"),
                        ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::LogParseError(err)=> println_error(&err)
                    }
                    exit(1);
                }
            };
            print!("{} : [",style("fair value").bold());
            println_fair_value(&account.fair_value);
            println!("]");
            println_name_value("refreshed slot",&account.refreshed_slot);
            println_aggregators("pyth oracles", &account.pyth_oracles);
        }
        RatePythSubcommand::Create(create_state) => {
            let rate_pyth_state = Keypair::new(); 
            let oracles:Vec<AccountMeta> = create_state.oracles.into_iter().map(|rate_account| {
                AccountMeta::new_readonly(rate_account, false)
            }).collect();                            
            let signature = program.request()
                .signer(&rate_pyth_state)
                .accounts(InitializeContext {
                    rate_data: rate_pyth_state.pubkey(),
                    signer: program.payer(),
                    system_program: system_program::ID
                })
                .accounts(oracles)
                .args(Initialize {})
                .send(); 
            let signature = match signature {
                Ok(transaction) => transaction,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not create a rate pyth state with given public key"),
                        ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::LogParseError(err)=> println_error(&err)
                    }
                    exit(1);
                }
            };
            println_name_value("Rate Pyth Plugin State successfully create at", &rate_pyth_state.pubkey());
            println_name_value("Transaction Id", &signature);
        }
    }
}