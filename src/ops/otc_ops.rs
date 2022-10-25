use {
    std::process::exit,
    crate::args::otc_args,
    crate::utils::{
        println_name_value,
        println_version,
        println_error
    },
    otc_args:: {
        OtcCommand,
        OtcSubcommand
    },
    anchor_client::{
        Program,
        ClientError
    },
    vyper_otc::state::OtcState
};



pub fn handle_otc_command(otc_command: OtcCommand, program: &Program) {
    let command = otc_command.command;
    match command {
        OtcSubcommand::Fetch(fetch_otc) => {
            let account:Result<OtcState,ClientError> = program.account(fetch_otc.state_id);
            let account = match account {
                Ok(otc_state) => otc_state,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find state of otc with given public key"),
                        ClientError::AnchorError(_) => println_error("Anchor not working"),
                        ClientError::ProgramError(_) => println_error("Vyper Otc program is not working"),
                        ClientError::SolanaClientError(_) => println_error("Solana client is not working"),
                        ClientError::SolanaClientPubsubError(_) => println_error("Solana client is not working") ,
                        ClientError::LogParseError(_)=> println_error("Could not parse the given public key")
                    }
                    exit(1);
                }
            };
            println_name_value("created", &account.created);
            println_name_value("deposit start", &account.deposit_start);
            println_name_value("deposit end", &account.deposit_end);
            println_name_value("settle start", &account.settle_start);
            println_name_value("settle executed", &account.settle_executed);
            println_name_value("junior deposit amount", &account.junior_deposit_amount);
            println_name_value("senior deposit amount", &account.senior_deposit_amount);
            println_name_value("junior side beneficiary", &account.junior_side_beneficiary);
            println_name_value("senior side beneficiary", &account.senior_side_beneficiary);
            println_name_value("vyper tranche config", &account.vyper_tranche_config);
            println_name_value("vyper core", &account.vyper_core);
            println_name_value("senior reserve token account", &account.otc_senior_reserve_token_account);
            println_name_value("junior reserve token account", &account.otc_junior_reserve_token_account);
            println_name_value("senior tranche token account", &account.otc_senior_tranche_token_account);
            println_name_value("junior tranche token account", &account.otc_junior_tranche_token_account);
            println_name_value("otc authority",&account.otc_authority);
            println_name_value("authority seed", &account.authority_seed);
            println_name_value("authority bump", &account.authority_bump);
            println_name_value("description", &account.description);
            println_version("version",&account.version);
        }
    }
}