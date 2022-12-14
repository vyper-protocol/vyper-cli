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
        Client,
        Program,
        Cluster,
        ClientError,
        solana_sdk:: {
            signer::keypair::Keypair,
            signer::Signer,
            pubkey:: Pubkey,
            system_program,
            instruction::AccountMeta
        },
    },
    vyper_otc::state::OtcState,
    rate_switchboard::{
        accounts::InitializeContext,
        instruction::Initialize,
    },
    console::style,
    requestty::{Question,prompt_one},
    crate::{RATE_SWITCHBOARD}
};




pub fn handle_otc_command(otc_command: OtcCommand, otc_program: &Program, core_program: &Program, client: &Client, cluster: Cluster) {
    let command = otc_command.command;
    match command {
        OtcSubcommand::Fetch(fetch_otc) => {
            let account:Result<OtcState,ClientError> = otc_program.account(fetch_otc.state_id);
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

        OtcSubcommand::Create => {
           
            let rate_plugin_question = Question::select("rate plugin")
                    .message("Which rate plugin to select?")
                    .choice("Rate Switchboard")
                    .choice("Rate Pyth")
                    .build();
            let rate_plugin_answer = prompt_one(rate_plugin_question).unwrap();
            let rate_plugin_type = rate_plugin_answer.as_list_item().unwrap();

            
            let redeem_plugin_question = Question::select("redeem plugin")
                    .message("Which redeem plugin to select?")
                    .choice("Redeem Forward")
                    .choice("Redeem Settled Forward")
                    .choice("Redeem Digital")
                    .choice("Redeem Vanilla Option")
                    .build();
            let redeem_plugin_answer = prompt_one(redeem_plugin_question).unwrap();
            let redeem_plugin_type = redeem_plugin_answer.as_list_item().unwrap();

            if rate_plugin_type.text == String::from("Rate Switchboard") {
                // rate switchboard program
                let rate_switchboard_program_id: Pubkey = Pubkey::new(&bs58::decode(&RATE_SWITCHBOARD).into_vec().expect("Invalid rate switchboard program id"));
                let rate_switchboard_program = client.program(rate_switchboard_program_id);

                let rate_switchboard_state = Keypair::new();
                let rateAccounts = match cluster.url() {
                    "https://api.devnet.solana.com" =>  "9LNYQZLJG5DAyeACCTzBFG6H3sDhehP5xtYLdhrZtQkA",
                    _ => "7Y3nWv5B2rLiDBsNpkfXqa4cbJqszJos2sZVutF8R3FE"
                };

                let signature = rate_switchboard_program.request()
                    .signer(&rate_switchboard_state)
                    .accounts(InitializeContext {
                        rate_data: rate_switchboard_state.pubkey(),
                        signer: rate_switchboard_program.payer(),
                        system_program: system_program::ID
                    })
                    .accounts(AccountMeta::new_readonly(Pubkey::new(&bs58::decode(&rateAccounts).into_vec().expect("Invalid otc program id")), false))
                    .args(Initialize {})
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
                println_name_value("Rate Switchboard Plugin State successfully create at", &rate_switchboard_state.pubkey());
                println_name_value("Transaction Id", &signature);
                    

            } else if rate_plugin_type.text == String::from("Rate Pyth") {

            } else {
                //error
            }

            if redeem_plugin_type.text == String::from("Redeem Forward") {
                println!("{}","RATE SWITCH");
            } else if redeem_plugin_type.text == String::from("Redeem Settled Forward") {

            } else if redeem_plugin_type.text == String::from("Redeem Digital") {
                
            } else if redeem_plugin_type.text == String::from("Redeem Vanilla") {

            } else {
                //error
            }


            println!("{:?} {:?}",rate_plugin_type.text, redeem_plugin_type.text);

            
        }
    }
}