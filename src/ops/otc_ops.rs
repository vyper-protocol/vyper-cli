use {
    std::process::exit,
    crate::args::otc_args,
    crate::utils::{
        println_name_value,
        println_version,
        println_error,
        println_date,
        println_beneficiary_value
    },
    otc_args:: {
        OtcCommand,
        OtcSubcommand
    },
    anchor_client::{
        Program,
        ClientError
    },
    vyper_otc::state::OtcState,
    console::style
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
                        ClientError::AccountNotFound => println_error("Could not otc state with given public key"),
                        ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                        ClientError::LogParseError(err)=> println_error(&err)
                    }
                    exit(1);
                }
            };
            println_date("created", &account.created);
            println_date("deposit start", &account.deposit_start);
            println_date("deposit end", &account.deposit_end);
            println_date("settle start", &account.settle_start);
            println_name_value("settle executed", &account.settle_executed);
            println_name_value("junior deposit amount", &account.junior_deposit_amount);
            println_name_value("senior deposit amount", &account.senior_deposit_amount);
            println_beneficiary_value("junior side beneficiary", &account.junior_side_beneficiary);
            println_beneficiary_value("senior side beneficiary", &account.senior_side_beneficiary);
            println_name_value("vyper tranche config", &account.vyper_tranche_config);
            println_name_value("vyper core", &account.vyper_core);
            println_name_value("senior reserve token account", &account.otc_senior_reserve_token_account);
            println_name_value("junior reserve token account", &account.otc_junior_reserve_token_account);
            println_name_value("senior tranche token account", &account.otc_senior_tranche_token_account);
            println_name_value("junior tranche token account", &account.otc_junior_tranche_token_account);
            println_name_value("otc authority",&account.otc_authority);
            println_name_value("authority seed", &account.authority_seed);
            println_name_value("authority bump", &account.authority_bump);
            println_version("version",&account.version);
        }
    }
}