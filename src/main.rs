mod day_1;
mod day_2;
mod day_3;
mod extras;

use day_1::solution::{part_one as d1_p1, part_two as d1_p2};
use day_2::solution::{part_one as d2_p1, part_two as d2_p2};
use day_3::solution::{part_one as d3_p1, part_two as d3_p2};

fn problem_selector(problem_number: u8, part1: fn(bool), part2: fn(bool)) {
    println!("Problem {problem_number}: ");

    let mut part = String::new();

    println!("Select part (1 or 2): ");
    std::io::stdin()
        .read_line(&mut part)
        .expect("Failed to read line");

    let mut example_string = String::new();

    println!("Want to run the example? (y/n): ");
    std::io::stdin()
        .read_line(&mut example_string)
        .expect("Failed to read line");

    let example = example_string.trim() == "y";

    if part.trim() == "1" {
        part1(example);
    } else {
        part2(example);
    }
}

fn main() {
    let problem_funcs: [(fn(bool), fn(bool)); 3] = [(d1_p1, d1_p2), (d2_p1, d2_p2), (d3_p1, d3_p2)];

    loop {
        println!(
            "Enter the number of the problem you want to run (1 - 25), other number will exit: "
        );

        let mut problem_number_string = String::new();

        std::io::stdin()
            .read_line(&mut problem_number_string)
            .expect("Failed to read line");

        let problem_number = problem_number_string.trim().parse::<u8>().unwrap_or(1);

        if problem_number > 25 {
            println!("Exiting...");
            break;
        }

        problem_selector(
            problem_number,
            problem_funcs[problem_number as usize - 1].0,
            problem_funcs[problem_number as usize - 1].1,
        );
    }
}
