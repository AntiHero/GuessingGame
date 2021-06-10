extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "I'm cat!")]
    // comments started with /// are documentation comments 
    /// What does the cat say?
    message: String
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");   
    println!("    =( I )=");
}
