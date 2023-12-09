use num::{Integer, Zero};
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut starting_line: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut end_elements: Vec<i32> = vec![];
        end_elements.push(starting_line[starting_line.len() - 1]);
        loop {
            starting_line = starting_line.windows(2).map(|n| n[1] - n[0]).collect();
            if starting_line.iter().all(|n| n.is_zero()) {
                break;
            }
            end_elements.push(starting_line[starting_line.len() - 1]);
        }
        sum += end_elements.iter().sum::<i32>()
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut starting_line: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let mut start_elements: Vec<i32> = vec![];
        start_elements.push(starting_line[0]);
        loop {
            starting_line = starting_line.windows(2).map(|n| n[1] - n[0]).collect();
            if starting_line.iter().all(|n| n.is_zero()) {
                break;
            }
            start_elements.push(starting_line[0]);
        }
        sum += start_elements
            .iter()
            .enumerate()
            .map(|(i, n)| {
                n * {
                    if i.is_odd() {
                        -1
                    } else {
                        1
                    }
                }
            })
            .sum::<i32>();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
