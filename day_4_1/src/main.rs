use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;


fn main() {
    let mut point_map: HashMap<(i32, i32), char> = HashMap::new();

    read_to_string("day_4_1/src/input.txt")
    .unwrap()
    .lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            point_map.insert((x as i32, y as i32), c);
        });
    });

    let regex = Regex::new(r"XMAS").unwrap();

    let mut num = 0;


    point_map.keys().for_each(|point| {
        let strings: Vec<String> = vec![
            find_right(point.0, point.1, point_map.clone()),
            find_left(point.0, point.1, point_map.clone()),
            find_up(point.0, point.1, point_map.clone()),
            find_down(point.0, point.1, point_map.clone()),
            find_up_right(point.0, point.1, point_map.clone()),
            find_up_left(point.0, point.1, point_map.clone()),
            find_down_right(point.0, point.1, point_map.clone()),
            find_down_left(point.0, point.1, point_map.clone()),
        ];

        strings.iter().for_each(|s| {
            regex.find_iter(s).for_each(|_| {
                num += 1;
            })
        });
    });


    println!("num is {}", num);
}

fn find_right(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut row: Vec<char> = Vec::new(); 
    let mut j_prime = j.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i, j_prime));
        match stored_c {
            Some(c) => row.push(*c),
            None => {}
        }

        j_prime += 1;
    }

    return row.iter().collect();
}

fn find_left(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut row: Vec<char> = Vec::new(); 
    let mut j_prime = j.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i, j_prime));
        match stored_c {
            Some(c) => row.push(*c),
            None => {}
        }
        j_prime -= 1;
    }

    return row.iter().collect();
}


fn find_up(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut col: Vec<char> = Vec::new(); 
    let mut i_prime = i.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i_prime, j));
        match stored_c {
            Some(c) => col.push(*c),
            None => {}
        }
        i_prime += 1;
    }

    return col.iter().collect();
}

fn find_down(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut col: Vec<char> = Vec::new(); 
    let mut i_prime = i.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i_prime, j));
        match stored_c {
            Some(c) => col.push(*c),
            None => {}
        }
        i_prime -= 1;
    }

    return col.iter().collect();
}

fn find_up_right(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut diag: Vec<char> = Vec::new(); 
    let mut i_prime = i.clone();
    let mut j_prime = j.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i_prime, j_prime));
        match stored_c {
            Some(c) => diag.push(*c),
            None => {}
        }
        i_prime += 1;
        j_prime += 1
    }

    return diag.iter().collect();
}

fn find_down_right(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut diag: Vec<char> = Vec::new(); 
    let mut i_prime = i.clone();
    let mut j_prime = j.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i_prime, j_prime));
        match stored_c {
            Some(c) => diag.push(*c),
            None => {}
        }
        i_prime -= 1;
        j_prime += 1
    }

    return diag.iter().collect();
}

fn find_up_left(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut diag: Vec<char> = Vec::new(); 
    let mut i_prime = i.clone();
    let mut j_prime = j.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i_prime, j_prime));
        match stored_c {
            Some(c) => diag.push(*c),
            None => {}
        }
        i_prime += 1;
        j_prime -= 1
    }

    return diag.iter().collect();
}

fn find_down_left(i: i32, j: i32, point_map: HashMap<(i32, i32), char>) -> String {
    let mut diag: Vec<char> = Vec::new(); 
    let mut i_prime = i.clone();
    let mut j_prime = j.clone();
    for _ in 0..4 {
        let stored_c = point_map.get(&(i_prime, j_prime));
        match stored_c {
            Some(c) => diag.push(*c),
            None => {}
        }
        i_prime -= 1;
        j_prime -= 1
    }

    return diag.iter().collect();
}

