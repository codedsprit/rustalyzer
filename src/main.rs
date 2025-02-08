use clap::{Arg, Command};
mod scanner;
mod parser;
mod fingerprints;

fn main() {
    // clap setup
    let matches = Command::new("Rustalyzer")
        .version("1.0")
        .author("Roshan Tiwaree <roshan0x01@gmail.com>")
        .about("A Wappalyzer-like tool written in Rust but in cli")
        .arg(
            Arg::new("url")
                .short('u')                 
                .long("url")                 
                .help("The URL of the website to analyze")  
                .required(true)
                .num_args(1),                
        )
        .get_matches();

    // accept the argument
    let url = matches.get_one::<String>("url").unwrap(); 

    // check for the website
    match scanner::scan_website(url) {
        Ok(html) => {
            let tech = parser::analyze_technologies(&html);
            if tech.is_empty() {
                println!("No technologies detected.");
            } else {
                println!("Detected Technologies: {:?}", tech);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
