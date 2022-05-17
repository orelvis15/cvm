use std::collections::HashMap;
use std::ops::Add;
use strfmt::strfmt;

pub fn url_build(args: Vec<&str>, last_slash: bool) -> String {
    let mut patter = String::new();
    let mut args_map: HashMap<String, String> = HashMap::new();

    for (i, arg) in args.iter().enumerate() {
        &args_map.insert(arg.clone().to_string(), arg.clone().to_string());

        &patter.push_str("{");
        &patter.push_str(&arg);
        &patter.push_str("}");

        if i != args.len() - 1 || last_slash{
            &patter.push_str("/");
        }
    }
    strfmt(&patter.as_str(), &args_map).unwrap()
}