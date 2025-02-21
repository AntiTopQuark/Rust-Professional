use std::{collections::{HashMap, HashSet}, slice::SliceIndex};

use regex::Regex;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        Self { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.parent[x];
        let y_root = self.parent[y];
        if x_root == y_root {
            return;
        } else {
            for i in 0..self.parent.len() {
                if self.parent[i] == x_root {
                    self.parent[i] = y_root;
                }
            }
        }
        
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    

}

pub fn count_provinces() -> String {
    // 读取json
    let file_content = std::fs::read_to_string("district.json").unwrap();
    let mut res = vec![];
    let mut iter = file_content.lines();
    
    let mut city_all = vec![];
    while let Some(line) =  iter.next() {
        let re1 = Regex::new(r#""(\d+)":\s*\{"#).unwrap();
        let mut current_number = 1;
        let mut need_output_once = false;
        for captures in re1.captures_iter(&line).take(1) {
            // 提取数字部分
            let number: usize = captures.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();

            if number > current_number {
                need_output_once = true;
                current_number = number;
            }
            
            let re = Regex::new(r#"[\u4e00-\u9fa5]+"#).unwrap(); 
            let mut city_batch = vec![];
            'aaa:while let Some(line2) = iter.next() {
                let mut city_vec = vec![];
                for captures in re.captures_iter(&line2) {
                    let city = captures.get(0).map_or("", |m| m.as_str());
                    city_vec.push(city);
                }
                if city_vec.is_empty() {
                    break;
                }
                city_batch.push(city_vec);
            }
            // println!("{number}, {:?}", city_batch);
            city_all.push(city_batch);
        }
    }

    for city_batch in city_all {
        let mut city_nums = 0;
        let mut city_index_map = HashMap::new();
        for connection in &city_batch {
            for c in connection {
                city_index_map.entry(c).or_insert(city_nums);
                city_nums = city_index_map.len();
            }
        }
        let mut uf = UnionFind::new(city_nums);

        for connection in &city_batch {
            let first_city_index = city_index_map.get(connection.get(0).unwrap()).unwrap();

            for i in 1..connection.len() {
                let index = city_index_map.get(connection.get(i).unwrap()).unwrap();
                uf.union(*first_city_index, *index);
            }
        }
        let mut provinces = HashSet::new();
        for i in 0..city_nums {
            provinces.insert(uf.find(i));
        }
        res.push(provinces.len());
    }
    let number: Vec<String> = res.iter().map(|&x| x.to_string()).collect();

    let sentence = number.join(",");  // 使用空格作为分隔符
    sentence
}
