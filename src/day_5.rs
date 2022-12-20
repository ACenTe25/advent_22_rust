// Standard library
use std::collections::HashMap;

// External crates
use anyhow::{Context, Result, anyhow};

// Crate modules
use crate::general::*;


/*--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded 
from the ships. Supplies are stored in stacks of marked crates, but because 
the needed supplies are buried under many other crates, the crates need to 
be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. 
To ensure none of the crates get crushed or fall over, the crane operator 
will rearrange them in a series of carefully-planned steps. After the 
crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate 
procedure, but they forgot to ask her which crate will end up where, and 
they want to be ready to unload them as soon as possible so they can 
embark.

They do, however, have a drawing of the starting stacks of crates and the 
rearrangement procedure (your puzzle input). For example:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: 
crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; 
from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a 
single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a 
quantity of crates is moved from one stack to a different stack. In the first 
step of the above rearrangement procedure, one crate is moved from stack 2 to 
stack 1, resulting in this configuration:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 

In the second step, three crates are moved from stack 1 to stack 3. Crates are 
moved one at a time, so the first crate to be moved (D) ends up below the second 
and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1. Again, because crates are 
moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack; in this 
example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you 
should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each 
stack?
 */

struct Stack {
    num_stacks: i32,
    stack_tags: Vec<String>,
    stack_map: HashMap<String, Vec<char>>
}

impl Stack {

    fn new() -> Result<Self> {

        let lines = get_input_lines("/home/acente/Advent_inputs/day_5_input_1.txt")
        .context("reading stack drawing")?;

        let (num_stacks, stack_tags) = find_stacks(&lines)
        .context("reading stack drawing")?;

        let box_lines = get_box_lines(&lines)
        .context("reading stack drawing")?;

        let box_rows = get_box_rows(&box_lines, num_stacks)
        .context("reading stack drawing")?;

        let box_stacks = get_box_stacks(&box_rows, num_stacks)
        .context("reading stack drawing")?;

        let mut stack_map: HashMap<String, Vec<char>> = HashMap::new();

        for stack in 0..num_stacks as usize {
            stack_map.insert(stack_tags[stack].clone(), box_stacks[stack].clone());
        }

        Ok(
            Self {
                num_stacks,
                stack_tags,
                stack_map
            }
        )
    }

    fn move_boxes(&mut self, move_vec: &[Movement]) -> Result<String> {

        for movement in move_vec {

            let mut all_moved_boxes: Vec<char> = Vec::new();

            let Some(origin_stack) = self.stack_map.get_mut(&movement.origin_stack) else {
                return Err(anyhow!("Stack {} not found in stack map.", movement.origin_stack))
            };

            for _ in 0..movement.number_of_crates {
 
                match origin_stack.pop() {
                    Some(box_tag) => all_moved_boxes.push(box_tag),
                    _ => break,
                };

            }

            match self.stack_map.get_mut(&movement.destination_stack) {
                Some(destination_stack) => {

                    for moved_box in 0..all_moved_boxes.len() {

                        destination_stack.push(all_moved_boxes[moved_box]);
                    }
                }
                None => {

                    let Some(origin_stack) = self.stack_map.get_mut(&movement.origin_stack) else {
                        return Err(anyhow!("Stack {} not found in stack map.", movement.origin_stack))
                    };

                    all_moved_boxes.reverse();

                    for moved_box in 0..all_moved_boxes.len() {

                        origin_stack.push(all_moved_boxes[moved_box]);

                    }

                    return Err(anyhow!("Stack {} not found in stack map.", movement.destination_stack))
                }
            }
        }

        Ok(self.top_boxes())
    }

    // Part 2:
    fn move_boxes_with_cratemover_9001(&mut self, move_vec: &[Movement]) -> Result<String> {

        for movement in move_vec {

            let mut all_moved_boxes: Vec<char> = Vec::new();

            let Some(origin_stack) = self.stack_map.get_mut(&movement.origin_stack) else {
                return Err(anyhow!("Stack {} not found in stack map.", movement.origin_stack))
            };

            for _ in 0..movement.number_of_crates {
 
                match origin_stack.pop() {
                    Some(box_tag) => all_moved_boxes.insert(0, box_tag),
                    _ => break,
                };

            }

            match self.stack_map.get_mut(&movement.destination_stack) {
                Some(destination_stack) => {

                    for moved_box in 0..all_moved_boxes.len() {

                        destination_stack.push(all_moved_boxes[moved_box]);
                    }
                }
                None => {

                    let Some(origin_stack) = self.stack_map.get_mut(&movement.origin_stack) else {
                        return Err(anyhow!("Stack {} not found in stack map.", movement.origin_stack))
                    };

                    for moved_box in 0..all_moved_boxes.len() {

                        origin_stack.push(all_moved_boxes[moved_box]);

                    }

                    return Err(anyhow!("Stack {} not found in stack map.", movement.destination_stack))
                }
            }
        }

        Ok(self.top_boxes())
    }

    fn top_boxes(&self) -> String {
        
        let mut top_boxes = String::new();

        for stack in &self.stack_tags {

            match self.stack_map.get(stack) {
                
                Some(stack_boxes) => match &stack_boxes.last() {
                
                    Some(last_box_tag) => top_boxes.push(**last_box_tag),
                
                    None => continue,
                },
                
                None => continue,
            }
        }

        top_boxes
    }
}

struct Movement {
    number_of_crates: i32,
    origin_stack: String,
    destination_stack: String
}

impl Movement {

    fn new(move_line: &str) -> Result<Self> {
        
        let move_line_parts: Vec<&str> = move_line.split_whitespace().collect();

        if move_line_parts[0] == "move" && move_line_parts[2] == "from" && move_line_parts[4] == "to" {

            let number_of_crates = move_line_parts[1].parse::<i32>()
            .context("decoding move line")?;

            let origin_stack = move_line_parts[3].to_string();

            let destination_stack = move_line_parts[5].to_string();

            Ok(
                Movement {
                    number_of_crates,
                    origin_stack,
                    destination_stack
                }
            )

        } else {

            Err(anyhow!("Unknown move line format: '{}'", move_line))
        }
    }
}

fn find_stacks(lines: &Vec<String>) -> Result<(i32, Vec<String>)> {

    let mut stack_tags_opt: Option<Vec<String>> = None;
    
    for line in lines {

        if line.contains(" 1   2  ") {

            stack_tags_opt = Some(
                line
                .split_whitespace()
                .map(|tag| tag.to_string())
                .collect()
            );

        }
    }

    let Some(stack_tags) = stack_tags_opt else {

        return Err(anyhow!("Stack tags line not found in ' 1   2 ...' format."))

    };

    Ok((stack_tags.len() as i32, stack_tags))
}

fn get_box_lines(lines: &Vec<String>) -> Result<Vec<String>> {

    let mut box_lines: Vec<String> = Vec::new();

    for line in lines {
        
        if line.contains("[") {

            box_lines.push(line.clone());

        }
    }

    Ok(box_lines)
}

fn get_box_rows (box_lines: &[String], num_stacks: i32) -> Result<Vec<Vec<char>>> {
    
    let mut box_rows: Vec<Vec<char>> = Vec::new();

    for line in box_lines {

        let raw_tag_row: Vec<char> = line.chars().collect();

        let mut tag_row: Vec<char> = Vec::new();

        for stack in 1..=num_stacks as usize {

            let box_tag_index = (stack - 1) * 4 + 1;

            tag_row.push(raw_tag_row[box_tag_index]);

        }

        box_rows.push(tag_row);

    }

    Ok(box_rows)
}

fn get_box_stacks(box_rows: &Vec<Vec<char>>, num_stacks: i32) -> Result<Vec<Vec<char>>> {
    
    let mut box_stacks: Vec<Vec<char>> = Vec::new();

    for stack in 1..=num_stacks as usize {

        let mut stack_tags: Vec<char> = Vec::new();

        for box_row in box_rows {

            if box_row[stack - 1] == ' ' {
                
                continue

            }

            stack_tags.insert(0, box_row[stack - 1]);
        }

        box_stacks.push(stack_tags);
    }

    Ok(box_stacks)
}

fn get_move_lines() -> Result<Vec<String>> {
    
    // Get all lines
    let lines = get_input_lines("/home/acente/Advent_inputs/day_5_input_1.txt")
    .context("reading stack drawing")?;

    let mut move_lines: Vec<String> = Vec::new();

    for line in lines {

        if line.contains("move") {
            move_lines.push(line.clone());
        }
    }

    Ok(move_lines)
}

fn get_move_vector(move_lines: &Vec<String>) -> Result<Vec<Movement>> {

    let mut move_vector: Vec<Movement> = Vec::new();

    for move_line in move_lines {

        move_vector.push(Movement::new(move_line).context("creating move vector")?);
    }

    Ok(move_vector)
}

// Part 1:
pub fn get_stack_tops() -> Result<String> {

    let mut stack = Stack::new()
    .context("moving boxes")?;

    let move_lines = get_move_lines()
    .context("moving boxes")?;

    let moves_vec = get_move_vector(&move_lines)
    .context("moving boxes")?;

    stack.move_boxes(&moves_vec)

}

/*--- Part Two ---

As you watch the crane operator expertly rearrange the crates, you notice the 
process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe 
it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air 
conditioning, leather seats, an extra cup holder, and the ability to pick up and 
move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 

However, the action of moving three crates from stack 1 to stack 3 means that 
those three moved crates stay in the same order, resulting in this new 
configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3

Next, as both crates are moved from stack 2 to stack 1, they retain their order 
as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3

Finally, a single crate is still moved from stack 1 to stack 2, but now it's 
crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3

In this example, the CrateMover 9001 has put the crates in a totally different 
order: MCD.

Before the rearrangement process finishes, update your simulation so that the 
Elves know where they should stand to be ready to unload the final supplies. 
After the rearrangement procedure completes, what crate ends up on top of each 
stack?
 */

// Part 2:
pub fn get_stack_tops_with_cratemover_9001() -> Result<String> {

    let mut stack = Stack::new()
    .context("moving boxes")?;

    let move_lines = get_move_lines()
    .context("moving boxes")?;

    let moves_vec = get_move_vector(&move_lines)
    .context("moving boxes")?;

    stack.move_boxes_with_cratemover_9001(&moves_vec)

}