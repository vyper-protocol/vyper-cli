use console::style;
use std::fmt::{Debug, Display};
use vyper_core::state::{
    TrancheFairValue, ReserveFairValue
};
use rust_decimal::{
    Decimal,
};

use anchor_client::solana_sdk::{
        pubkey:: Pubkey
};

use std::process::exit;
use solana_cli_config::{CONFIG_FILE, Config};



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


pub fn println_tranche_fair_value(name: &str, fair_value: &TrancheFairValue) {
    print!(
        "{} : {{ value: [",
        style(name).bold(),
    );
    let mut first: bool = true;
    for value in fair_value.value {
        if !first {
            print!(",");
        }
        print!(
            "{}",
            style(&Decimal::deserialize(value)),
        );
        first=false;
    }
    print!("], slot_tracking: ",);
    println!("{:?} }}",fair_value.slot_tracking);
}


pub fn println_reserve_fair_value(name: &str, fair_value: &ReserveFairValue) {
    print!(
        "{} : {{ value: [",
        style(name).bold(),
    );
    println_fair_value(fair_value.value);
    print!("], slot_tracking: ",);
    println!("{:?} }}",fair_value.slot_tracking);
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

pub fn println_fair_value(fair_value: [[u8; 16]; 10]) {
    let mut first: bool = true;
    for value in fair_value {
        if !first {
            print!(",");
        }
        print!(
            "{}",
            style(&Decimal::deserialize(value)),
        );
        first=false;
    }
}

pub fn println_switchboard_aggregators(name: &str, aggregators: &[Option<Pubkey>; 10]) {
    print!(
        "{} : [",
        style(name).bold(),
    );
    for value in aggregators {
        match value {
            Some(key) => print!("{},",key),
            None => print!("")
        }
    }
    println!("]")
}

