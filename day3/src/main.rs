use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const SYMBOLS: [char; 10] = ['&', '/', '%', '=', '@', '*', '#', '$', '+', '-'];

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let row: Vec<char> = line?.chars().collect();

        matrix.push(row);
    }

    let mut surrounded_nums_idxs: HashSet<Vec<usize>> = HashSet::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let mut has_surrounding_symbol = false;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let new_i = i as isize + di;
                    let new_j = j as isize + dj;

                    if new_i >= 0
                        && new_i < matrix.len() as isize
                        && new_j >= 0
                        && new_j < matrix[0].len() as isize
                        && SYMBOLS.contains(&matrix[new_i as usize][new_j as usize])
                    {
                        has_surrounding_symbol = true;
                    }
                }
            }

            if has_surrounding_symbol && matrix[i][j].is_numeric() {
                surrounded_nums_idxs.insert(vec![i, j]);
            }
        }
    }

    let mut sum = 0;

    for i in 0..matrix.len() {
        let mut j = 0;

        while j < matrix[i].len() {
            let mut has_surrounding_symbol = false;
            let mut curr_num = String::new();

            while j < matrix[i].len() && matrix[i][j].is_numeric() {
                if surrounded_nums_idxs.contains(&vec![i, j]) {
                    has_surrounding_symbol = true;
                }
                curr_num.push(matrix[i][j]);
                j += 1;
            }

            if !curr_num.is_empty() {
                if let Ok(num) = curr_num.parse::<i32>() {
                    if has_surrounding_symbol {
                        sum += num;
                    }
                }
            }

            j += 1;
        }
    }

    println!("{sum}");

    return Ok(());
}
