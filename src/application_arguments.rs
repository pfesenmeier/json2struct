use clap::Parser;

/// Json 2 Struct for rust.
#[derive(Parser)]
#[clap(author, version, about)]
pub struct ApplicationArguments {
    /// a json string, e.g. '{"test":"test"}'
    pub json: String,

    /// make fields public
    #[clap(default_value = "true", short, long)]
    pub public: String,

    /// derive options e.g. '#[derive(Debug)]'
    #[clap(default_value = "#[derive(Debug)]", short, long)]
    pub derive: String,

    /// camelCase struct fields (???) 
    #[clap(default_value = "false", short, long)]
    pub camel_case: String,

    /// struct name
    #[clap(default_value = "XXX", short, long)]
    pub struct_name:String,
}
