// Standard library


// External crates
use anyhow::{Context, Result, anyhow};

// Crate modules
use crate::general::*;

// Function 1
// As I learned in day 1, I should really do small functions in order to reuse them
// in case conditions change.

/*--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to 
the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in 
each round, the players each simultaneously choose one of Rock, Paper, or Scissors using 
a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors 
defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round 
instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your 
    puzzle input) that they say will be sure to help you win. "The first column is what 
    your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second 
    column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for 
Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have 
been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is 
the sum of your scores for each round. The score for a single round is the score for the 
shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the 
outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate 
the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). 
    This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you 
        won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). 
    This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 
    3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 
(8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
 */

#[derive(Debug, Clone, Copy)]
pub enum RPSMove {
    Rock,
    Paper,
    Scissors
}


impl RPSMove {

    fn decrypt_from_char(encrypted_move: char) -> Result<Self> {
        
        match encrypted_move {
        
            'A' | 'X' => Ok(RPSMove::Rock),
        
            'B' | 'Y' => Ok(RPSMove::Paper),
        
            'C' | 'Z' => Ok(RPSMove::Scissors),
        
            _ => Err(anyhow!("Unknown encryption character: '{}'", encrypted_move)),
        }
    }

    fn as_score(&self) -> i32 {
        
        match self {
            
            Self::Rock => 1,
            
            Self::Paper => 2,
            
            Self::Scissors => 3,
        }
    }

    fn play(&self, opponent_move: Self) -> RPSResult {
        match self {

            RPSMove::Rock => {
                match opponent_move {
                    RPSMove::Rock => RPSResult::Draw(self.as_score() + 3),
                    RPSMove::Paper => RPSResult::SelfLoss(self.as_score()),
                    RPSMove::Scissors => RPSResult::SelfWin(self.as_score() + 6),
                }
            }
            
            RPSMove::Paper => {
                match opponent_move {
                    RPSMove::Rock => RPSResult::SelfWin(self.as_score() + 6),
                    RPSMove::Paper => RPSResult::Draw(self.as_score() + 3),
                    RPSMove::Scissors => RPSResult::SelfLoss(self.as_score()),
                }
            }
            
            RPSMove::Scissors => {
                match opponent_move {
                    RPSMove::Rock => RPSResult::SelfLoss(self.as_score()),
                    RPSMove::Paper => RPSResult::SelfWin(self.as_score() + 6),
                    RPSMove::Scissors => RPSResult::Draw(self.as_score() + 3),
                }
            }
        }
    }
}


pub enum RPSResult {
    SelfWin(i32),
    SelfLoss(i32),
    Draw(i32)
}

impl RPSResult {
    fn get_score(&self) -> i32 {
        match self {
            Self::SelfWin(num) | 
            Self::SelfLoss(num) | 
            Self::Draw(num) => *num
        }
    }
}

pub struct RPSRound {
    opponent_move: RPSMove,
    self_move: RPSMove,
    result: RPSResult
}


impl RPSRound {
    fn new(line: &str) -> Result<Self> {
        
        // Get the characters
        let mut line_chars = line.chars();

        // Get the opponent's move
        let Some(opponent_char) = line_chars.next() else {
            return Err(anyhow!("No characters found in round"))
        };

        let opponent_move = RPSMove::decrypt_from_char(opponent_char)
        .context("decrypting new round")?;

        // Skip the space
        line_chars.next();
        
        // Get the self move
        let Some(self_char) = line_chars.next() else {
            return Err(anyhow!("Only one character found in round"))
        };

        let self_move = RPSMove::decrypt_from_char(self_char)
        .context("decrypting new round")?;

        // Get the result
        let result = self_move.play(opponent_move);
        
        Ok(RPSRound {
            opponent_move,
            self_move,
            result
        })
    }
}

fn get_encrypted_rps_rounds() -> Result<Vec<String>> {

    // Read the input file
    let input_string = read_text_input("/home/acente/Advent_inputs/day_2_input_1.txt")
    .context("getting encrypted strategy rounds for RPS")?;

    // Turn lines into vector of Strings
    let mut input_lines: Vec<String> = Vec::new();

    for line in input_string.lines() {
        input_lines.push(line.to_string());
    }

    Ok(input_lines)

}

fn decrypt_rps_rounds(input_lines: &Vec<String>) -> Result<Vec<RPSRound>> {

    let mut decrypted_rounds: Vec<RPSRound> = Vec::new();

    for line in input_lines {
        decrypted_rounds.push(RPSRound::new(line).context("decrypting RPS rounds")?);
    }

    Ok(decrypted_rounds)
}

fn get_game_score(decrypted_rounds: &[RPSRound]) -> i32 {
    
    let mut game_score = 0;
    
    for round in decrypted_rounds {
        
        game_score += round.result.get_score();

    }

    game_score
}

pub fn simulate_input_strategy_results() -> Result<i32> {

    let input_lines = get_encrypted_rps_rounds()
    .context("simulating input strategy results")?;

    let decrypted_rounds = decrypt_rps_rounds(&input_lines)
    .context("simulating input strategy results")?;

    Ok(get_game_score(&decrypted_rounds))
}