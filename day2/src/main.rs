use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut id_sums: usize = 0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Game ") {
            let parts: Vec<&str> = line.split(":").collect();

            let game = parts[0].trim();
            let scores = parts[1].trim();

            id_sums += is_game_possible(game, scores);
        }    
    }

    println!("{}", id_sums);
    return Ok(());
}

fn is_game_possible(game: &str, scores: &str) -> usize {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let game_num: usize = game
        .split(" ")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse()
        .unwrap();
    let scores: Vec<&str> = scores.split(";").collect(); 
    for single_score in scores {
        let single_score =single_score.trim();
        let single_score: Vec<&str> =single_score.split(",").collect();
        
        for ballz in single_score {
            let ballz: Vec<&str> = ballz
                .split(" ")
                .filter(|&x| !x.is_empty())
                .collect();
            match ballz[1].trim() {
                "red" => {
                    let num_of_ballz: usize = ballz[0].parse().unwrap();
                    if num_of_ballz > max_red {
                        return 0;
                    }
                },
                "green" => {
                    let num_of_ballz: usize = ballz[0].parse().unwrap();
                    if num_of_ballz > max_green {
                        return 0;
                    }
                },
                "blue" => {
                     let num_of_ballz: usize = ballz[0].parse().unwrap();
                    if num_of_ballz > max_blue {
                        return 0;
                    }
                }
                _ => return 0
            }
        }
    }

    return game_num;
}

// You play several games and record the information from each game 
// (your puzzle input). 
// Each game is listed with its ID number 
// (like the 11 in Game 11: ...) 
// followed by a semicolon-separated list of subsets 
// of cubes that were revealed from the bag 
// (like 3 red, 5 green, 4 blue).
//
// Game #: subset; subset; subset
//
// Red: max 12
// Green: max 13
// Blue: max 14
// Game number, all subsets
// From each subset number of ballz
// skip
// else 
// add ID of the game to sum
