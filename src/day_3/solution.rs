use crate::extras::read_lines;
use regex::Regex;

pub fn part_one(example: bool) {
    let filenames = ["src/day_3/input.txt", "src/day_3/example.txt"];

    let option = if example { 1 } else { 0 };
    let mut total_mul: u32 = 0;

    if let Ok(lines) = read_lines(filenames[option]) {
        for line in lines {
            match line {
                Ok(line) => {
                    let memory = line.trim();
                    let regex = Regex::new("mul\\((?<first>\\d+),(?<second>\\d+)\\)").unwrap();

                    total_mul += regex.captures_iter(memory).fold(0, |acc, caps| {
                        let first = caps.name("first").unwrap().as_str().parse::<u32>().unwrap();
                        let second = caps
                            .name("second")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();

                        return acc + first * second;
                    })
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    println!("Total multiplications: {total_mul}");
}

pub fn part_two(example: bool) {
    let filenames = ["src/day_3/input.txt", "src/day_3/example_2.txt"];

    let option = if example { 1 } else { 0 };
    let mut total_mul: u32 = 0;
    let mut enabled = true;

    if let Ok(lines) = read_lines(filenames[option]) {
        for line in lines {
            match line {
                Ok(line) => {
                    let memory = line.trim();
                    let regex = Regex::new("mul\\((?<first>\\d+),(?<second>\\d+)\\)|(?<do>do)\\(\\)|(?<dont>don't)\\(\\)").unwrap();

                    total_mul += regex.captures_iter(memory).fold(0, |acc, caps| {
                        if let Some(_) = caps.name("dont") {
                            enabled = false;
                            return acc;
                        }

                        if let Some(_) = caps.name("do") {
                            enabled = true;
                            return acc;
                        }

                        if !enabled {
                            return acc;
                        }

                        let first = caps.name("first").unwrap().as_str().parse::<u32>().unwrap();
                        let second = caps
                            .name("second")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();

                        return acc + first * second;
                    })
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    println!("Total enabled multiplications: {total_mul}");
}
