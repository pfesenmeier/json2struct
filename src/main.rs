use application_arguments::ApplicationArguments;
use clap::Parser;
use serde_json::Value;
use std::error::Error;

mod application_arguments;
mod json_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = json_parser::Parser::new();
    let opt = ApplicationArguments::parse();

    let params: Value = serde_json::from_str(&opt.json)?;
    let public = &opt.public;
    if public == "true" {
        parser.set_pub(String::from("pub"))
    }

    let camel_case = &opt.camel_case;
    if camel_case != "false" {
        parser.set_camel(String::from("#[allow(non_snake_case)]"))
    }

    let derive: &str = &opt.derive;
    parser.set_derive(derive.to_string());
    let res = parser.parse(&params, &opt.struct_name);
    println!("{}", res);

    Ok(())
}
