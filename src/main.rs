use std::env;
use std::process;
use std::str::FromStr;

extern crate cargo_version;

use cargo_version::{create_version, BumpLevel, CargoVersionError, SemVerError};

fn main() {
    let mut arguments = env::args();

    match arguments.nth(1) {
        None => {
            println!("Usage: cargo-version [<newversion> | major | minor | patch]");
            process::exit(1);
        },
        Some(level_str) => {
            match BumpLevel::from_str(level_str.as_str()) {
                Err(_) => {
                    println!("Invalid bump level \"{}\" specified", level_str);
                    process::exit(1);
                }
                Ok(level) => {
                    match create_version(level) {
                        Ok(()) => (),
                        Err(CargoVersionError::GitNotClean) => {
                            println!("Git working directory not clean");
                            process::exit(1);
                        },
                        Err(CargoVersionError::NoVersionFound) => {
                            println!("Failed to find version in Cargo.toml");
                            process::exit(1);
                        },
                        Err(CargoVersionError::SemVerError(SemVerError::ParseError(version))) => {
                            println!("Failed to parse version \"{}\"", version);
                            process::exit(1);
                        },
                        Err(CargoVersionError::IoError(err)) => {
                            println!("{}", err);
                            process::exit(1);
                        },
                    }
                }
            }
        }
    }
}
