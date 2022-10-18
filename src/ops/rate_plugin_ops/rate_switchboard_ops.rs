use {
    std::process::exit,
    crate::args::rate_plugin_args,
    rate_plugin_args::rate_switchboard_args:: {
        RateSwitchboardCommand,
        RateSwitchboardSubcommand
    },
    rate_switchboard::RateState,
    anchor_client::{
        Program,
        ClientError
    },
    crate::utils:: {
        println_name_value,
        println_fair_value,
        println_error,
        println_switchboard_aggregators
    },
    console::style,
};



pub fn handle_rate_switchboard_command(redeem_logic_command: RateSwitchboardCommand, program: &Program) {
    let command = redeem_logic_command.command;
    match command {
        RateSwitchboardSubcommand::Fetch(fetch_state) => {
            let account:Result<RateState,ClientError> = program.account(fetch_state.state_id);
            let account = match account {
                Ok(rate_state) => rate_state,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find a state with given public key"),
                        ClientError::AnchorError(_) => println_error("Anchor not working"),
                        ClientError::ProgramError(_) => println_error("Rate Switchboard program is not working"),
                        ClientError::SolanaClientError(_) => println_error("Solana client is not working"),
                        ClientError::SolanaClientPubsubError(_) => println_error("Solana client is not working") ,
                        ClientError::LogParseError(_)=> println_error("Could not parse the given public key")
                    }
                    exit(1);
                }
            };
            print!("{} : [",style("fair value").bold());
            println_fair_value(account.fair_value);
            println!("]");
            println_name_value("refreshed slot",&account.refreshed_slot);
            println_switchboard_aggregators("switchboard aggregators", &account.switchboard_aggregators)
        }
    }
}