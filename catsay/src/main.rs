extern crate structopt;
extern crate colored;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "I'm cat!")]
    // comments started with /// are documentation comments
    /// What does the cat say?
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// Makes cat appear dead
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let mut message = options.message;

    if message.to_lowercase() == "woof" {
        eprintln!("A cat can't bark!");
    }

    let eye = if options.dead { "x" } else { "o" };
    message = if options.dead { "X__x...".to_string() } else { message };

    println!("{}", message.yellow().underline().bold());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye.green().bold());
    println!("    =( I )=");
}
