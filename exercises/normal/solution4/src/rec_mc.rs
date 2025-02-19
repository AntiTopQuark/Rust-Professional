pub fn dp_rec_mc(amount: u32) -> u32 {
    let coins = [100, 50, 30, 20, 10, 5, 2, 1];  // 纸币面额从大到小排列
    let mut remaining_amount = amount;
    let mut count = 0;
    
    for &coin in coins.iter() {
        if remaining_amount == 0 {
            break;
        }
        
        // 使用尽量多的当前面额纸币
        count += remaining_amount / coin;
        remaining_amount %= coin;
    }

    count
}
