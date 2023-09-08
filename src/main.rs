use serde_json;
use std::env;

// run with cargo run decode 5:hello

// Available if you need it!
// use serde_bencode

// #[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    // println!("encoded_value: {}", encoded_value);
    // i52e
    if encoded_value.chars().next().unwrap().is_digit(10) {
        // Example: "5:hello" -> "5"
        let colon_index = encoded_value.find(':').unwrap();
        // println!("colon_index: {}", colon_index);

        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
    } else if encoded_value.chars().next().unwrap() == 'i' {
        // Example: "i52e" -> "52"
        // extract number and check if there is an e after
        let end_index = encoded_value.find('e').unwrap();
        let number_string = &encoded_value[1..end_index];
        let number = number_string.parse::<i64>().unwrap();
        return serde_json::Value::Number(serde_json::Number::from(number));
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
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
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

#[cfg(test)]
mod tests {
    use crate::decode_bencoded_value;

    #[test]
    fn test_decode_bencode_value() {
        assert_eq!(decode_bencoded_value("5:hello"), "hello");
        assert_eq!(decode_bencoded_value("1:s"), "s");
        assert_eq!(decode_bencoded_value("i52e"), 52);
        assert_eq!(decode_bencoded_value("i3e"), 3);
    }
}
