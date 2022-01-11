mod cli_args;
mod json_parser;

use clap::Parser;
use cli_args::CliArgs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let CliArgs {
        derive,
        private,
        json,
        struct_name,
    } = CliArgs::parse();

    let mut parser = json_parser::Parser::new(private, derive);

    let res = parser.parse(&serde_json::from_str(&json)?, &struct_name);

    println!("{}", res);

    Ok(())
}
