use {
    crate::args::core_args,
    core_args:: {
        CoreCommand,
        CoreSubcommand,
    },
    anchor_client::{
        Program,
    },
    vyper_core:: {
        state::TrancheConfig
    }
};

// TODO
// impl fmt::Display for TrancheConfig {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} - {}", self.authority_seed)
//     }
// }

pub fn handle_core_command(core_command: CoreCommand, program: &Program) {
    let command = core_command.command;
    match command {
        CoreSubcommand::Fetch(fetch_tranche) => {
            let account: TrancheConfig = program.account(fetch_tranche.tranche_id).unwrap();
            //    println!("{}",account);
            println!("reserve mint: {}",account.reserve_mint);
        }
    }
}
