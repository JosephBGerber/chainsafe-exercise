use std::error::Error;
use chainsafe_exercise::{get_icon, save_icon};
use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use web3::types::{Address, H160};



fn is_address_valid(address: &str) -> Result<(), String> {
    let re = Regex::new("(0x)?[A-Fa-f0-9]{20}").unwrap();
    if re.is_match(address) {
        Ok(())
    } else {
        Err("Invalid format for ethereum address.".to_string())
    }
}

fn parse_address(mut address: &str) -> Address {
    if address.starts_with("0x") {
        address = &address[2..]
    }

    let mut user_address: [u8; 20] = [0; 20];
    hex::decode_to_slice(address, &mut user_address).unwrap();

    H160::from(user_address)
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("iconic")
        .subcommand(
            App::new("set")
                .arg(
                    Arg::new("address")
                        .index(1)
                        .takes_value(true)
                        .value_name("ADDRESS")
                        .help("Your ethereum address encoded as hexadecimal.")
                        .required(true)
                        .validator(is_address_valid),
                )
                .arg(
                    Arg::new("filename")
                        .index(2)
                        .takes_value(true)
                        .value_name("FILENAME")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("get")
                .arg(
                    Arg::new("address")
                        .index(1)
                        .takes_value(true)
                        .value_name("ADDRESS")
                        .help("Your ethereum address encoded as hexadecimal.")
                        .required(true)
                        .validator(is_address_valid),
                )
                .arg(
                    Arg::new("target")
                        .index(2)
                        .takes_value(true)
                        .help("The ethereum address of the user you want to search for. If not provided, then your address will be used.")
                        .value_name("TARGET ADDRESS")
                        .required(false)
                        .validator(is_address_valid),
                ),
        )
        .get_matches();

    if let Some(set) = matches.subcommand_matches("set") {
        let address = set.value_of("address").unwrap();
        let address = parse_address(address);
        let filename = set.value_of("filename").unwrap();
        let filename = PathBuf::from(filename);
        let result = save_icon(address, filename).await;
        match result {
            Ok(()) => {
                println!("Icon was successfully saved.")
            }
            Err(err) => {
                println!("An error occurred while saving your icon - {:#?}", err)
            }
        }
    } else if let Some(get) = matches.subcommand_matches("get") {
        let address = get.value_of("address").unwrap();
        let address = parse_address(address);
        let target = get.value_of("target");

        let target = match target {
            Some(target) => parse_address(target),
            None => address,
        };

        let result = get_icon(target).await;
        match result {
            Ok(content) => {
                let mut file = File::create("icon").expect("Failed to open target for icon.");
                file.write(&content)
                    .expect("Failed to write contents to icon file.");
                file.flush().expect("Failed to flush icon to file.");
                open::that("icon").expect("Failed to open icon using default program.");
            }
            Err(err) => {
                println!(
                    "An error occurred while fetching target's icon - {:#?}",
                    err
                )
            }
        }
    } else {
        println!("No subcommand specified.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new()?;
    runtime.block_on(async_main())?;

    Ok(())
}
