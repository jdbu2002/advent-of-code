use crate::extras::read_lines;

pub fn part_one(example: bool) {
    let filenames = ["src/day_2/input.txt", "src/day_2/example.txt"];

    let option = if example { 1 } else { 0 };
    let mut safe_reports: u32 = 0;

    if let Ok(lines) = read_lines(filenames[option]) {
        for line in lines {
            match line {
                Ok(line) => {
                    let report = line
                        .trim()
                        .split_whitespace()
                        .map(|value| value.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    if validate_report(&report) {
                        safe_reports += 1;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    println!("Safe reports: {safe_reports}");
}

pub fn part_two(example: bool) {
    let filenames = ["src/day_2/input.txt", "src/day_2/example.txt"];

    let option = if example { 1 } else { 0 };
    let mut safe_reports: u32 = 0;

    if let Ok(lines) = read_lines(filenames[option]) {
        for line in lines {
            match line {
                Ok(line) => {
                    let report = line
                        .trim()
                        .split_whitespace()
                        .map(|value| value.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    if validate_report(&report) {
                        safe_reports += 1;
                        continue;
                    }

                    for i in 0..report.len() {
                        let mut new_report = report.clone();
                        new_report.remove(i);

                        if validate_report(&new_report) {
                            safe_reports += 1;
                            break;
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
            }
        }
    }

    println!("Safe reports: {safe_reports}");
}

fn validate_report(report: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut valid = true;

    for i in 0..(report.len() - 1) {
        if increasing && report[i] < report[i + 1] && report[i + 1] - report[i] <= 3 {
            decreasing = false;
        } else if decreasing && report[i] > report[i + 1] && report[i] - report[i + 1] <= 3 {
            increasing = false;
        } else {
            valid = false;
            break;
        }
    }

    return valid;
}
