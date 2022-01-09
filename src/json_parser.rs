use heck::CamelCase;
use serde_json::Value;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

static mut PUB: String = String::new();
static mut STRUCT_NAME: Vec<String> = vec![];
static mut INDEX: i8 = 0;
static mut DERIVE: String = String::new();
static mut CAMEL: String = String::new();

pub fn set_pub(public: String) {
    unsafe {
        PUB = public;
    }
}

pub fn set_derive(derive: String) {
    unsafe {
        DERIVE = derive;
    }
}

pub fn set_camel(camel: String) {
    unsafe { CAMEL = camel }
}

pub fn rust_parse(params: &Value, struct_name: &str) -> String {
    unsafe {
        let struct_header = format!(
            "{}\n{}\n{} struct {} {}",
            CAMEL, DERIVE, PUB, struct_name, "{"
        );
        let mut fields: Vec<String> = vec![];
        let mut new_struct = String::new();
        if params.is_object() {
            let cur_res = is_object(params);
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
}

/// 对map类型的值进行处理
fn is_object(params: &Value) -> (Vec<String>, String) {
    let mut fields: Vec<String> = vec![];
    let cur_map = params.as_object().unwrap();
    let mut new_struct = String::new();
    for key_val in cur_map.iter() {
        let key = key_val.0;
        let val = key_val.1;
        let data = get_data_type(val, key);
        let cur_type = data.0;
        let ok = data.1;
        let came_key = key.as_str().to_camel_case();
        let snake_key = key;
        let mut cur_struct = String::new();
        if val.is_object() {
            if data.2 {
                cur_struct = is_ok(&cur_type, &came_key, val, ok)
            }
        } else if val.is_array() {
            let cur = val.as_array().unwrap();
            if !cur.is_empty() {
                let cur_val = is_array(val);
                println!("{:?}", &came_key);
                cur_struct = is_ok(&cur_type, &came_key, cur_val, ok)
            }
            println!("{:?}", &cur_struct);
        }
        new_struct += cur_struct.as_str();
        unsafe {
            let field = format!("    {} {}: {},", PUB, snake_key, cur_type);
            fields.push(field);
        }
    }
    (fields, new_struct)
}

fn is_ok(cur_type: &str, came_key: &str, val: &Value, ok: bool) -> String {
    if ok {
        let next_key = cur_type.to_camel_case();
        rust_parse(val, &next_key)
    } else {
        rust_parse(val, came_key)
    }
}

/// 对列表类型的数据进行处理
fn is_array(params: &Value) -> &Value {
    let cur = params.as_array().unwrap();
    let val = cur.get(0).unwrap();
    val
}

/// 获取数据类型
fn get_data_type(params: &Value, key: &str) -> (String, bool, bool) {
    let mut ok = false;
    let mut flag = true;
    if params.is_object() {
        let mut cur_key = key.to_string();
        let res = key_exists(cur_key.clone(), cur_key.clone());
        cur_key = res.0;
        ok = res.1;
        let mut cur_type = cur_key.as_str().to_camel_case();
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
        let cur = get_data_type(first, key);
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

fn key_exists(key: String, mut new_key: String) -> (String, bool) {
    let mut ok = false;
    unsafe {
        INDEX += 1;
        let cur_key = format!("{}{}", key, INDEX);
        if STRUCT_NAME.contains(&new_key) {
            ok = true;
            let cur_res = key_exists(key, cur_key);
            new_key = cur_res.0;
        } else {
            STRUCT_NAME.push(new_key.clone())
        }
        INDEX = 0;
    }
    (new_key, ok)
}
