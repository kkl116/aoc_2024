use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    //basically define the expected states of each char - essentially creating a finite state automata
    //but it's painful af  
    let num_chars = "1234567890";
    let mut expected_map: HashMap<char, Option<Vec<char>>> = HashMap::new();
    expected_map.insert('m', Some(vec!['u']));
    expected_map.insert('u', Some(vec!['l']));
    expected_map.insert('l', Some(vec!['(']));
    expected_map.insert('(', Some(num_chars.chars().collect()));
    num_chars.chars().for_each(|c| {
        expected_map.insert(c, Some(num_chars.chars().chain(vec![',', ')']).collect()));
    });
    expected_map.insert(',', Some(num_chars.chars().collect()));
    expected_map.insert(')', Some(vec!['m']));

    let content = read_to_string("day_3_1/src/input.txt").unwrap();
    let initial_expected = Some(vec!['m']);
    let mut nums: Vec<i64> = Vec::new();
    let mut int1: Vec<char> = Vec::new();
    let mut int2: Vec<char> = Vec::new();
    let mut expected: Option<Vec<char>> = initial_expected.clone();  
    let mut is_int_2 = false;
    content.chars().for_each(|c| {
        println!("c is: {}", c);
        println!("expected is: {:?}", expected);

        match &expected {
            Some(expected_chars) => {
                //check if current char is valid 
                if expected_chars.contains(&c) {
                    //is numeric - push to the right vec 
                    if c.is_numeric() {
                        if is_int_2 {
                            int2.push(c)
                        } else {
                            int1.push(c)
                        }
                    }

                    if c == '(' {
                        is_int_2 = false; 
                    }

                    if c == ',' {
                        is_int_2 = true;
                    }

                    if c == ')' {
                        let int1_string: String = int1.iter().collect();
                        let int2_string: String = int2.iter().collect(); 
                    
                        // Check if both parses were successful
                        match (int1_string.parse::<i64>(), int2_string.parse::<i64>()) {
                            (Ok(num1), Ok(num2)) => {
                                nums.push(num1);
                                nums.push(num2);
                            }
                            _ => {}
                        }

                        int1.clear();
                        int2.clear();
                    }

                    //update expected
                    if let Some(value) = expected_map.get(&c) {
                        expected = value.clone();
                    } else {
                        expected = None;
                    }
                } else {
                    expected = initial_expected.clone();
                    int1.clear();
                    int2.clear();
                }
            },

            None => {
                int1.clear();
                int2.clear();
                expected = initial_expected.clone();
            }
        }
    });

    let sum: i64 = nums.chunks(2)
    .map(|pair| pair[0] * pair[1])
    .sum();

    println!("nums is: {:?}", nums);
    println!("sum is: {}", sum);
}
