use crate::extras::read_lines;

pub fn solution(example: bool) {
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
