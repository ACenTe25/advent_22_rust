mod general;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use day_1::*;
use day_2::*;
use day_3::*;
use day_4::*;
use day_5::*;
use day_6::*;

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

    match get_complete_overlaps() {
        Result::Ok(ans) => println!("Day 4 Part 1 result: {} complete overlaps.\n\n", ans),
        Result::Err(error) => println!("Error in Day 4 Part 1: {:?}\n\n", error),
    };

    match get_partial_overlaps() {
        Result::Ok(ans) => println!("Day 4 Part 2 result: {} partial overlaps.\n\n", ans),
        Result::Err(error) => println!("Error in Day 4 Part 2: {:?}\n\n", error),
    };

    match get_stack_tops() {
        Result::Ok(ans) => println!("Day 5 Part 1 result: Top stack boxes: {}.\n\n", ans),
        Result::Err(error) => println!("Error in Day 5 Part 1: {:?}\n\n", error),
    };

    match get_stack_tops_with_cratemover_9001() {
        Result::Ok(ans) => println!("Day 5 Part 2 result: Top stack boxes with CrateMover 9001: {}.\n\n", ans),
        Result::Err(error) => println!("Error in Day 5 Part 2: {:?}\n\n", error),
    };

    match get_start_of_packet_4_unrepeated() {
        Result::Ok(Some(ans)) => println!("Day 6 Part 1 result: Start-of-packet position: {}.\n\n", ans),
        Result::Ok(None) => println!("Day 6 Part 1 result: No start of packet found!"),
        Result::Err(error) => println!("Error in Day 6 Part 1: {:?}\n\n", error),
    };

    match get_start_of_packet_14_unrepeated() {
        Result::Ok(Some(ans)) => println!("Day 6 Part 2 result: Start-of-message position: {}.\n\n", ans),
        Result::Ok(None) => println!("Day 6 Part 2 result: No start of message found!"),
        Result::Err(error) => println!("Error in Day 6 Part 2: {:?}\n\n", error),
    };

}
