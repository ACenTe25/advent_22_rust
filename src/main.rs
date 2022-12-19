mod general;
mod day_1;
mod day_2;
mod day_3;

use day_1::*;
use day_2::*;
use day_3::*;

fn main() {
    match get_elf_calories() {
        Result::Ok(ans) => println!("Day 1 Part 1 result: {} Calories\n\n", ans),
        Result::Err(error) => println!("Error in Day 1 Part 1: {:?}\n\n", error),
    };

    match get_top_n_elf_calories(3) {
        Result::Ok(ans) => println!("Day 1 Part 2 result: {} Calories\n\n", ans),
        Result::Err(error) => println!("Error in Day 1 Part 2: {:?}\n\n", error),
    };

    match simulate_input_strategy_results() {
        Result::Ok(ans) => println!("Day 2 Part 1 result: {} points.\n\n", ans),
        Result::Err(error) => println!("Error in Day 2 Part 1: {:?}\n\n", error),
    };

    match simulate_input_strategy_results_2() {
        Result::Ok(ans) => println!("Day 2 Part 2 result: {} points.\n\n", ans),
        Result::Err(error) => println!("Error in Day 2 Part 2: {:?}\n\n", error),
    };

    match get_total_priority() {
        Result::Ok(ans) => println!("Day 3 Part 1 result: {} total priority.\n\n", ans),
        Result::Err(error) => println!("Error in Day 3 Part 1: {:?}\n\n", error),
    };

    match get_badges_priority() {
        Result::Ok(ans) => println!("Day 3 Part 2 result: {} total priority.\n\n", ans),
        Result::Err(error) => println!("Error in Day 3 Part 2: {:?}\n\n", error),
    };
}
