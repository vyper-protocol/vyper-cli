mod args;
use args::VyperCliArgs;
use clap::Parser;


fn main() {
    let args = VyperCliArgs::parse();
    println!("{:?}", args );
}
