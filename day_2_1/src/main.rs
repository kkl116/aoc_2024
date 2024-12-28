use std::fs::read_to_string;

fn main() {

    let mut num_safe = 0;
    for line in read_to_string("day_2_1/src/input.txt").unwrap().lines() {

        let report: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

        num_safe += if is_report_safe(report) { 1 } else { 0 };
    };

    println!("num safe is: {}", num_safe);
}

fn is_report_safe(report: Vec<i32>) -> bool {
    //diff between each element - then inspect 
    let mut diffs: Vec<i32> = Vec::new();

    report.iter()
    .enumerate()
    .for_each(|(i, _)| {
        if i > 0 {
            diffs.push(report[i-1] - report[i])
        }
    });

    return (is_grad_decrease(diffs.clone()) || is_grad_increase(diffs.clone())) &&
    is_step_size_valid(diffs.clone()); 
}

fn is_grad_increase(diffs: Vec<i32>) -> bool {
    return diffs.iter()
    .all(|num| num > &0)
}

fn is_grad_decrease(diffs: Vec<i32>) -> bool {
    return diffs.iter()
    .all(|num| num < &0)
}

fn is_step_size_valid(diffs: Vec<i32>) -> bool {
    return !diffs.iter()
    .any(|num| num.abs() > 3);
}
