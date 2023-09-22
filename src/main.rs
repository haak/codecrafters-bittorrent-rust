use serde_json;
use std::{env, fs};

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

        return (map, rest);
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

#[derive(Debug)]
struct Torrent {
    announce: String,
    length: u64,
}

fn decode_torrent(encoded_torrent: &str) -> Torrent {
    let decoded = decode_bencoded_value(&encoded_torrent);
    let obj = decoded.0.as_object().unwrap();
    return Torrent {
        announce: obj.get("announce").unwrap().as_str().unwrap().to_string(),
        length: obj
            .get("info")
            .unwrap()
            .get("length")
            .unwrap()
            .as_u64()
            .unwrap(),
    };
}

fn decode_command(encoded_value: &str) -> serde_json::Value {
    let decoded_value = decode_bencoded_value(encoded_value).0;
    return decoded_value;
}

fn info_command(file_path: &str) -> Torrent {
    let file = fs::read(file_path).unwrap();
    let content = unsafe { String::from_utf8_unchecked(file) };
    let torrent = decode_torrent(&content);
    return torrent;
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            let encoded_value = &args[2];
            let decoded_value = decode_command(encoded_value);
            println!("{}", decoded_value);
        }
        "info" => {
            let file_path = &args[2];
            let torrent = info_command(file_path);
            println!("Tracker URL: {}", torrent.announce);
            println!("Length: {}", torrent.length);
        }
        _ => println!("unknown command: {}", command),
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
