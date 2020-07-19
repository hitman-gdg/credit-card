const INVALID: &str = "INVALID";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env::args;
    if args().len() != 2 { std::process::exit(1); }
    let cc_number: u64 = args().nth(1).unwrap().parse()?;
    println!("{}", process_number(cc_number));
    Ok(())
}

fn process_number(card_num: u64) -> &'static str {
    let card_string = card_num.to_string();
    let arr: Vec<u8> = card_string.chars()
        .map(|a| a.to_digit(10).unwrap() as u8).collect();
    let length = arr.len() as u8;
    if length != 13 && length != 15 && length != 16 { return INVALID; }
    let out = check_type(&arr, length);
    if digit_check(length, &arr) {
        return out;
    }
    INVALID
}

fn check_type(arr: &[u8], length: u8) -> &'static str {
    let first = arr[0];
    let second = arr[1];
    match length as i32 {
        13 | 16 if first == 4 => "VISA",
        15 if first == 3 && second == 4 || second == 7 => "AMEX",
        16 if first == 5 && second > 0 && second < 6 => "MASTERCARD",
        _ => INVALID 
    }
}

fn digit_check(length: u8, arr: &[u8]) -> bool {
    let mut total = 0;
    for i in 0..length {
        let mut value = arr[i as usize];
        if (length - 1 - i) % 2 != 0 {
            value *= 2;
            if value > 9 { value -= 9; }
        }
        total += value;
    }
    total % 10 == 0
}