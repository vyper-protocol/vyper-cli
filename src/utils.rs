use {
    console::style,
    std::fmt::{Debug, Display},
    vyper_core::state::SlotTracking,
    rust_decimal::Decimal,
    anchor_client::solana_sdk::{
        pubkey:: Pubkey,
        signature::Signature
    },
    anchor_client::ClientError,
    std::process::exit,
    solana_cli_config::{CONFIG_FILE, Config},
    chrono::prelude::*,
    inquire::{InquireError}
};

pub fn println_name_value<T:Debug>(name: &str, value: &T) {
    println!(
        "{} : {:?} ",
        style(name).bold(),
        style(value),
    );
}

pub fn println_version<T:Display>(name: &str, value: &[T]) {
    println!(
        "{} : {}.{}.{}",
        style(name).bold(),
        style(&value[0]),
        style(&value[1]),
        style(&value[2])
    );
}

pub fn println_name_fair_value(name: &str, fair_value: &[[u8;16]], slot: &SlotTracking) {
    print!(
        "{} : {{ value: [",
        style(name).bold(),
    );
    println_fair_value(fair_value);
    print!("], slot_tracking: ",);
    println!("{:?} }}",slot);
}

pub fn println_error(err: &str) {
    println!(
        "{} : {}",
        style("error").red().bold(),
        style(err)
    );
}

pub fn get_solana_config() -> Config {
    let config_file = CONFIG_FILE.as_ref();
    let config_file = match config_file {
        Some(file) => file,
        None => {
            println_error("Could not read the config file");
            exit(1);
        }
    };
    let cli_config = Config::load(config_file);
    match cli_config {
        Ok(config) => config,
        Err(_) => {
            println_error("Could not load the config file");
            exit(1);
        }
    }
}

pub fn println_fair_value(fair_value: &[[u8; 16]]) {
    let mut first: bool = true;
    for value in fair_value {
        if !first {
            print!(",");
        }
        print!(
            "{}",
            style(&Decimal::deserialize(*value)),
        );
        first=false;
    }
}

pub fn println_aggregators(name: &str, aggregators: &[Option<Pubkey>; 10]) {
    print!(
        "{} : [",
        style(name).bold(),
    );
    let mut first: bool = true;
    for value in aggregators {
        match value {
            Some(key) => {
                if !first {
                    print!(",{}",key)
                }
                else {
                    print!("{}",key);
                }
            }
            None => break
        }
        first=false;
    }
    println!("]")
}

pub fn println_date(name: &str, timestamp: &i64) {
    let naive = NaiveDateTime::from_timestamp_opt(*timestamp, 0).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M").to_string();
    println!(
        "{} : {:?} ",
        style(name).bold(),
        style(newdate),
    );
}

pub fn println_beneficiary_value(name: &str, value: &Option<Pubkey>) {
    if !value.is_none() {
        println!(
            "{} : {:?} ",
            style(name).bold(),
            style(value.unwrap()),
        );
    } else {
        println!(
            "{} : {:?} ",
            style(name).bold(),
            style("None"),
        );
    }
}

pub fn inquire_input<T>(input: Result<T,InquireError>) -> T{
    match input {
        Ok(value) => value,
        Err(_) => {
            println_error("Could not parse the given input");
            exit(1);
        }
    }
}

pub fn error_handler(signature: Result<Signature,ClientError>) -> Signature {
    match signature {
        Ok(transaction) => transaction,
        Err(err) => {
            match err {
                ClientError::AccountNotFound => println_error("Could not find a account with given public key"),
                ClientError::AnchorError(err) => println!("{} : {}",style("error").red().bold(),err),
                ClientError::ProgramError(err) => println!("{} : {}",style("error").red().bold(),err),
                ClientError::SolanaClientError(err) => println!("{} : {}",style("error").red().bold(),err),
                ClientError::SolanaClientPubsubError(err) => println!("{} : {}",style("error").red().bold(),err),
                ClientError::LogParseError(err)=> println_error(&err)
            }
            exit(1);
        }
    }
}

