// Standard library


// External crates
use anyhow::{Context, Result, anyhow};

// Crate modules
use crate::general::*;

/*--- Day 4: Camp Cleanup ---

Space needs to be cleared before the last supplies can be unloaded from the ships, 
and so several Elves have been assigned the job of cleaning up sections of the 
camp. Every section has a unique ID number, and each Elf is assigned a range of 
section IDs.

However, as some of the Elves compare their section assignments with each other, 
they've noticed that many of the assignments overlap. To try to quickly find 
overlaps and reduce duplicated effort, the Elves pair up and make a big list of 
the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8

For the first few pairs, this list means:

    Within the first pair of Elves, the first Elf was assigned sections 2-4 
    (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 
    (sections 6, 7, 8).
    The Elves in the second pair were each assigned two sections.
    The Elves in the third pair were each assigned three sections: one got sections 
    5, 6, and 7, while the other also got 7, plus 8 and 9.

This example list uses single-digit section IDs to make it easier to draw; your 
actual list might contain larger numbers. Visually, these pairs of section 
assignments look like this:

.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8

Some of the pairs have noticed that one of their assignments fully contains the other. 
For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where 
one assignment fully contains the other, one Elf in the pair would be exclusively 
cleaning sections their partner will already be cleaning, so these seem like the 
most in need of reconsideration. In this example, there are 2 such pairs.

In how many assignment pairs does one range fully contain the other?
 */

struct AssignmentPair {
    elf_1: (i32, i32),
    elf_2: (i32, i32),
    total_overlap: bool,
    // Part 2:
    simple_overlap: bool,
}

impl AssignmentPair {
    
    fn new(assignment_line: &str) -> Result<Self> {

        let limits: Vec<&str> = assignment_line.split(&[',', '-'][..]).collect();

        if limits.len() == 4 {

            let elf_1_lower = limits[0].parse::<i32>().context("an assignment limit is not a number")?;
            let elf_1_upper = limits[1].parse::<i32>().context("an assignment limit is not a number")?;
            let elf_2_lower = limits[2].parse::<i32>().context("an assignment limit is not a number")?;
            let elf_2_upper = limits[3].parse::<i32>().context("an assignment limit is not a number")?;

            let elf_1 = (elf_1_lower, elf_1_upper);
            let elf_2 = (elf_2_lower, elf_2_upper);

            let total_overlap = fully_contains(elf_1, elf_2);

            // Part 2:
            let simple_overlap = partially_contains(elf_1, elf_2);

            Ok(
                AssignmentPair { 
                    elf_1,
                    elf_2, 
                    total_overlap,
                    // Part 2:
                    simple_overlap
                }
            )
        } else {

            Err(anyhow!("An assignment line doesn't have exactly 4 limits: '{}'", assignment_line))
        }
    }
}

fn fully_contains(range_1: (i32, i32), range_2: (i32, i32)) -> bool {

    if range_1.0 <= range_2.0 && range_1.1 >= range_2.1 { true }
    else if range_2.0 <= range_1.0 && range_2.1 >= range_1.1 {true }
    else { false }

}

fn get_assignment_lines() -> Result<Vec<String>> {

    // Get input file as a string
    let input_string = read_text_input("/home/acente/Advent_inputs/day_4_input_1.txt")
    .context("getting assignment pairs list")?;

    // Turn lines into vector of Strings
    let mut input_lines: Vec<String> = Vec::new();

    for line in input_string.lines() {
        input_lines.push(line.to_string());
    }

    Ok(input_lines)
}

fn get_assignments_vec(assignment_lines: &Vec<String>) -> Result<Vec<AssignmentPair>> {

    let mut assignments_vec: Vec<AssignmentPair> = Vec::new(); 

    for assignment in assignment_lines {

        assignments_vec.push(AssignmentPair::new(assignment).context("getting assingment pairs")?);

    }

    Ok(assignments_vec)
}

fn get_number_of_complete_overlaps(assignments_vec: &[AssignmentPair]) -> i32 {

    let mut complete_overlaps = 0;

    for assignment_pair in assignments_vec {
        if assignment_pair.total_overlap {
            complete_overlaps += 1;
        }
    }

    complete_overlaps
}

pub fn get_complete_overlaps() -> Result<i32> {

    let assignment_lines = get_assignment_lines()
    .context("counting complete overlaps in assignments")?;

    let assignments_vec = get_assignments_vec(&assignment_lines)
    .context("counting complete overlaps in assignments")?;

    Ok(get_number_of_complete_overlaps(&assignments_vec))
}

/*--- Part Two ---

It seems like there is still quite a bit of duplicate work planned. Instead, the 
Elves would like to know the number of pairs that overlap at all.

In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, 
while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do 
overlap:

    5-7,7-9 overlaps in a single section, 7.
    2-8,3-7 overlaps all of the sections 3 through 7.
    6-6,4-6 overlaps in a single section, 6.
    2-6,4-8 overlaps in sections 4, 5, and 6.

So, in this example, the number of overlapping assignment pairs is 4.

In how many assignment pairs do the ranges overlap?
 */

 // Part 2:
fn partially_contains(range_1: (i32, i32), range_2: (i32, i32)) -> bool {

    let actual_range_1 = range_1.0..=range_1.1;
    let actual_range_2 = range_2.0..=range_2.1;

    if actual_range_1.contains(&range_2.0) || 
    actual_range_1.contains(&range_2.1) ||
    actual_range_2.contains(&range_1.0) ||
    actual_range_2.contains(&range_1.1) {

        true

    } else {
        
        false
    }
}

// Part 2:
fn get_number_of_partial_overlaps(assignments_vec: &[AssignmentPair]) -> i32 {

    let mut partial_overlaps = 0;

    for assignment_pair in assignments_vec {
        if assignment_pair.simple_overlap {
            partial_overlaps += 1;
        }
    }

    partial_overlaps
}

pub fn get_partial_overlaps() -> Result<i32> {

    let assignment_lines = get_assignment_lines()
    .context("counting complete overlaps in assignments")?;

    let assignments_vec = get_assignments_vec(&assignment_lines)
    .context("counting complete overlaps in assignments")?;

    Ok(get_number_of_partial_overlaps(&assignments_vec))
}