use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();  

    for line in read_to_string("day_1_2/src/input.txt").unwrap().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        list1.push(parts[0].parse::<i64>().unwrap());
        list2.push(parts[1].parse::<i64>().unwrap());
    }

    //naive solution - build freq map then iterate
    let mut frequency: HashMap<i64, i64> = HashMap::new();

    list2.iter().for_each(|&num| {
        *frequency.entry(num).or_insert(0) += 1
    });

    let mut res: i64 = 0;
    list1.iter().for_each(|&num| {
        let freq = frequency.get(&num)
        .unwrap_or_else(|| {
            &0
        });

        res += &num * freq;
    });

    println!("sum is: {}", res)
}
