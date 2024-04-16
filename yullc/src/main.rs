use clap::Parser;

mod args;
mod version;
pub use version::*;

fn main() {
    let args = args::Args::parse();

    args.execute().unwrap();
}
