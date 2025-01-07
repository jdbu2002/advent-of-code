mod day_1;
mod extras;

use day_1::solution::solution as day_1_solution;

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
                println!("Want to run the example? (y/n): ");
                let mut example_string = String::new();
                std::io::stdin()
                    .read_line(&mut example_string)
                    .expect("Failed to read line");

                let example = example_string.trim() == "y";

                day_1_solution(example);
            }
            _ => {
                println!("Exiting...");
                break;
            }
        }
    }
}
