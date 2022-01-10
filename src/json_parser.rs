use heck::{SnakeCase, CamelCase};
use serde_json::Value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[derive(Default)]
pub struct Parser {
    public: String,
    name: Vec<String>,
    index: i8,
    derive: String,
}

impl Parser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_pub(&mut self, public: String) {
        self.public = public
    }
    pub fn set_derive(&mut self, derive: String) {
        self.derive = derive
    }

    pub fn parse(&mut self, params: &Value, struct_name: &str) -> String {
        let serde_camel_case = r#"#[serde(rename_all = "camelCase")]"#;
        let struct_header = format!(
            "{}\n{}\n{} struct {} {}",
            self.derive, serde_camel_case, self.public, struct_name, "{"
        );
        let mut fields: Vec<String> = vec![];
        let mut new_struct = String::new();
        if params.is_object() {
            let cur_res = self.is_object(params);
            fields = cur_res.0;
            fields.sort();
            new_struct = cur_res.1;
        }
        let res = format!(
            "{}\n{}\n{}\n{}",
            struct_header,
            fields.join("\n"),
            "}\n",
            new_struct
        );
        res
    }

    fn is_object(&mut self, params: &Value) -> (Vec<String>, String) {
        let mut fields: Vec<String> = vec![];
        let cur_map = params.as_object().unwrap();
        let mut new_struct = String::new();
        for key_val in cur_map.iter() {
            let key = key_val.0;
            let val = key_val.1;
            let data = self.get_data_type(val, key);
            let cur_type = data.0;
            let ok = data.1;
            let camel_key = key.as_str().to_camel_case();
            let snake_key = key.as_str().to_snake_case();
            let mut cur_struct = String::new();
            if val.is_object() {
                if data.2 {
                    cur_struct = self.is_ok(&cur_type, &camel_key, val, ok)
                }
            } else if val.is_array() {
                let cur = val.as_array().unwrap();
                if !cur.is_empty() {
                    let cur_val = Parser::is_array(val);
                    println!("{:?}", &camel_key);
                    cur_struct = self.is_ok(&cur_type, &camel_key, cur_val, ok)
                }
                println!("{:?}", &cur_struct);
            }
            new_struct += cur_struct.as_str();
            let field = format!("    {} {}: {},", self.public, snake_key, cur_type);
            fields.push(field);
        }
        (fields, new_struct)
    }

    fn is_ok(&mut self, cur_type: &str, came_key: &str, val: &Value, ok: bool) -> String {
        if ok {
            let next_key = cur_type.to_snake_case();
            self.parse(val, &next_key)
        } else {
            self.parse(val, came_key)
        }
    }

    fn is_array(params: &Value) -> &Value {
        let cur = params.as_array().unwrap();
        let val = cur.get(0).unwrap();
        val
    }

    fn get_data_type(&mut self, params: &Value, key: &str) -> (String, bool, bool) {
        let mut ok = false;
        let mut flag = true;
        if params.is_object() {
            // 1
            let mut cur_key = key.to_string();
            let res = self.key_exists(cur_key.clone(), cur_key.clone());
            cur_key = res.0;
            ok = res.1;
            let mut cur_type = cur_key.as_str().to_snake_case();
            // 2
            let flag_str = serde_json::to_string(params).unwrap();
            if flag_str == "{}" {
                cur_type = String::from("HashMap<String, Value>");
                flag = false
            }
            (cur_type, ok, flag)
        } else if params.is_string() {
            let cur_type = String::from("String");
            (cur_type, ok, flag)
        } else if params.is_i64() {
            let cur_type = String::from("i64");
            (cur_type, ok, flag)
        } else if params.is_boolean() {
            let cur_type = String::from("bool");
            (cur_type, ok, flag)
        } else if params.is_array() {
            let values = params.as_array().unwrap();
            let first = values.get(0).unwrap_or(&serde_json::Value::Null);
            if first == &serde_json::Value::Null {
                let cur_type = format!("Vec<{}>", "Value");
                return (cur_type, ok, flag);
            }
            let cur = self.get_data_type(first, key);
            ok = cur.1;
            flag = cur.2;
            let cur_type = format!("Vec<{}>", cur.0);
            (cur_type, ok, flag)
        } else if params.is_f64() {
            let cur_type = String::from("f64");
            (cur_type, ok, flag)
        } else if params.is_u64() {
            let cur_type = String::from("u64");
            (cur_type, ok, flag)
        } else {
            let cur_type = String::from("Value");
            (cur_type, ok, flag)
        }
    }

    fn key_exists(&mut self, key: String, mut new_key: String) -> (String, bool) {
        let mut ok = false;
        self.index += 1;
        let cur_key = format!("{}{}", key, self.index);
        if self.name.contains(&new_key) {
            ok = true;
            let cur_res = self.key_exists(key, cur_key);
            new_key = cur_res.0;
        } else {
            self.name.push(new_key.clone())
        }
        self.index = 0;
        (new_key, ok)
    }
}
