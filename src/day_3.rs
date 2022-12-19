// Standard library
use std::collections::HashMap;

// External crates
use anyhow::{Context, Result, anyhow};

// Crate modules
use crate::general::*;

// Part 1:
/*--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for 
the jungle journey. Unfortunately, that Elf didn't quite follow the packing 
instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to 
go into exactly one of the two compartments. The Elf that did the packing failed 
to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your 
    puzzle input), but they need your help finding the errors. Every item type is 
    identified by a single lowercase or uppercase letter (that is, a and A refer 
        to different types of items).

The list of items for each rucksack is given as characters all on a single line. 
A given rucksack always has the same number of items in each of its two compartments, 
so the first half of the characters represent items in the first compartment, while 
the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means 
    its first compartment contains the items vJrwpWtwJgWr, while the second 
    compartment contains the items hcsFMMfFFhFp. The only item type that appears 
    in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. 
    The only item type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only 
    common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both compartments 
of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of 
these is 157.

Find the item type that appears in both compartments of each rucksack. What is the 
sum of the priorities of those item types?
 */

struct ItemTypePriorities {
    dictionary: HashMap<char, i32>
}

impl ItemTypePriorities {

    fn new() -> Self {
        let mut dictionary: HashMap<char, i32> = HashMap::new();
    
        let keys = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let mut values_counter = 1;

        for char in keys.chars() {
            dictionary.insert(char, values_counter);
            values_counter += 1;
        } 

        ItemTypePriorities {dictionary}
    }

    fn get_item_priority(&self, item_type: char) -> Result<i32> {
        
        match self.dictionary.get(&item_type) {
            Some(priority) => Ok(*priority),
            _ => Err(anyhow!("Unknown item type '{}'", item_type)),
        }
    }

    fn get_rucksacks_priorities(&self, rucksacks: &[Rucksack]) -> Vec<i32> {

        let mut priorities: Vec<i32> = Vec::new();

        for rucksack in rucksacks {
            
            // Should never panic because Rucksack can't contain invalid Item Types
            priorities.push(self.get_item_priority(rucksack.wrong_item)
            .expect("Invalid Item Type in Rucksack!"));
        }

        priorities

    }
}

struct Rucksack {
    compartment_1: String,
    compartment_2: String,
    wrong_item: char,
}

impl Rucksack {

    fn new(items: &str) -> Result<Self> {

        // Check for invalid item types
        for item in items.chars() {
            if item.is_ascii_lowercase() || item.is_ascii_uppercase() {
                continue
            } else {
                return Err(anyhow!("Unknown item type '{}'", item))
            }
        };

        // See if both compartments include the same amount of items
        let num_items = items.len();

        if num_items % 2 == 0 {
            
            // Split for compartments and look for the wrong repeated item
            let mid = num_items / 2;

            let (
                compartment_1_str, 
                compartment_2_str
            ) = items.split_at(mid);

            let mut wrong_item_opt: Option<char> = None;

            for item in compartment_1_str.chars() {
                if compartment_2_str.contains(item) {
                    wrong_item_opt = Some(item);
                    break
                }
            }

            // Get the repeated wrong item
            let Some(wrong_item) = wrong_item_opt else {

                // There was no repeated item
                return Err(anyhow!("No repeated item in compartments: '{}'", items))

            };

            Ok(Rucksack {
                compartment_1: compartment_1_str.to_string(),
                compartment_2: compartment_2_str.to_string(),
                wrong_item
            })

        } else {

            Err(anyhow!("Uneven number of items in Rucksack: '{}'", items))

        }
    }

}

fn get_rucksack_lines() -> Result<Vec<String>> {

    // Get input string
    let input_string = read_text_input("/home/acente/Advent_inputs/day_3_input_1.txt")
    .context("getting list of rucksacks")?;

    // Turn lines into vector of Strings
    let mut input_lines: Vec<String> = Vec::new();

    for line in input_string.lines() {
        input_lines.push(line.to_string());
    }

    Ok(input_lines)

}

fn get_rucksack_vec(rucksack_lines: &Vec<String>) -> Result<Vec<Rucksack>> {

    let mut rucksack_vec: Vec<Rucksack> = Vec::new();
    
    // For each line, get a Rucksack
    for line in rucksack_lines {

        rucksack_vec.push(Rucksack::new(line).context("getting rucksack contents")?);

    }

    Ok(rucksack_vec)
}

// Day 3 Part 1:
pub fn get_total_priority() -> Result<i32> {

    let item_types = ItemTypePriorities::new();

    let rucksack_lines = get_rucksack_lines()
    .context("getting total rucksack priority sum")?;

    let rucksack_vec = get_rucksack_vec(&rucksack_lines)
    .context("getting total rucksack priority sum")?;

    let priorities_vec = item_types.get_rucksacks_priorities(&rucksack_vec);

    Ok(priorities_vec.iter().sum())
}