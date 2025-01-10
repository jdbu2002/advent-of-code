use std::collections::HashMap;

use crate::extras::read_lines;

pub fn part_one(example: bool) {
    let filenames = ["src/day_1/input.txt", "src/day_1/example.txt"];

    let option = if example { 1 } else { 0 };

    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];

    if let Ok(lines) = read_lines(filenames[option]) {
        for line in lines {
            match line {
                Ok(line) => {
                    let number_pair = line
                        .trim()
                        .split_whitespace()
                        .map(|value| value.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    first_list.push(number_pair[0]);
                    second_list.push(number_pair[1]);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    first_list.sort_by(|a, b| a.cmp(&b));
    second_list.sort_by(|a, b| a.cmp(&b));

    let mut distance = 0;

    for i in 0..first_list.len() {
        let item_distance = first_list[i] - second_list[i];
        distance += item_distance.abs();
    }

    println!("Total Distance: {}", distance);
}

pub fn part_two(example: bool) {
    let filenames = ["src/day_1/input.txt", "src/day_1/example.txt"];

    let option = if example { 1 } else { 0 };

    let mut number_map: HashMap<u32, u32> = HashMap::new();
    let mut number_list: Vec<u32> = vec![];

    if let Ok(lines) = read_lines(filenames[option]) {
        for line in lines {
            match line {
                Ok(line) => {
                    let number_pair = line
                        .trim()
                        .split_whitespace()
                        .map(|value| value.parse::<u32>().unwrap())
                        .collect::<Vec<_>>();

                    number_list.push(number_pair[0]);
                    let key = number_pair[1];

                    if number_map.contains_key(&key) {
                        let value = number_map.get_mut(&key);
                        if let Some(value) = value {
                            *value += 1;
                        }
                    } else {
                        number_map.insert(number_pair[1], 1);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    let mut similarity: u32 = 0;

    for number in number_list {
        let value = number_map.get(&number);
        if let Some(value) = value {
            similarity += *value * number;
        }
    }

    println!("Total Similarity: {}", similarity);
}
