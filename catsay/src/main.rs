extern crate structopt;

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

    let eye = if options.dead { "x" } else { "o" };
    message = if options.dead { "Ooops...X__x".to_string() } else { message };

    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( {eye} {eye} )", eye=eye);
    println!("    =( I )=");
}
