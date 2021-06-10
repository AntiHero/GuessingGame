extern crate colored;
extern crate structopt;

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
    #[structopt(short = "f", long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let mut message = options.message;

    if message.to_lowercase() == "woof" {
        eprintln!("A cat can't bark!");
    }

    let eye = if options.dead { "x" } else { "o" };
    message = if options.dead {
        "X__x...".to_string()
    } else {
        message
    };

    match &options.catfile {
        Some(path) => {
            let cat_template =
                std::fs::read_to_string(path).expect(&format!("could not read file! {:?}", path));
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", message.yellow().underline().bold());
            println!("{}", &cat_picture);
        }
        None => {
            println!("{}", message.yellow().underline().bold());
            println!(" \\");
            println!("  \\");
            println!("     /\\_/\\");
            println!("    ( {eye} {eye} )", eye = eye.green().bold());
            println!("    =( I )=");
        }
    }
}
