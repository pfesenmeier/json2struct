mod json2struct;
extern crate serde_derive;
extern crate serde_json;
extern crate heck;
use serde_json::Value;
use json2struct::rust::{rust_parse, set_pub, set_derive};
use json2struct::ApplicationArguments;
use crate::json2struct::rust::set_camel;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = ApplicationArguments::parse();

    let params: Value = serde_json::from_str(&opt.json)?;
    let public = &opt.public;
    if public == "true" {
        set_pub(String::from("pub"))
    }

    let camel_case = &opt.camel_case;
    if camel_case != "false" {
        set_camel(String::from("#[allow(non_snake_case)]"))
    }

    let derive:&str = &opt.derive;
    set_derive(derive.to_string());
    let res = rust_parse(&params, &opt.struct_name);
    println!("{}", res);

    Ok(())
}

