use application_arguments::ApplicationArguments;
use clap::Parser;
use serde_json::Value;
use std::error::Error;

mod application_arguments;
mod json_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = ApplicationArguments::parse();

    let params: Value = serde_json::from_str(&opt.json)?;

    let derive: &str = &opt.derive;

    let mut parser = json_parser::Parser::new(opt.private, derive.to_string()); 
    let res = parser.parse(&params, &opt.struct_name);

    println!("{}", res);

    Ok(())
}
