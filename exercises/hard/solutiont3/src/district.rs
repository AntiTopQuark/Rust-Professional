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
    // let mut _res = vec![];
    let mut iter = file_content.lines();
    while let Some(line) =  iter.next() {
        let re1 = Regex::new(r#""(\d+)":\s*\{"#).unwrap();
        let mut current_number = 1;
        let mut need_output_once = false;
        for captures in re1.captures_iter(&line).take(1) {
            // 提取数字部分
            let number = captures.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();

            if number > current_number {
                need_output_once = true;
                current_number = number;
            }
            
            let re = Regex::new(r#"[\u4e00-\u9fa5]+"#).unwrap(); 
            
            let mut city_map = HashMap::new();
            'aaa:while let Some(line2) = iter.next() {
                let mut first_city = "";
                let mut city_vec = vec![];
                for captures in re.captures_iter(&line2) {
                    let city = captures.get(0).map_or("", |m| m.as_str());
                    if city == "" {
                        break 'aaa;
                    } else if first_city == "" {
                        first_city = city;
                    } else {
                        city_vec.push(city);
                    }
                }
                city_map.insert(first_city, city_vec);
            }
            println!("{number}, {:?}", city_map);
            println!();
        }
        
    }



    
    // if let Value::Object(batchs) = data {
    //     for (key, value) in batchs {
    //         println!("{}:{}", key, value);
            // let city_data: HashMap<String, Vec<String>> = serde_json::from_value(value).unwrap();
            // let mut city_nums = 0;
            // let mut city_index_map = HashMap::new();
            // for (city, connection) in &city_data {
            //     city_index_map.entry(city).or_insert(city_nums);
            //     city_nums = city_index_map.len();
            //     for c in connection {
            //         city_index_map.entry(c).or_insert(city_nums);
            //         city_nums = city_index_map.len();
            //         println!("{}:{}", city, c);

            //     }
            // }

            // let mut uf = UnionFind::new(city_nums);

            // for (city, connections) in &city_data {
            //     let city_index = city_index_map.get(city).copied().unwrap();
                
            //     for c in connections {
            //         let c_index = city_index_map.get(c).copied().unwrap();
            //         uf.union(city_index, c_index);
            //         println!("{}:{}, {}:{}, {}, {}", city, city_index, c, c_index, uf.parent[city_index], uf.parent[c_index]);
            //     }
            // }

            // println!("{:?}", uf.parent);
            
            // // wordcount
            // let mut provinces = HashSet::new();
            // for i in 0..city_nums {
            //     provinces.insert(uf.find(i));
            // }
            // println!("{:?}", provinces);
    //     }
    // }

    "".to_string()
}
