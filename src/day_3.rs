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

    // Part 2:
    fn get_badges_priorities(&self, rucksack_groups: &[RucksackGroup]) -> Vec<i32> {

        let mut priorities: Vec<i32> = Vec::new();

        for rucksack_group in rucksack_groups {

            // Should never panic because RucksackGroup can't contain invalid Item Types
            priorities.push(self.get_item_priority(rucksack_group.badge)
        .expect("Invalid Item Type as Badge!"));
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

/*--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another 
issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge 
that identifies their group. For efficiency, within each group of three Elves, 
the badge is the only item type carried by all three Elves. That is, if a group's 
badge is item type B, then all three Elves will have item type B somewhere in 
their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker 
on the badges. All of the badges need to be pulled out of the rucksacks so the new 
authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. 
The only way to tell which item type is the right one is by finding the one item 
type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group 
can have a different badge item type. So, in the above example, the first group's 
rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is 
lowercase r; this must be their badges. In the second group, their badge item 
type must be Z.

Priorities for these items must still be found to organize the sticker attachment 
efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. 
The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is 
the sum of the priorities of those item types?
 */



 struct RucksackGroup {
    rucksack_1_items: String,
    rucksack_2_items: String,
    rucksack_3_items: String,
    badge: char
}

fn get_elf_rucksack_groups(rucksack_lines: &Vec<String>) -> Result<Vec<RucksackGroup>> {

    // Create groups vector
    let mut rucksack_group_vec: Vec<RucksackGroup> = Vec::new();

    // Get a rucksack counter to assemble groups of 3
    let mut rucksack_counter = 1;

    // Create optional buffers for the contents
    let mut buffer_rucksack_1: Option<String> = None;
    let mut buffer_rucksack_2: Option<String> = None;

    let mut last_flag_3 = false;
    
    for rucksack in rucksack_lines {

        // Check that rucksack contains only valid items
        for item in rucksack.chars() {
            if item.is_ascii_lowercase() || item.is_ascii_uppercase() {
                continue
            } else {
                return Err(anyhow!("Unknown item type '{}'", item))
            }
        }

        // Save rucksack 1 contents in buffer
        if rucksack_counter == 1 {

            buffer_rucksack_1 = Some(rucksack.clone());
            rucksack_counter += 1;
            last_flag_3 = false;

        // Save rucksack 2 contents in buffer
        } else if rucksack_counter == 2 {

            buffer_rucksack_2 = Some(rucksack.clone());
            rucksack_counter += 1;
            last_flag_3 = false;
            

        // Get contents from previous buffers and get current contents
        } else {

            let Some(rucksack_1_items) = buffer_rucksack_1.clone() else {

                return Err(anyhow!("Uninitialized Rucksack!"))

            };

            let Some(rucksack_2_items) = buffer_rucksack_2.clone() else {

                return Err(anyhow!("Uninitialized Rucksack!"))
                
            };

            let rucksack_3_items = rucksack.clone();

            // Get badge
            let mut badge_opt: Option<char> = None;

            for item in rucksack_1_items.chars() {
                if rucksack_2_items.contains(item) && rucksack_3_items.contains(item) {
                    badge_opt = Some(item);
                    break
                }
            }

            let Some(badge) = badge_opt else {

                return Err(
                    anyhow!(
                        "Elf group without a badge:\nElf 1 Rucksack: '{}'\nElf 2 Rucksack: '{}'\nElf 3 Rucksack: '{}'\n",
                        rucksack_1_items,
                        rucksack_2_items,
                        rucksack_3_items
                    )
                )
            };

            // Create this Group
            rucksack_group_vec.push(
                RucksackGroup {
                    rucksack_1_items,
                    rucksack_2_items,
                    rucksack_3_items,
                    badge
                }
            );

            // Reset buffers and counter
            buffer_rucksack_1 = None;
            buffer_rucksack_2 = None;
            rucksack_counter = 1;
            last_flag_3 = true;
        }
    }

    if last_flag_3 {
        
        Ok(rucksack_group_vec)
    
    } else {
    
        Err(anyhow!("Last group was incomplete!"))
    }
}

// Day 3 Part 2:
pub fn get_badges_priority() -> Result<i32> {

    let item_types = ItemTypePriorities::new();

    let rucksack_lines = get_rucksack_lines()
    .context("getting total rucksack priority sum")?;

    let rucksack_groups_vec = get_elf_rucksack_groups(&rucksack_lines)
    .context("getting total rucksack priority sum")?;

    let priorities_vec = item_types.get_badges_priorities(&rucksack_groups_vec);

    Ok(priorities_vec.iter().sum())
}