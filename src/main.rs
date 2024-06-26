use std::path::Path;

use dartfile::parse;

mod cli;
mod dartfile;
mod dartgun;

fn main() {
    let test_path = Path::new("./dartgun.toml");
    let test_dotfile = parse(test_path);
    println!("{:?}", parse(test_path));
    match test_dotfile.validate() {
        Ok(_) => println!("Dotfile seems valid!"),
        Err(_) => println!("Dotfile is invalid!"),
    }
    cli::run_cli();
}
