use std::{error::Error, path::PathBuf};
use clap::{arg, command, value_parser, ArgAction, Command};

///
/// Synchronized, optional, Update platform.
/// 
/// # Purpose
/// The official toolchain updater for the Caldo Programming Language
/// 
/// ## Environmental Variables
/// `SOUP_HOME`: 
/// 
/// # Returns
/// `Result<(), Box<dyn Error>>`: Will return nothing or an error.
/// 
fn main() -> Result<(), Box<dyn Error>> {
    let soup_home = std::env::var("SOUP_HOME");
    match soup_home {
        Ok(val) => println!("SOUP_HOME: {}", val),
        Err(_e) => println!("SOUP_HOME is not defined"),
    }
    let matches = command!()
        .arg(arg!(-V --version ... " Print version information")
            .required(false))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    Ok(())
}
