// Standard library

// External crates
use anyhow::{Context, Result, anyhow};

// Crate modules
use crate::general::*;

// Function 1:
/* 
This list represents the Calories of the food carried by five Elves:

    The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
    The second Elf is carrying one food item with 4000 Calories.
    The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
    The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
    The fifth Elf is carrying one food item with 10000 Calories.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: 
they'd like to know how many Calories are being carried by the Elf carrying the most Calories. 

In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. How many total Calories is that Elf carrying? */
pub fn get_elf_calories() -> Result<i32> {

    // Read the input file
    let input_string = read_text_input("/home/acente/Advent_inputs/day_1_input_1.txt")
    .context("getting Calories from top-carrying Elf")?;

    // Create Elf Vector
    let mut elf_calories: Vec<i32> = Vec::new();

    // Initialize a Calorie counter
    let mut calorie_counter = 0;

    // Read each line
    for line in input_string.lines() {
        
        if line.is_empty() {

            // Push as the total Calorie count of an Elf, reset counter, continue
            elf_calories.push(calorie_counter);
            calorie_counter = 0;

        } else {

            // Try to convert to i32
            match line.parse::<i32>() {

                // If succeeded, add to the Calorie counter
                Result::Ok(num) => calorie_counter += num,

                // Otherwise, show me what went wrong, push as total Calorie count of an Elf, reset counter, continue
                Result::Err(error) => {

                    println!("Parsing error: {:?}", error);
                    println!("Line: {}", line);
                    elf_calories.push(calorie_counter);
                    calorie_counter = 0;

                }
            }
        }
    };


    // Give me the Calorie count of the Elf with the most Calories
    let Some(max_calories) = elf_calories.iter().max() else {
        return Err(anyhow!("Couldn't find a maximum in the Elf vector, since it was empty."))
    };

    // Return the result
    Ok(*max_calories)
}


/*--- Part Two ---

By the time you calculate the answer to the Elves' question, they've already realized that the Elf 
carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories 
carried by the top three Elves carrying the most Calories. That way, even if one of those Elves 
runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third 
Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories 
carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in 
total?
 */
fn get_elf_calories_vector() -> Result<Vec<i32>> {

    // So now I see how this goes, I will start re-factoring instead. This function just gives me 
    // the Elf vector, and I will write other functions for working with that.

    // Read the input file
    let input_string = read_text_input("/home/acente/Advent_inputs/day_1_input_1.txt")
    .context("getting Calories from top-carrying Elf")?;

    // Create Elf Vector
    let mut elf_calories: Vec<i32> = Vec::new();

    // Initialize a Calorie counter
    let mut calorie_counter = 0;

    // Read each line
    for line in input_string.lines() {
        
        if line.is_empty() {

            // Push as the total Calorie count of an Elf, reset counter, continue
            elf_calories.push(calorie_counter);
            calorie_counter = 0;

        } else {

            // Try to convert to i32
            match line.parse::<i32>() {

                // If succeeded, add to the Calorie counter
                Result::Ok(num) => calorie_counter += num,

                // Otherwise, show me what went wrong, push as total Calorie count of an Elf, reset counter, continue
                Result::Err(error) => {

                    println!("Parsing error: {:?}", error);
                    println!("Line: {}", line);
                    elf_calories.push(calorie_counter);
                    calorie_counter = 0;

                }
            }
        }
    };

    Ok(elf_calories)
}

fn sort_elf_calories_vec(elf_calories_vec: &mut [i32], high_to_low: bool) {
    
    // Sort the vector, low to high
    elf_calories_vec.sort_unstable();

    if high_to_low {
        elf_calories_vec.reverse();
    }

}

fn get_total_count(elf_calories_vec: &[i32], n_first: usize) -> i32 {

    //Check size
    if n_first >= elf_calories_vec.len() {

        // If n_size is larger than the number of elves, return total sum
        elf_calories_vec.iter().sum()

    } else {

        // Otherwise return the total count of the n_first first elements
        elf_calories_vec[0..n_first].iter().sum()

    }
}

pub fn get_top_n_elf_calories(n_first: usize) -> Result<i32> {

    // Get the vector
    let mut elf_calories_vec = get_elf_calories_vector()
    .context("getting top elf calories")?;

    // Sort it, high to low
    sort_elf_calories_vec(&mut elf_calories_vec, true);

    // Get n_first sum
    Ok(get_total_count(&elf_calories_vec, n_first))
}