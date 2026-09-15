use std::{ env, fs::create_dir };
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    mo_path: String,

    /// Replace ship names
    #[arg(long)]
    ships: Option<String>
}

/// Creates 'out' dir if it doesn't already exist.
fn create_out_dir() {
    let mut exe_dir = env::current_exe().expect("Failed to get current executable path");
    exe_dir.pop();

    let out_dir = exe_dir.as_path().join("out");

    match out_dir.try_exists() {
        Ok(boolean) => {
            if !boolean {
                create_dir(out_dir).expect("Error creating 'out' directory");
            }
        },
        Err(e) => {
            eprintln!("Error while checking for 'out' directory: {:?}", e.raw_os_error());
        }
    }
}

fn main() {
    let args = Args::parse();
    create_out_dir();

    let ships_arg = args.ships.unwrap_or("Nothing".to_string());
    eprintln!("{}", ships_arg);
}
