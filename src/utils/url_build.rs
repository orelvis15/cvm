#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use strfmt::strfmt;

pub fn url_build(args: Vec<&str>, last_slash: bool) -> String {
    let mut patter = String::new();
    let mut args_map: HashMap<String, String> = HashMap::new();

    for (i, arg) in args.iter().enumerate() {
        let _ = &args_map.insert(arg.clone().to_string(), arg.clone().to_string());

        let _ = &patter.push_str("{");
        let _ = &patter.push_str(&arg);
        let _ = &patter.push_str("}");

        if i != args.len() - 1 || last_slash{
            let _ = &patter.push_str("/");
        }
    }
    strfmt(&patter.as_str(), &args_map).unwrap()
}