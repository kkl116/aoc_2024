use std::fs::read_to_string;

fn main() {
    
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();  

    for line in read_to_string("day_1_1/src/input.txt").unwrap().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        list1.push(parts[0].parse::<i32>().unwrap());
        list2.push(parts[1].parse::<i32>().unwrap());
    }
    
    //naive solution - sort both then get the diff 
    list1.sort();
    list2.sort();

    let mut out: Vec<i32> = Vec::new();
    for (i, j) in list1.iter().zip(list2.iter()) {
        let diff = i - j;
        out.push(diff.abs())
    }

    let total: i32 = out.iter().sum();
    println!("sum is: {}", total)
}
