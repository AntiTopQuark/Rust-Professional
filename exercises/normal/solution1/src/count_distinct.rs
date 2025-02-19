use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    // 使用HashSet
    let mut set = HashSet::new();

    // 使用split方法将字符串分割成单词
    for word in input_str.split(',') {
        // 将单词插入到HashSet中
        set.insert(word);
    }

    set.len()
}
