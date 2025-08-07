mod command;
mod commands;

use command::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match Command::parse_args(&args) {
        Ok(command) => command.execute(),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

