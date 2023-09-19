use serde_json;
use std::env;

// run with cargo run decode 5:hello

// Available if you need it!
// use serde_bencode

// #[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    // println!("encoded_value: {}", encoded_value);
    if encoded_value.chars().next().unwrap().is_digit(10) {
        let (string, remaining) = handle_string(encoded_value);

        return (serde_json::Value::String(string.to_string()), remaining);
    } else if encoded_value.chars().next().unwrap() == 'i' {
        let (number, remaining) = handle_number(encoded_value);
        return (serde_json::Value::Number(number.into()), remaining);
    } else if encoded_value.chars().next().unwrap() == 'l' {
        let (values, remaining) = handle_list(encoded_value);

        return (serde_json::Value::Array(values), remaining);
    } else if encoded_value.chars().next().unwrap() == 'd' {
        let (map, rest) = handle_dictionary(encoded_value);

        // let mut map = serde_json::Map::new();
        // let mut rest = &encoded_value[1..];
        // // println!("rest: {}", rest);
        // while rest.chars().next().unwrap() != 'e' {
        //     let (key, remaining) = decode_bencoded_value(rest);
        //     let (value, remaining) = decode_bencoded_value(remaining);
        //     // println!("key: {}", key);
        //     // println!("value: {}", value);
        //     // let mut map = map.clone();
        //     map.insert(key.to_string(), value);
        //     rest = remaining;
        // }
        return (map, rest);
        // println!("encoded_value: {}", encoded_value);
        // return (serde_json::Value::Null, encoded_value);
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn handle_string(encoded_value: &str) -> (&str, &str) {
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    let remaining = &encoded_value[colon_index + 1 + number as usize..];
    return (string, remaining);
}

fn handle_number(encoded_value: &str) -> (i64, &str) {
    let end_index = encoded_value.find('e').unwrap();
    let number_string = &encoded_value[1..end_index];
    let number = number_string.parse::<i64>().unwrap();
    let remaining = &encoded_value[end_index + 1..];
    return (number, remaining);
}

fn handle_list(encoded_value: &str) -> (Vec<serde_json::Value>, &str) {
    let mut values = Vec::new();
    let mut rest = &encoded_value[1..];
    while rest.chars().next().unwrap() != 'e' {
        let (value, remaining) = decode_bencoded_value(rest);
        values.push(value);
        rest = remaining;
    }

    return (values, rest);
}

fn handle_dictionary(encoded_value: &str) -> (serde_json::Value, &str) {
    let mut map = serde_json::Map::new();
    let mut rest = &encoded_value[1..];
    while rest.chars().next().unwrap() != 'e' {
        let (key, remaining) = decode_bencoded_value(rest);
        let (value, remaining) = decode_bencoded_value(remaining);
        map.insert(key.as_str().unwrap().to_string(), value);
        rest = remaining;
    }
    return (serde_json::Value::Object(map), rest);
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value).0;
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_decode_bencode_value() {
        assert_eq!(decode_bencoded_value("5:hello").0, "hello");
        assert_eq!(decode_bencoded_value("1:s").0, "s");
        assert_eq!(decode_bencoded_value("i52e").0, 52);
        assert_eq!(decode_bencoded_value("i3e").0, 3);
        assert_eq!(
            decode_bencoded_value("l5:helloi52ee").0,
            json!(["hello", 52])
        );
        assert_eq!(
            decode_bencoded_value("d3:foo3:bar5:helloi52ee").0,
            json!({"foo": "bar", "hello": 52})
        );

        assert_eq!(decode_bencoded_value("de").0, json!({}));
    }
}
