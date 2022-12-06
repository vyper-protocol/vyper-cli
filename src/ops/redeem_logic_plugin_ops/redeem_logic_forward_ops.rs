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
    },
    redeem_logic_forward::RedeemLogicConfig,
    crate::utils:: {
        println_name_value,
        println_error
    },
    rust_decimal::{
        Decimal
    }
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
                        ClientError::AccountNotFound => println_error("Could not find a state with given public key"),
                        ClientError::AnchorError(_) => println_error("Anchor not working"),
                        ClientError::ProgramError(_) => println_error("Redeem Logic Forward program is not working"),
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
        }
    }
}