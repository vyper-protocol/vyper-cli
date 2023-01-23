use {
    std::process::exit,
    crate::args::otc_args,
    crate::utils::{
        println_name_value,
        println_version,
        println_error,
        println_date,
        println_beneficiary_value,
        inquire_input,
        error_handler
    },
    otc_args:: {
        OtcCommand,
        OtcSubcommand
    },
    anchor_client::{
        Client,
        Cluster,
        Program,
        RequestBuilder,
        ClientError,
        solana_sdk:: {
            signer::keypair::Keypair,
            signer::Signer,
            pubkey:: Pubkey,
            system_program,
            instruction::AccountMeta,
            sysvar
        },
    },
    vyper_otc::{
        state::OtcState,
        accounts::InitializeContext as OtcInitializeContext,
        instruction::Initialize as OtcInitialize,
        instructions::InitializeInputData as OtcInitialInput
    },
    rate_switchboard::{
        accounts::InitializeContext as RateSwitchInitializeContext,
        instruction::Initialize as RateSwitchInitialize,
    },
    rate_pyth::{
        accounts::InitializeContext as RatePythInitializeContext,
        instruction::Initialize as RatePythInitialize,
    },
    redeem_logic_forward::{
        accounts::InitializeContext as RedeemLogicForwardInitializeContext,
        instruction::Initialize as RedeemLogicForwardInitialize,
    },
    redeem_logic_settled_forward::{
        accounts::InitializeContext as RedeemLogicSettledForwardInitializeContext,
        instruction::Initialize as RedeemLogicSettledForwardInitialize,
    },
    redeem_logic_digital::{
        accounts::InitializeContext as RedeemLogicDigitalInitializeContext,
        instruction::Initialize as RedeemLogicDigitalInitialize,
    },
    redeem_logic_vanilla_option::{
        accounts::InitializeContext as RedeemLogicVanillaOptionInitializeContext,
        instruction::Initialize as RedeemLogicVanillaOptionInitialize,
    },
    vyper_core::{
        accounts::InitializeContext as CoreInitializeContext,
        instruction::Initialize as CoreInitialize,
        instructions::InitializeInput as CoreInitialInput
    },
    console::style,
    crate::{RATE_SWITCHBOARD, RATE_PYTH,REDEEM_LOGIC_FORWARD,REDEEM_LOGIC_SETTLE_FORWARD,REDEEM_LOGIC_DIGITAL,REDEEM_LOGIC_VANILLA_OPTION},
    inquire::{CustomType,Select,Confirm},
    chrono::{
        NaiveDateTime
    }
};

const USDC_MAINNET: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const DEV_USD: &str = "7XSvJnS19TodrQJSbjUR6tEGwmYyL1i9FX7Z5ZQHc53W";
const RATE_SWITCHBOARD_SOLUSD_MAINNET :&str = "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR";
const RATE_SWITCHBOARD_SOLUSD_DEVNET: &str = "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR";
const RATE_PYTH_SOLUSD_ORACLE_MAINNET: &str = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";
const RATE_PYTH_SOLUSD_ORACLE_DEVNET: &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";

pub fn handle_otc_command(otc_command: OtcCommand, otc_program: &Program, core_program: &Program, client: &Client, current_cluster: &Cluster) {
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

            let default_collateral_mint = match &current_cluster {
                Cluster::Mainnet => USDC_MAINNET,
                _ => DEV_USD
            };

            // collateral mint input
            let collateral_mint= CustomType::<Pubkey>::new("Collateral Mint")
                .with_default(Pubkey::new(&bs58::decode(&default_collateral_mint).into_vec().expect("Invalid collateral mint")))
                .with_error_message("Please type enter valid Public key")
                .prompt();
            let collateral_mint = inquire_input(collateral_mint);
            
            
            // rate plugin choice
            let rate_plugin_options = vec![
                "Rate Switchboard",
                "Rate Pyth",
            ];
            let rate_plugin_choice = Select::new("Which rate plugin to choose?", rate_plugin_options).prompt();
            let rate_plugin_choice = inquire_input(rate_plugin_choice);
            
            // rate plugin details
            let rate_plugin_transaction: RequestBuilder;
            let rate_plugin_state: Keypair = Keypair::new();
            let rate_plugin_program: Program;


            if rate_plugin_choice == "Rate Switchboard" {
                // input for rate switcboard
                let rate_switchboard_program_id: Pubkey = Pubkey::new(&bs58::decode(&RATE_SWITCHBOARD).into_vec().expect("Invalid rate switchboard program id"));
                rate_plugin_program = client.program(rate_switchboard_program_id);
                let tot_aggregators = CustomType::<i64>::new("How many aggregators to use?")
                    .with_error_message("Please type a valid number")
                    .with_help_message("Maximum 10 aggregators are supported")
                    .prompt();
                let tot_aggregators = inquire_input(tot_aggregators);
                let mut cnt=0;
                let mut rate_accounts = Vec::new();
                
                let default_aggregator = match &current_cluster {
                    Cluster::Mainnet => RATE_SWITCHBOARD_SOLUSD_MAINNET,
                    _ => RATE_SWITCHBOARD_SOLUSD_DEVNET
                };

                while cnt<tot_aggregators {
                    let mut message = String::from("Aggregator #");
                    message+=&(cnt+1).to_string();
                    let aggregator = CustomType::<Pubkey>::new(&message)
                        .with_default(Pubkey::new(&bs58::decode(&default_aggregator).into_vec().expect("Invalid rate switchboard aggregator")))
                        .with_error_message("Please enter a vaid Public key")
                        .prompt();
                    let aggregator = inquire_input(aggregator);
                    rate_accounts.push(aggregator);
                    cnt+=1;
                }
                let aggregators:Vec<AccountMeta> = rate_accounts.into_iter().map(|rate_account| {
                    AccountMeta::new_readonly(rate_account, false)
                }).collect(); 
                
                // transaction builder
                rate_plugin_transaction = rate_plugin_program.request()
                    .signer(&rate_plugin_state)
                    .accounts(RateSwitchInitializeContext {
                        rate_data: rate_plugin_state.pubkey(),
                        signer: rate_plugin_program.payer(),
                        system_program: system_program::ID
                    })
                    .accounts(aggregators)
                    .args(RateSwitchInitialize {});

            } else if rate_plugin_choice == "Rate Pyth" {
                let rate_pyth_program_id: Pubkey = Pubkey::new(&bs58::decode(&RATE_PYTH).into_vec().expect("Invalid rate pyth program id"));
                rate_plugin_program = client.program(rate_pyth_program_id);
                let tot_oracles = CustomType::<i64>::new("How many oracles to use?")
                    .with_error_message("Please type a valid number")
                    .with_help_message("Maximum 10 oracles are supported")
                    .prompt();
                let tot_oracles = inquire_input(tot_oracles);
                let mut cnt=0;
                let mut rate_accounts = Vec::new();
                let default_oracle = match &current_cluster {
                    Cluster::Mainnet => RATE_PYTH_SOLUSD_ORACLE_MAINNET,
                    _ => RATE_PYTH_SOLUSD_ORACLE_DEVNET
                };
                while cnt<tot_oracles {
                    let mut message = String::from("Oracle #");
                    message+=&(cnt+1).to_string();
                    let oracle = CustomType::<Pubkey>::new(&message)
                        .with_error_message("Please enter a vaid Public key")
                        .with_default(Pubkey::new(&bs58::decode(&default_oracle).into_vec().expect("Invalid rate pyth oracle")))
                        .prompt();
                    let oracle = inquire_input(oracle);  
                    rate_accounts.push(oracle);
                    cnt+=1;
                }
                let oracles:Vec<AccountMeta> = rate_accounts.into_iter().map(|rate_account| {
                    AccountMeta::new_readonly(rate_account, false)
                }).collect();  
                
                // transaction builder
                rate_plugin_transaction = rate_plugin_program.request()
                    .signer(&rate_plugin_state)
                    .accounts(RatePythInitializeContext {
                        rate_data: rate_plugin_state.pubkey(),
                        signer: rate_plugin_program.payer(),
                        system_program: system_program::ID
                    })
                    .accounts(oracles)
                    .args(RatePythInitialize {});
                
            } else {
                println_error("Please choose a valid Rate Plugin");
                exit(1);
            }

            // redeem plugin choice
            let redeem_plugin_options = vec![
                "Redeem Forward",
                "Redeem Settled Forward",
                "Redeem Digital",
                "Redeem Vanilla Option"
            ];
            let redeem_plugin_choice = Select::new("Which redeem plugin to choose?", redeem_plugin_options).prompt();
            let redeem_plugin_choice = inquire_input(redeem_plugin_choice);
            // redeem plugin details
            let redeem_plugin_transaction: RequestBuilder;
            let redeem_plugin_state: Keypair = Keypair::new();
            let redeem_plugin_program: Program;

            if redeem_plugin_choice == "Redeem Forward" {
                let redeem_logic_forward_program_id: Pubkey = Pubkey::new(&bs58::decode(&REDEEM_LOGIC_FORWARD).into_vec().expect("Invalid redeem logic forward program id"));
                redeem_plugin_program = client.program(redeem_logic_forward_program_id);
                let notional = CustomType::<u64>::new("Notional")
                    .with_error_message("Please type a valid number")
                    .prompt();
                let notional=inquire_input(notional);
                let strike = CustomType::<f64>::new("Strike")
                    .with_error_message("Please type a valid number")
                    .prompt();
                let strike=inquire_input(strike);
                let is_linear = Confirm::new("Is Linear?")
                    .with_default(true)
                    .prompt();
                let is_linear=inquire_input(is_linear);
                
                // transaction builder
                redeem_plugin_transaction = redeem_plugin_program.request()
                    .signer(&redeem_plugin_state)
                    .accounts(RedeemLogicForwardInitializeContext {
                        redeem_logic_config: redeem_plugin_state.pubkey(),
                        payer: redeem_plugin_program.payer(),
                        system_program: system_program::ID,
                    })
                    .args(RedeemLogicForwardInitialize { notional,is_linear,strike});

            } else if redeem_plugin_choice == "Redeem Settled Forward" {
                let redeem_logic_settle_forward_program_id: Pubkey = Pubkey::new(&bs58::decode(&REDEEM_LOGIC_SETTLE_FORWARD).into_vec().expect("Invalid redeem logic forward program id"));
                redeem_plugin_program = client.program(redeem_logic_settle_forward_program_id);
                let notional = CustomType::<u64>::new("Notional")
                    .with_error_message("Please type a valid number")
                    .prompt();
                let notional=inquire_input(notional);
                let strike = CustomType::<f64>::new("Strike")
                    .with_error_message("Please type a valid number")
                    .prompt();
                let strike=inquire_input(strike);
                let is_linear = Confirm::new("Is Linear?")
                    .with_default(true)
                    .prompt();
                let is_linear=inquire_input(is_linear);
                let is_standard = Confirm::new("Is Standard?")
                    .with_default(true)
                    .prompt();
                let is_standard=inquire_input(is_standard);
                
                // transaction builder
                redeem_plugin_transaction = redeem_plugin_program.request()
                    .signer(&redeem_plugin_state)
                    .accounts(RedeemLogicSettledForwardInitializeContext {
                        redeem_logic_config: redeem_plugin_state.pubkey(),
                        payer: redeem_plugin_program.payer(),
                        system_program: system_program::ID,
                    })
                    .args(RedeemLogicSettledForwardInitialize { notional,is_linear, strike, is_standard});
                
            } else if redeem_plugin_choice == "Redeem Digital" {
                let redeem_logic_digital_program_id: Pubkey = Pubkey::new(&bs58::decode(&REDEEM_LOGIC_DIGITAL).into_vec().expect("Invalid redeem logic digital program id"));
                redeem_plugin_program = client.program(redeem_logic_digital_program_id);
                let strike = CustomType::<f64>::new("Strike")
                    .with_error_message("Please type a valid number")
                    .prompt();
                let strike=inquire_input(strike);
                let is_call = Confirm::new("Is Call?")
                    .with_default(true)
                    .prompt();
                let is_call=inquire_input(is_call);

                // transaction builder
                redeem_plugin_transaction = redeem_plugin_program.request()
                    .signer(&redeem_plugin_state)
                    .accounts(RedeemLogicDigitalInitializeContext {
                        redeem_logic_config: redeem_plugin_state.pubkey(),
                        payer: redeem_plugin_program.payer(),
                        system_program: system_program::ID,
                    })
                    .args(RedeemLogicDigitalInitialize {is_call, strike});
                    
            } else if redeem_plugin_choice == "Redeem Vanilla Option" {
                let redeem_logic_vanilla_option_program_id: Pubkey = Pubkey::new(&bs58::decode(&REDEEM_LOGIC_VANILLA_OPTION).into_vec().expect("Invalid redeem logic forward program id"));
                redeem_plugin_program = client.program(redeem_logic_vanilla_option_program_id);
                let notional = CustomType::<u64>::new("Notional")
                    .with_error_message("Please type a valid number")
                    .prompt();
                let notional=inquire_input(notional);
                let strike = CustomType::<f64>::new("Strike")
                    .with_error_message("Please type a valid number")
                    .prompt();
                let strike=inquire_input(strike);
                let is_linear = Confirm::new("Is Linear?")
                    .with_default(true)
                    .prompt();
                let is_linear=inquire_input(is_linear);
                let is_call = Confirm::new("Is Call?")
                    .with_default(true)
                    .prompt();
                 let is_call=inquire_input(is_call);
                
                // transaction builder 
                redeem_plugin_transaction = redeem_plugin_program.request()
                    .signer(&redeem_plugin_state)
                    .accounts(RedeemLogicVanillaOptionInitializeContext {
                        redeem_logic_config: redeem_plugin_state.pubkey(),
                        payer: redeem_plugin_program.payer(),
                        system_program: system_program::ID,
                    })
                    .args(RedeemLogicVanillaOptionInitialize { notional, is_call, is_linear, strike});
            } else {
                println_error("Please choose a valid Redeem Logic Plugin");
                exit(1);
            }

            let otc_state = Keypair::new();
            let (otc_authority, _otc_bump) = Pubkey::find_program_address(&[otc_state.pubkey().as_ref(),b"authority"], &otc_program.id());

	        let junior_tranche_mint = Keypair::new();
            let senior_tranche_mint = Keypair::new();
            let tranche_config = Keypair::new();
            let (tranche_authority,_tranche_bump) = Pubkey::find_program_address(
                &[tranche_config.pubkey().as_ref(),b"authority"],
                &core_program.id()
            );
            let (reserve, _res_bump) = Pubkey::find_program_address(&[tranche_config.pubkey().as_ref(), collateral_mint.as_ref()], &core_program.id());
            let initial_input: CoreInitialInput = CoreInitialInput{
                tranche_mint_decimals: 6,
                halt_flags : 0,
                owner_restricted_ixs : (1<<0) | (1<<2)
            };
            let core_transaction = core_program.request()
                .accounts(CoreInitializeContext{
                    payer: core_program.payer(),
                    owner: otc_authority,
                    tranche_config: tranche_config.pubkey(),
                    tranche_authority,
                    rate_program: rate_plugin_program.id(),
                    rate_program_state: rate_plugin_state.pubkey(),
                    redeem_logic_program: redeem_plugin_program.id(),
                    redeem_logic_program_state: redeem_plugin_state.pubkey(),
                    reserve_mint: collateral_mint,
                    reserve,
                    junior_tranche_mint: junior_tranche_mint.pubkey(),
                    senior_tranche_mint: senior_tranche_mint.pubkey(),
                    system_program: system_program::ID,
                    token_program: spl_token::ID,
                    rent: sysvar::rent::ID
                })
                .args(CoreInitialize{input_data:initial_input})
                .signer(&junior_tranche_mint)
                .signer(&senior_tranche_mint)
                .signer(&tranche_config);

            let senior_deposit_amount = CustomType::<u64>::new("Senior Deposit Amount")
                .with_error_message("Please type a amount")
                .prompt();
            let senior_deposit_amount = inquire_input(senior_deposit_amount);
            let junior_deposit_amount = CustomType::<u64>::new("Junior Deposit Amount")
                .with_error_message("Please type a amount")
                .prompt();
            let junior_deposit_amount = inquire_input(junior_deposit_amount);

            let default_deposit_start = chrono::Local::now()-chrono::Duration::hours(1);
            let deposit_start = CustomType::<NaiveDateTime>::new("Deposit Start")
                .with_placeholder("yyyy-mm-dd hh:mm:ss")
                .with_default(default_deposit_start.naive_local())
                .with_parser(&|i| NaiveDateTime::parse_from_str(i, "%Y-%m-%d %H:%M:%S").map_err(|_| ()))
                .with_error_message("Please type a valid date and time")
                .prompt();
            let deposit_start = inquire_input(deposit_start);
            let default_deposit_end = chrono::Local::now()+chrono::Duration::minutes(5);
            let deposit_end = CustomType::<NaiveDateTime>::new("Deposit End")
                .with_default(default_deposit_end.naive_local())
                .with_placeholder("yyyy-mm-dd hh:mm:ss")
                .with_parser(&|i| NaiveDateTime::parse_from_str(i, "%Y-%m-%d %H:%M:%S").map_err(|_| ()))
                .with_error_message("Please type a valid date and time")
                .prompt();
            let deposit_end = inquire_input(deposit_end);
            let settle_start_default = chrono::Local::now()+chrono::Duration::minutes(15);
            let settle_start = CustomType::<NaiveDateTime>::new("Settle Start")
                .with_placeholder("yyyy-mm-dd hh:mm:ss")
                .with_default(settle_start_default.naive_local())
                .with_parser(&|i| NaiveDateTime::parse_from_str(i, "%Y-%m-%d %H:%M:%S").map_err(|_| ()))
                .with_error_message("Please type a valid date and time")
                .prompt();
            let settle_start = inquire_input(settle_start);
            
            let otc_senior_reserve_token_account = Keypair::new();
            let otc_junior_reserve_token_account = Keypair::new();
            let otc_senior_tranche_token_account = Keypair::new();
            let otc_junior_tranche_token_account = Keypair::new();
            
            let otc_initial_input = OtcInitialInput{
                senior_deposit_amount,
                junior_deposit_amount,
                deposit_start: Option::Some(deposit_start.timestamp()),
                deposit_end: deposit_end.timestamp(),
                settle_start: settle_start.timestamp(),
                description: [0; 128]
            };
            
            let otc_transaction = otc_program.request()
                .accounts(OtcInitializeContext{
                    signer: otc_program.payer(),
                    reserve_mint: collateral_mint,
                    otc_authority,
                    otc_state: otc_state.pubkey(),
                    junior_tranche_mint: junior_tranche_mint.pubkey(),
                    senior_tranche_mint: senior_tranche_mint.pubkey(),
                    otc_senior_reserve_token_account: otc_senior_reserve_token_account.pubkey(),
                    otc_junior_reserve_token_account: otc_junior_reserve_token_account.pubkey(),
                    otc_senior_tranche_token_account: otc_senior_tranche_token_account.pubkey(),
                    otc_junior_tranche_token_account: otc_junior_tranche_token_account.pubkey(),
                    vyper_tranche_config: tranche_config.pubkey(),
                    vyper_core: core_program.id(),
                    system_program: system_program::ID,
                    token_program: spl_token::ID,
                    rent: sysvar::rent::ID
                })
                .args(OtcInitialize{input_data:otc_initial_input})
                .signer(&otc_state)
                .signer(&otc_senior_reserve_token_account)
                .signer(&otc_junior_reserve_token_account)
                .signer(&otc_senior_tranche_token_account)
                .signer(&otc_junior_tranche_token_account);

            println!("Tx Sending ...");

            let rate_plugin_transaction = rate_plugin_transaction.send();
            let _rate_plugin_signature = error_handler(rate_plugin_transaction);

            let redeem_plugin_transaction = redeem_plugin_transaction.send();
            let _redeem_plugin_signature = error_handler(redeem_plugin_transaction);

            let core_transaction = core_transaction.send();
            let _core_signature = error_handler(core_transaction);

            let otc_transaction = otc_transaction.send();
            let otc_signature = error_handler(otc_transaction);
            
            println_name_value("Otc contract successfully created at", &otc_state.pubkey());
            println_name_value("Transaction Id", &otc_signature);
        }
    }
}