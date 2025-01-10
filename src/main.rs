mod day_1;
mod extras;

use day_1::solution::{part_one as d1_p1, part_two as d1_p2};

fn main() {
    loop {
        println!(
            "Enter the number of the problem you want to run (1 - 25), other number will exit: "
        );

        let mut problem_number_string = String::new();

        std::io::stdin()
            .read_line(&mut problem_number_string)
            .expect("Failed to read line");

        let problem_number = problem_number_string.trim().parse::<u8>().unwrap_or(0);

        match problem_number {
            1 => {
                println!("Problem 1: ");

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
                    d1_p1(example);
                } else {
                    d1_p2(example);
                }
            }
            _ => {
                println!("Exiting...");
                break;
            }
        }
    }
}
