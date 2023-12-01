use std::collections::HashMap;
advent_of_code::solution!(1);


pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        let vec = get_vec_with_numbers(line);

        sum += vec[0] * 10 + vec[vec.len() - 1];
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    let mut numbers = HashMap::new();
    let arr = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i, number_word) in arr.iter().enumerate() {
        numbers.insert(number_word, (i + 1).to_string());
    }

    for line in lines {
        let mut new_line = String::from(line);
        let mut change_str: Vec<(usize, &str)> = Vec::new();
        for number_word in arr {
            for (match_index, _) in new_line.match_indices(number_word) {
                change_str.push((match_index, &numbers[&number_word]));
            }
        }
        change_str.sort_by_key(|k| k.0);
        for (corr, (i, num)) in change_str.iter().enumerate() {
            new_line.insert_str(*i + corr, num);
        }

        let vec = get_vec_with_numbers(&new_line);
        sum += vec[0] * 10 + vec[vec.len() - 1];
    }
    Some(sum)
}

fn get_vec_with_numbers(line: &str) -> Vec<u32> {
    line.chars().flat_map(|s| s.to_digit(10)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_2", DAY));
        assert_eq!(result, Some(281));
    }
}