use console::style;
use std::fmt::{Debug, Display};
use vyper_core::state::{
    TrancheFairValue, ReserveFairValue
};
use rust_decimal::{
    Decimal
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


pub fn println_error(err: &str) {
    println!(
        "{} : {}",
        style("error").red().bold(),
        style(err)
    );
}

