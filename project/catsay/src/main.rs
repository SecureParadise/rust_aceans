use std::fmt::format;
use clap::Parser;
use colored::Colorize;
#[derive(Parser)]
struct Options{
    #[clap (default_value = "Meow ! ")]
    /// What does the cat say?
    message:String,
    #[clap (short = 'd',long ="dead")]
    /// Make the cat appear dead
    dead:bool,
    #[clap (short = 'f',long ="file")]
    /// Load the cat pictures from the specified file
    catfile: Option<std::path::PathBuf>,
}
fn main() {
    // let message = std::env::args()
    //     .nth(1)
    //     .expect("Missing the message. Usaage: catsay <message>");
    let options = Options::parse();
    match &options.catfile{
        Some(path) => {
            let cat_tempalte = std::fs::read_to_string(path).expect(
                &format!("could not read file {:?}",path)
            );
            let eye = format!("{}",eye.red().bold());
            let cat_picture = cat_tempalte.replace("{eye}", &eye);
            println!("{}",&cat_picture);
        },
        None =>{
        
            let message = options.message;
    let eye = if options.dead { "x" } else { "o" };
    if message.to_lowercase() == "woof"{
        eprintln!("A cat shouldn't bark like a dog .")
    }
    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("      /\\_/\\");
    println!("     ( {eye} {eye} ) ",eye = eye.red().bold());
    println!("    = ( I ) =");
        }
    }


    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };
    if message.to_lowercase() == "woof"{
        eprintln!("A cat shouldn't bark like a dog .")
    }
    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("      /\\_/\\");
    println!("     ( {eye} {eye} ) ",eye = eye.red().bold());
    println!("    = ( I ) =");
}
