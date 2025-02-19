pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0; // 如果人数超过365，必定有两个人生日相同
    }

    let mut probability_no_match = 1.0;
    for i in 0..n {
        probability_no_match *= (365 - i) as f64 / 365.0;
    }


    round_to_four_decimal_places(1.0 -probability_no_match)
}

fn round_to_four_decimal_places(value: f64) -> f64 {
    (value * 10000.0).round() / 10000.0
}
