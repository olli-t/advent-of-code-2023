advent_of_code::solution!(3);

use std::collections::HashMap;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut symbol_matrix: Vec<Vec<bool>> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for (i, line) in input.lines().enumerate() {
        symbol_matrix.push(Vec::new());
        for character in line.chars() {
            match character {
                '0'..='9' | '.' => symbol_matrix[i].push(false),
                _ => symbol_matrix[i].push(true),
            }
        }
    }
    let dim_y: usize = symbol_matrix.len();
    let dim_x: usize = symbol_matrix[0].len();

    for (y, line) in input.lines().enumerate() {
        for word in re.find_iter(line) {
            let y_start = get_ok_index(y, 'y', dim_y, dim_x, '-').unwrap();
            let x_start = get_ok_index(word.start(), 'x', dim_y, dim_x, '-').unwrap();
            let y_end = get_ok_index(y, 'y', dim_y, dim_x, '+').unwrap();
            let x_end = get_ok_index(word.end() - 1, 'x', dim_y, dim_x, '+').unwrap();
            let symbol: bool = symbol_matrix[y_start..=y_end].iter().map(|v| v[x_start..=x_end].iter().any(|s| *s)).any(|s| s);
            if symbol {
                sum += word.as_str().parse::<u32>().unwrap();
            }
        }
    }


    Some(sum)
}

fn get_ok_index(index: usize, direction: char, dim_y: usize, dim_x: usize, operator: char) -> Result<usize, ()> {
    let mut max: usize = 0;
    match direction {
        'y' => { max = dim_y - 1 }
        'x' => { max = dim_x - 1 }
        _ => return Err(()),
    }
    let return_index = match operator {
        '+' => {
            if index == max {
                index
            } else {
                index + 1
            }
        }
        '-' => {
            match index {
                0 => 0,
                _ => index - 1
            }
        }
        _ => return Err(())
    };


    Ok(return_index)
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct FindPos {
    pub x: usize,
    pub y: usize,
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut findings: HashMap<FindPos, Vec<u32>> = HashMap::new();
    let mut symbol_matrix: Vec<Vec<bool>> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for (i, line) in input.lines().enumerate() {
        symbol_matrix.push(Vec::new());
        for character in line.chars() {
            match character {
                '*' => symbol_matrix[i].push(true),
                _ => symbol_matrix[i].push(false),
            }
        }
    }
    let dim_y: usize = symbol_matrix.len();
    let dim_x: usize = symbol_matrix[0].len();

    for (y, line) in input.lines().enumerate() {
        for word in re.find_iter(line) {
            let y_start = get_ok_index(y, 'y', dim_y, dim_x, '-').unwrap();
            let x_start = get_ok_index(word.start(), 'x', dim_y, dim_x, '-').unwrap();
            let y_end = get_ok_index(y, 'y', dim_y, dim_x, '+').unwrap();
            let x_end = get_ok_index(word.end() - 1, 'x', dim_y, dim_x, '+').unwrap();
            for (i, v) in symbol_matrix[y_start..=y_end].iter().enumerate(){
                for (j, symbol) in v[x_start..=x_end].iter().enumerate() {
                    if *symbol {
                        let entry = findings.entry(FindPos { y: y_start+i, x: x_start + j });
                        entry.or_default().push(word.as_str().parse().unwrap())
                    }
                }
            }
        }
    }
    let mut sum: u32 = 0;
    for (_, numbers) in findings {
        if numbers.len() ==2 {
            sum += numbers.iter().product::<u32>();
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
