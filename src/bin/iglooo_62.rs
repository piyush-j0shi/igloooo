fn fields(line: &str) -> Vec<(&str, &str)> {
    let mut somevec = Vec::new();

    for intermediate in line.split_whitespace() {
        let something = intermediate.split_once("=");

        if let Some(v) = something {
            somevec.push(v);
        }
    }
    somevec
}

// we need lifetimes here because in input we have two diffrent strings which are borrowed and in
// return we are returning single borrowed string so rust needs to know which one of them would be
// alive (this is one way to say) or which borrowe string should be returned that's why we need
// lifetimes here in other cases there are single input so it uses same lifetime.

fn get<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    line.split_whitespace()
        .filter_map(|field| field.split_once('='))
        .find(|(k, _)| *k == key)
        .map(|(_, value)| value)
}

fn tags(line: &str) -> Vec<&str> {
    get(line, "tags").unwrap().split(",").collect()
}

fn strip_comment(line: &str) -> &str {
    let returnvalue = line.split_once("#");
    returnvalue.unwrap().0.trim()
}

fn version(version: &str) -> &str {
    let returnvalue = version.strip_prefix("v");
    returnvalue.unwrap()
}

fn summary(line: &str) -> String {
    let first_intermediate_value = get(line, "host");
    let second_intermediate_value = get(line, "port");

    let summary_result = format!(
        "{}:{}",
        first_intermediate_value.unwrap(),
        second_intermediate_value.unwrap()
    );
    summary_result
}

fn main() {
    let someline = "host=db-01  port=5432   user=admin  tags=prod,primary,eu-west  cache=redis  ttl=300   # tuning pass, revisit";

    println!("{:?}", fields(someline));
    println!("{:?}", get(someline, "redis"));
    println!("{:?}", tags(someline));
    println!("{}", strip_comment(someline));
    println!("{}", version("v2.0.1"));
    println!("{}", summary(someline));
}
