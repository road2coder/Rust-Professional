use std::vec;

fn char_to_int(c: char) -> Result<u32, String> {
    let code = c as u32;
    match c {
        '0'..='9' => Ok(code - 48),
        'a'..='z' => Ok(code - 87),
        'A'..='Z' => Ok(code - 55),
        _ => Err(format!("Invalid character: {}", c)),
    }
}

fn parse_origin_str(num_str: &str) -> Result<u32, String> {
    let mut nums = Vec::new();
    let mut chars = num_str.chars();
    let mut base = String::new();
    for char in chars.by_ref() {
        if char == '(' {
            break;
        } else {
            nums.push(char_to_int(char)?);
        }
    }
    for char in chars {
        if char.is_ascii_digit() {
            base.push(char);
        } else if char == ')' {
            break;
        } else {
            return Err("invalid num".to_string());
        }
    }
    let base = base
        .parse::<u32>()
        .map_err(|_| "invalid base".to_string())?;
    // TODO: 处理类似 8 进制中出现8的情况
    Ok(nums
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, num)| acc + num * base.pow(i as u32)))
}

fn int_to_char(int: u32) -> char {
    let to_add = if int < 10 { 48 } else { 87 };
    char::from_u32(int + to_add).unwrap()
}

fn to_string(mut num: u32, mut base: u32) -> String {
    let mut arr = vec![1];
    while base <= num {
        arr.push(base);
        base = base * base;
    }
    arr.into_iter().rev().fold(String::new(), |mut acc, item| {
        let div = num / item;
        num -= div * item;
        acc.push(int_to_char(div));
        acc
    })
}

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let num = parse_origin_str(num_str).unwrap();
    to_string(num, to_base)
}
