use std::collections::HashSet;


fn sieve_of_eratosthenes(limit: usize) -> (Vec<usize>, Vec<usize>) {
    // 创建一个大小为 limit + 1 的布尔向量，所有值初始化为 true
    let mut is_prime = vec![true; limit + 1];
    
    // 0 和 1 不是素数
    is_prime[0] = false;
    is_prime[1] = false;
    
    // 从 2 开始筛选
    for i in 2..=(limit as f64).sqrt() as usize {
        if is_prime[i] {
            // 标记 i 的所有倍数为合数
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    // 返回所有标记为素数的数字
    ((2..=limit).filter(|&x| is_prime[x]).collect(), (2..=limit).filter(|&x| !is_prime[x] && x % 2 == 1).collect())
}

pub fn goldbach_conjecture() -> String {

    let (primes, composites) = sieve_of_eratosthenes(10000);
    let twice_square = (0..200).map(|n| n*n*2).collect::<HashSet<usize>>();
    let mut result = vec![];
    for num in composites {
        let mut found = false;
        for prime in &primes {
            if prime > &num {
                break;
            }
            if twice_square.contains(&(num - prime)) {
                found = true;
                break;
            }
        }
        if !found {
            result.push(num);
        }
        if result.len() == 2 {
            return format!("{},{}", result[0], result[1]);
        }

    }
    "".to_string()
}
