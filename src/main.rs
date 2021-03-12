extern crate structopt;

use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    message: String // [1]
}
fn main() {
    let options = Options::from_args(); // [2]
    let message = options.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("       /\\_/\\");
    println!("      ( o o )");
    println!("      =( I )=");
}
