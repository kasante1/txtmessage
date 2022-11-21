extern crate colored;
use colored::*;

use structopt::StructOpt;


#[derive(StructOpt)]
    
struct Options{
    message: String,
    
    #[structopt(short = "d", long = "dead")]
    dead: bool,
}


fn main() {


    let options = Options::from_args();

    let message = options.message;

    let eye = if options.dead{"X"} else{"O"};

    // let message = std::env::args().nth(1)
    // .expect("missing the message. Usage: new message <message>");

    if message.to_lowercase() == "woof"{
        eprintln!(" a man shouldn't bark like a dog")
    }

    println!("\n");
    print!("< {} >", message.bright_yellow().underline().on_purple());
    println!("\n");
    println!("   _______");
    println!("     \\");
    println!("      \\");
    println!("       _______");
    println!("      /_______\\ ");
    println!("      |       |");
    println!("    (| {eye}.{eye} |)");
    println!("      \\  __  /");
    println!("       \\____/ ");
}
