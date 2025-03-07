pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let parts: Vec<&str> = num_str.trim_end_matches(')').split('(').collect();
    if parts.len() != 2 {
        return "错误的输入格式".to_string();
    }

    let num_str = parts[0];
    let from_base = parts[1].parse::<u32>().unwrap();

    if from_base < 2 || from_base > 16 || to_base < 2 || to_base > 16 {
        return "进制必须在2到16之间".to_string();
    }

    let mut decimal = 0u64;
    for c in num_str.chars() {
        let digit = match char_to_digit(c, from_base) {
            Some(d) => d,
            None => return format!("无效的字符 {} 在 {} 进制中", c, from_base),
        };
        decimal = decimal * from_base as u64 + digit as u64;
    }

    if decimal == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut temp = decimal;
    while temp > 0 {
        let remainder = temp % to_base as u64;
        result.insert(0, digit_to_char(remainder as u32));
        temp /= to_base as u64;
    }

    result
}

fn char_to_digit(c: char, base: u32) -> Option<u32> {
    let digit = match c {
        '0'..='9' => c as u32 - '0' as u32,
        'a'..='f' => c as u32 - 'a' as u32 + 10,
        'A'..='F' => c as u32 - 'A' as u32 + 10,
        _ => return None,
    };

    if digit < base {
        Some(digit)
    } else {
        None
    }
}

fn digit_to_char(digit: u32) -> char {
    if digit < 10 {
        (digit + '0' as u32) as u8 as char
    } else {
        (digit - 10 + 'a' as u32) as u8 as char
    }
}
