pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 检查base
    if to_base < 2 || to_base > 16 {
        return "Invalid base".to_string();
    }

    // 检查输入字符串是否为空
    if num_str.is_empty() {
        return "Invalid input".to_string();
    }

    let digits = "0123456789abcdef";

    let mut result = String::new();

    // 检查是否有括号
    let (num_str, from_base) = if num_str.contains('(') {
        let (num_str, base_str) = num_str.split_at(num_str.find('(').unwrap());
        let base_str = &base_str[1..base_str.len() - 1];
        (num_str, base_str.parse::<u32>().unwrap())
    } else {
        (num_str, 10)
    };

    // 转化为十进制
    let mut num = 0;
    for (i, c) in num_str.chars().rev().enumerate() {
        let digit = digits.find(c).unwrap() as u32;
        num += digit * from_base.pow(i as u32);
    }

    // 转换到目标进制
    while num > 0 {
        let digit = num % to_base;
        result.insert(0, digits.chars().nth(digit as usize).unwrap());
        num /= to_base;
    }

    result
    
}
