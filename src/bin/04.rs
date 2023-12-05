use std::collections::{HashMap, HashSet};
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut winning: HashSet<u32> = HashSet::new();
        let mut having: HashSet<u32> = HashSet::new();
        let relevant_part: &str = line.split(':').collect::<Vec<_>>()[1];
        let both_sides = relevant_part.split('|').collect::<Vec<_>>();
        let winning_side = both_sides[0];
        let having_side = both_sides[1];
        for number in winning_side.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()) {
            winning.insert(number);
        }
        for number in having_side.split_ascii_whitespace().map(|n| n.parse().unwrap()) {
            having.insert(number);
        }
        let winning_numbers: Vec<_> = winning.intersection(&having).collect();
        if !winning_numbers.is_empty()  { sum += 2_u32.pow((winning_numbers.len() as u32) - 1); }
    }


    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards_with_winning_numbers: HashMap<usize, usize> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let mut winning: HashSet<u32> = HashSet::new();
        let mut having: HashSet<u32> = HashSet::new();
        let relevant_part: &str = line.split(':').collect::<Vec<_>>()[1];
        let both_sides = relevant_part.split('|').collect::<Vec<_>>();
        let winning_side = both_sides[0];
        let having_side = both_sides[1];
        for number in winning_side.split_ascii_whitespace().map(|n| n.parse::<u32>().unwrap()) {
            winning.insert(number);
        }
        for number in having_side.split_ascii_whitespace().map(|n| n.parse().unwrap()) {
            having.insert(number);
        }
        let winning_numbers: Vec<_> = winning.intersection(&having).collect();
        cards_with_winning_numbers.insert(i + 1, winning_numbers.len());
    }
    let mut number_of_cards: HashMap<usize, usize> = cards_with_winning_numbers.keys().map(|c| (*c, 1_usize)).collect();

    for k in 1..=cards_with_winning_numbers.len() {
        let v = *cards_with_winning_numbers.get(&k).unwrap();
        let n = *number_of_cards.get(&k).unwrap();
        for x in k + 1..=k + v  {
            if x <= cards_with_winning_numbers.len() {
                number_of_cards.insert(x, number_of_cards.get(&x).unwrap() + n);
            } else { break }
        }
    }
    Some(number_of_cards.values().map(|n|*n as u32).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_2", DAY));
        assert_eq!(result, Some(30));
    }
}
