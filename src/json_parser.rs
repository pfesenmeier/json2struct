use heck::{CamelCase, SnakeCase};
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
    pub fn new(private: bool, derive: String) -> Self {
        let public = if private { "" } else { "pub" }.to_string();

        Self {
            public,
            derive,
            ..Default::default()
        }
    }

    pub fn parse(&mut self, params: &Value, struct_name: &str) -> String {
        let serde_camel_case = r#"#[serde(rename_all = "camelCase")]"#;
        let struct_header = format!(
            "{}\n{}\n{} struct {} {}",
            self.derive, serde_camel_case, self.public, struct_name, "{"
        );

        let (fields, new_struct) = if params.is_object() {
            let (mut fields, new_struct) = self.is_object(params);
            fields.sort();
            (fields, new_struct)
        } else {
            Default::default()
        };

        format!(
            "{}\n{}\n{}\n{}",
            struct_header,
            fields.join("\n"),
            "}\n",
            new_struct
        )
    }

    fn is_object(&mut self, params: &Value) -> (Vec<String>, String) {
        let mut fields: Vec<String> = vec![];
        let cur_map = params.as_object().unwrap();
        let mut new_struct = String::new();

        for key_val in cur_map.iter() {
            let (key, val) = key_val;
            let (cur_type, ok, data2) = self.get_data_type(val, key);

            new_struct += &if val.is_object() && data2 {
                let camel_key = key.as_str().to_camel_case();
                self.is_ok(&cur_type, &camel_key, val, ok)
            } else if val.is_array() && !val.as_array().unwrap().is_empty() {
                let camel_key = key.as_str().to_camel_case();
                let cur_val = Parser::is_array(val);
                println!("{:?}", &camel_key);
                self.is_ok(&cur_type, &camel_key, cur_val, ok)
            } else {
                Default::default()
            };

            let snake_key = key.as_str().to_snake_case();
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
        params.as_array().unwrap().get(0).unwrap()
    }

    fn get_data_type(&mut self, params: &Value, key: &str) -> (String, bool, bool) {
        if params.is_object() {
            let (cur_key, ok) = self.key_exists(String::from(key), String::from(key));

            if serde_json::to_string(params).unwrap() == "{}" {
                return ("HashMap<String, Value>".to_string(), ok, false);
            }

            (cur_key.to_string(), ok, true)
        } else if params.is_string() {
            ("String".to_string(), false, true)
        } else if params.is_i64() {
            ("i64".to_string(), false, true)
        } else if params.is_boolean() {
            ("bool".to_string(), false, true)
        } else if params.is_array() {
            let first = params
                .as_array()
                .unwrap()
                .get(0)
                .unwrap_or(&serde_json::Value::Null);

            if first == &serde_json::Value::Null {
                let cur_type = format!("Vec<{}>", "Value");
                (cur_type, false, true)
            } else {
                let (cur0, ok, flag) = self.get_data_type(first, key);
                (format!("Vec<{}>", cur0), ok, flag)
            }
        } else if params.is_f64() {
            ("f64".to_string(), false, true)
        } else if params.is_u64() {
            ("u64".to_string(), false, true)
        } else {
            ("Value".to_string(), false, true)
        }
    }

    fn key_exists(&mut self, key: String, new_key: String) -> (String, bool) {
        self.index += 1;
        let cur_key = format!("{}{}", key, self.index);
        self.index = 0;

        if self.name.contains(&new_key) {
            let (new_key, ..) = self.key_exists(key, cur_key);
            (new_key, true)
        } else {
            self.name.push(new_key.clone());
            (new_key, false)
        }
    }
}
