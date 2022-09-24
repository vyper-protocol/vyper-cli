mod args;
mod ops;

use args::VyperCliArgs;
use clap::Parser;
use args::Vyper;
use ops::core_ops::handle_core_command;

fn main() {
    let args = VyperCliArgs::parse();
    match args.vyper {
        Vyper::Core(core) => {
            handle_core_command(core)
        }
    }
}
