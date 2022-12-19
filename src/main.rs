mod general;
mod day_1;

use day_1::*;

fn main() {
    match get_elf_calories() {
        Result::Ok(ans) => println!("Day 1 Part 1 result: {} Calories\n\n", ans),
        Result::Err(error) => println!("Error in Day 1 Part 1: {:?}\n\n", error),
    };

    match get_top_n_elf_calories(3) {
        Result::Ok(ans) => println!("Day 1 Part 2 result: {} Calories\n\n", ans),
        Result::Err(error) => println!("Error in Day 1 Part 2: {:?}\n\n", error),
    };
}
