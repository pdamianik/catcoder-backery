use clap::{Parser, Subcommand};

mod level1;
mod level2;
mod level3;

/// CCC Catcoder Backery
#[derive(Parser)]
struct Arguments {
    /// The level to run
    #[command(subcommand)]
    level: Level,
}

#[derive(Subcommand, Copy, Clone)]
enum Level {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

fn main() {
    simple_logger::init_with_env().unwrap();

    let args = Arguments::parse();

    match args.level {
        Level::Level1 => level1::main(),
        Level::Level2 => level2::main(),
        Level::Level3 => level3::main(),
        _ => unimplemented!(),
    }
}
