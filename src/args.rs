use clap::Parser;
/// The args to be pased to the CLI for determining image properties.
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub width: i32,
    #[arg(long)]
    pub height: i32,
}
