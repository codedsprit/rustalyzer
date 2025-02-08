mod scanner;
mod parser;
mod fingerprints;

fn main() {
    let url = "https://example.com"; // Change this to scan a real site

    match scanner::scan_website(url) {
        Ok(html) => {
            let tech = parser::analyze_technologies(&html);
            println!("Detected Technologies: {:?}", tech);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
