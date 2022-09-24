use crate::args::core_args;

use core_args::CoreCommand;
use core_args::CoreSubcommand;


pub fn handle_core_command(core_command: CoreCommand) {
    let command = core_command.command;
    match command {
        CoreSubcommand::Fetch(fetch_tranche) => {
           println!("{:?}",fetch_tranche.tranche_id);
        }
    }
}
