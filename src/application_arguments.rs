use clap::Parser;

/// Json 2 Struct for rust.
#[derive(Parser)]
#[clap(author, version, about)]
pub struct ApplicationArguments {
    /// a json string, e.g. '{"test":"test"}'
    pub json: String,

    /// make fields public
    #[clap(short, long)]
    pub private: bool,

    /// derive options e.g. '#[derive(Debug)]'
    #[clap(default_value = "#[derive(Debug)]", short, long)]
    pub derive: String,

    /// struct name
    #[clap(default_value = "XXX", short, long)]
    pub struct_name:String,
}
