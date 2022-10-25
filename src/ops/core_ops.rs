use {
    std::process::exit,
    crate::args::core_args,
    crate::utils::{
        println_name_value,
        println_version,
        println_tranche_fair_value,
        println_reserve_fair_value,
        println_error
    },
    core_args:: {
        CoreCommand,
        CoreSubcommand,
    },
    anchor_client::{
        Program,
        ClientError
    },
    vyper_core:: {
        state::TrancheConfig
    }
};



pub fn handle_core_command(core_command: CoreCommand, program: &Program) {
    let command = core_command.command;
    match command {
        CoreSubcommand::Fetch(fetch_tranche) => {
            let account:Result<TrancheConfig,ClientError> = program.account(fetch_tranche.tranche_id);
            let account = match account {
                Ok(tranche_config) => tranche_config,
                Err(err) => {
                    match err {
                        ClientError::AccountNotFound => println_error("Could not find tranche cofiguration with given public key"),
                        ClientError::AnchorError(_) => println_error("Anchor not working"),
                        ClientError::ProgramError(_) => println_error("Vyper core program is not working"),
                        ClientError::SolanaClientError(_) => println_error("Solana client is not working"),
                        ClientError::SolanaClientPubsubError(_) => println_error("Solana client is not working") ,
                        ClientError::LogParseError(_)=> println_error("Could not parse the given public key")
                    }
                    exit(1);
                }
            };
            println_name_value("reserve mint",&account.reserve_mint);
            println_name_value("reserve",&account.reserve);
            println_name_value("deposited quantity", &account.tranche_data.deposited_quantity);
            println_reserve_fair_value("reserve fair value",  &account.tranche_data.reserve_fair_value);
            println_tranche_fair_value("tranche fair value", &account.tranche_data.tranche_fair_value);
            println_name_value("halt_flags", &account.tranche_data.get_halt_flags().expect("Could not get tranche halt flags"));
            println_name_value("owner restricted ix", &account.tranche_data.get_owner_restricted_ixs().expect("Could not get owner restricted ix"));
            println_name_value("deposited cap", &account.tranche_data.deposit_cap);
            println_name_value("fee to collect quantity", &account.tranche_data.fee_to_collect_quantity);
            println_name_value("senior tranche_mint",&account.senior_tranche_mint);
            println_name_value("junior tranche_mint", &account.junior_tranche_mint);
            println_name_value("tranche authority", &account.tranche_authority);
            println_name_value("authority seed", &account.authority_seed);
            println_name_value("authority bump", &account.authority_bump);
            println_name_value("owner", &account.owner);
            println_name_value("rate program", &account.rate_program);
            println_name_value("rate program state", &account.rate_program_state);
            println_name_value("redeem logic program", &account.redeem_logic_program);
            println_name_value("redeem logic program state", &account.redeem_logic_program_state);
            println_version("version", &account.version);
        }
    }
}