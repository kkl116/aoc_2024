use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let content = read_to_string("day_3_1/src/input.txt").unwrap();
    let all_regex = Regex::new(r"mul\((-?\d+),(-?\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut matches = Vec::new(); 

    all_regex.find_iter(&content)
    .for_each(|matched| {
        matches.push((matched.start(), matched.as_str()))
    });

    let mut is_do = true;
    let mut nums = Vec::new();
    let num_regex = Regex::new(r"(-?\d+)").unwrap();
    matches.iter()
    .map(|matches| matches.1)
    .for_each(|matched_string| {
        if matched_string.contains("do") {
            is_do = true;
        }

        if matched_string.contains("don't") {
            is_do = false;
        }

        num_regex.find_iter(&matched_string)
        .for_each(|matched| {
            if is_do {
                nums.push(matched.as_str().parse::<i32>().unwrap())
            }
        })
    });


    let sum: i32 = nums.chunks(2)
    .map(|pair| pair[0] * pair[1])
    .sum();

    println!("sum is: {}", sum);
}
