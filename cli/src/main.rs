use structopt::StructOpt;

/// A program for adding two numbers together.
#[derive(Debug, StructOpt)]
struct Args {
    /// The first number.
    first: u32,
    /// The second number.
    second: u32,
}

fn main() {
    let Args { first, second } = Args::from_args();

    let sum = cdir_core::add(first, second);
    println!("{} + {} = {}", first, second, sum);
}
