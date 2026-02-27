use std::env;

mod when;
mod bat;
mod state;

extern crate battery;
extern crate libc;

fn usage() {
    eprintln!("Usage: powernotifs <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("start    Start monitoring battery events");
}

fn main() -> battery::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "start" => state::start(),
        "warning" => when::warning(),
        _ => {
            usage();
            std::process::exit(1);
        }
    }
}
