pub mod rust;

use structopt::StructOpt;

/// Json 2 Struct for rust.
#[derive(Debug, StructOpt)]
pub struct ApplicationArguments {
    /// Input a json string, example: json2struct rust '{"test":"test"}'
    pub json: String,

    /// 是否添加 pub, example: json2struct rust '{"test":"test"}' -p false
    #[structopt(default_value = "true", short)]
    pub public: String,

    /// 添加 derive, example: json2struct rust '{"test":"test"}' -d '#[derive(Debug)]'
    #[structopt(default_value = "#[derive(Debug)]", short)]
    pub derive: String,

    /// 是否允许字段为驼峰 camel, example: json2struct rust '{"test":"test"}' -c true
    #[structopt(default_value = "false", short)]
    pub camel: String,

    /// 指定结构体名字, example: json2struct rust '{"test":"test"}' -s TTTT
    #[structopt(default_value = "XXX", short)]
    pub struct_name:String,
}
