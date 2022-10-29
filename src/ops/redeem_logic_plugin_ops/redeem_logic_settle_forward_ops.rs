use {
    std::process::exit,
    crate::args::redeem_logic_plugin_args,
    redeem_logic_plugin_args::redeem_logic_settle_forward_args:: {
        RedeemLogicSettleForwardCommand,
        RedeemLogicSettleForwardSubcommand
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
    redeem_logic_settled_forward:: {
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
};



pub fn handle_redeem_logic_settle_forward_command(redeem_logic_command: RedeemLogicSettleForwardCommand, program: &Program) {
    let command = redeem_logic_command.command;
    match command {
        RedeemLogicSettleForwardSubcommand::Fetch(fetch_state) => {
            let account:Result<RedeemLogicConfig,ClientError> = program.account(fetch_state.state_id);
            let account = match account {
                Ok(redeem_config) => redeem_config,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find a state with given public key"),
                        ClientError::AnchorError(_) => println_error("Anchor not working"),
                        ClientError::ProgramError(_) => println_error("Redeem Logic Settle Forward program is not working"),
                        ClientError::SolanaClientError(_) => println_error("Solana client is not working"),
                        ClientError::SolanaClientPubsubError(_) => println_error("Solana client is not working") ,
                        ClientError::LogParseError(_)=> println_error("Could not parse the given public key")
                    }
                    exit(1);
                }   
            };
            println_name_value("notional", &account.notional);
            println_name_value("is_linear", &account.is_linear);
            println_name_value("strike",&Decimal::deserialize(account.strike));
            println_name_value("is_standard", &account.is_standard);
            println_name_value("owner", &account.owner);
        },
        RedeemLogicSettleForwardSubcommand::Create(plugin_state) => {
            let plugin_config =  Keypair::new();
            let authority = program.payer();
            let signature = program.request()
                .signer(&plugin_config)
                .accounts(InitializeContext {
                    redeem_logic_config: plugin_config.pubkey(),
                    owner: authority,
                    payer: authority,
                    system_program: system_program::ID,
                })
                .args(Initialize { notional: plugin_state.notional,is_linear:plugin_state.is_linear, strike:plugin_state.strike, is_standard:plugin_state.is_standard})
                .send(); 
            let signature = match signature {
                Ok(transaction) => transaction,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find a state with given public key"),
                        ClientError::AnchorError(_) => println_error("Anchor not working"),
                        ClientError::ProgramError(_) => println_error("Redeem Logic Settle Forward program is not working"),
                        ClientError::SolanaClientError(_) => println_error("Solana client is not working"),
                        ClientError::SolanaClientPubsubError(_) => println_error("Solana client is not working") ,
                        ClientError::LogParseError(_)=> println_error("Could not parse the given input")
                    }
                    exit(1);
                }
            };
            println_name_value("Redeem Logic Setle Forward State successfully create at", &plugin_config.pubkey());
            println_name_value("Transaction Id", &signature);                                       
        }
    }
}