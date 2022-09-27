use {
    crate::args::core_args,
    core_args:: {
        CoreCommand,
        CoreSubcommand,
    },
    anchor_client::{
        Program,
        solana_sdk::{
            pubkey:: Pubkey,
        },
        anchor_lang::prelude::*
    },
    vyper_core:: {
        state::TrancheConfig
    }
    
};






pub fn handle_core_command(core_command: CoreCommand, program: &Program) {
    let command = core_command.command;
    match command {
        CoreSubcommand::Fetch(fetch_tranche) => {
           let account: Result<TrancheConfig, anchor_client::ClientError> = program.account(fetch_tranche.tranche_id).unwrap();
           println!("{:?}",account);
        }
    }
}
