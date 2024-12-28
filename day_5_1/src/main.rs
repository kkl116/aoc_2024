use std::fs::read_to_string;
use std::collections::HashMap;
use std::ops::Deref;

fn main() {
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    let mut res = 0;

    read_to_string("day_5_1/src/input.txt").unwrap()
    .lines()
    .filter(|line| !line.trim().is_empty())
    .for_each(|line| {

        //build rules 
        if line.contains("|") {
            let mut splitted = line.split("|");
            let key = splitted.next().unwrap();
            let value = splitted.next().unwrap();
            rules.entry(key.to_string())
                .or_insert_with(Vec::new)
                .push(value.to_string())
        } else {
            //parse line
            let pages: Vec<String> = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
            
            //for each num - check if the previous 
            for i in (0..pages.len()).rev() {

                let prev_block = pages[0..i].to_vec();
                let prior = rules.get(pages.get(i)
                .unwrap())
                .map_or(Vec::new(), |v| v.to_vec());

                let conflict: Option<usize> = prev_block.iter().enumerate()
                .filter(|(_, s)| prior.contains(s))
                .map(|(i, _)| i)
                .min();

                match conflict { 
                    Some(i) => {
                        
                    }

                    None => {

                    }
                }

            }
        }
    });
}
