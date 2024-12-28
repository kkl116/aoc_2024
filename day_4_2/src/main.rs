use std::fs::read_to_string;
use std::collections::HashMap;


fn main() {
    let mut point_map: HashMap<(i32, i32), char> = HashMap::new();

    read_to_string("day_4_1/src/input.txt")
    .unwrap()
    .lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            point_map.insert((x as i32, y as i32), c);
        });
    });

    let mut num = 0;

    point_map.keys().for_each(|point| {
        let strings: Vec<String> = vec![
            find_right_diag(point.0, point.1, point_map.clone()),
            find_left_diag(point.0, point.1, point_map.clone()),
        ];

        num += if strings.iter().all(|s| s == "SAM" || s == "MAS") { 1 } else { 0 };
    });


    println!("num is {}", num);
}

fn find_right_diag(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut diag: Vec<char> = Vec::new(); 

    let diffs = vec![(-1, -1), (0, 0), (1, 1)];

    diffs.iter().for_each(|(y_diff, x_diff)| {
        let stored_c = point_map.get(&(i+y_diff, j+x_diff));
        match stored_c {
            Some(c) => diag.push(*c),
            None => {}
        }
    });

    return diag.iter().collect();
}

fn find_left_diag(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut diag: Vec<char> = Vec::new(); 

    let diffs = vec![(-1, 1), (0, 0), (1, -1)];

    diffs.iter().for_each(|(y_diff, x_diff)| {
        let stored_c = point_map.get(&(i+y_diff, j+x_diff));
        match stored_c {
            Some(c) => diag.push(*c),
            None => {}
        }
    });

    return diag.iter().collect();
}

