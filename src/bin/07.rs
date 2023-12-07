use std::collections::HashMap;

advent_of_code::solution!(7);

#[derive(Debug)]
pub struct Hand {
    type_: Option<u32>,
    cards: String,
    overall_strength: Option<u32>,
    bid: u32,
}

impl Hand {
    pub fn calculate_hand(&mut self) {
        let mut cards: HashMap<char, u32> = HashMap::new();
        let mut multiples: [u32; 5] = [0, 0, 0, 0, 0];
        for c in self.cards.chars() {
            let option = cards.get_mut(&c).cloned();
            match option {
                Some(n) => {
                    cards.insert(c, n + 1);
                    multiples[n as usize] += 1;
                    multiples[n as usize - 1] -= 1;
                }
                None => {
                    cards.insert(c, 1);
                    multiples[0] += 1
                }
            }
        }
        self.type_ = Some(match multiples {
            [0, 0, 0, 0, 1] => 6,
            [1, 0, 0, 1, 0] => 5,
            [0, 1, 1, 0, 0] => 4,
            [2, 0, 1, 0, 0] => 3,
            [1, 2, 0, 0, 0] => 2,
            [3, 1, 0, 0, 0] => 1,
            [5, 0, 0, 0, 0] => 0,
            _ => panic!(),
        });
        let mut strength_cards: u32 = 0;
        for (i, char) in self.cards.chars().enumerate() {
            let card_strength: u32 = match char {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                _ => panic!(),
            };
            strength_cards += 16_u32.pow((4 - i) as u32) * card_strength
        }

        self.overall_strength = Some(self.type_.unwrap() * 16_u32.pow(5) + strength_cards)
    }

    pub fn calculate_hand_2(&mut self) {
        let mut cards: HashMap<char, u32> = HashMap::new();
        let mut multiples: [u32; 6] = [0, 0, 0, 0, 0, 0];
        for c in self.cards.chars() {
            let option = cards.get_mut(&c).cloned();
            match option {
                Some(n) => {
                    cards.insert(c, n + 1);
                    if c != 'J' {
                        multiples[n as usize] += 1;
                        multiples[n as usize - 1] -= 1;
                    } else {
                        multiples[5] += 1
                    }
                }
                None => {
                    cards.insert(c, 1);
                    if c != 'J' {
                        multiples[0] += 1
                    } else {
                        multiples[5] += 1
                    }
                }
            }
        }
        self.type_ = Some(match multiples {
            [_, _, _, _, 1, 0] => 6,
            [_, 0, 0, 1, 0, 0] => 5,
            [0, 1, 1, 0, 0, 0] => 4,
            [_, 0, 1, 0, 0, 0] => 3,
            [_, 2, 0, 0, 0, 0] => 2,
            [_, 1, 0, 0, 0, 0] => 1,
            [_, 0, 0, 0, 0, 0] => 0,
            [0, 0, 0, 1, 0, 1] => 6,
            [_, _, 1, _, _, n] => 4 + n,
            [_, 2, 0, 0, 0, 1] => 4,
            [_, 1, _, _, _, 1] => 3,
            [_, 1, _, _, _, 2] => 5,
            [_, 1, _, _, _, 3] => 6,
            [_, 0, 0, 0, 0, n] => match 1 + n {
                2 => 1,
                3 => 3,
                4 => 5,
                5 => 6,
                6 => 6,
                _ => {
                    println!("{self:?}");
                    panic!();
                }
            },

            _ => {
                println!("{self:?}");
                panic!();
            }
        });

        let mut strength_cards: u32 = 0;
        for (i, char) in self.cards.chars().enumerate() {
            let card_strength: u32 = match char {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 0,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                _ => panic!(),
            };
            strength_cards += 16_u32.pow((4 - i) as u32) * card_strength
        }

        self.overall_strength = Some(self.type_.unwrap() * 16_u32.pow(5) + strength_cards)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut all_hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let whitespace: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut new_hand = Hand {
            type_: None,
            cards: (whitespace[0]).parse().unwrap(),
            overall_strength: None,
            bid: (whitespace[1].parse().unwrap()),
        };
        new_hand.calculate_hand();
        all_hands.push(new_hand)
    }

    all_hands.sort_by_key(|h| h.overall_strength);
    Some(
        all_hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut all_hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let whitespace: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut new_hand = Hand {
            type_: None,
            cards: (whitespace[0]).parse().unwrap(),
            overall_strength: None,
            bid: (whitespace[1].parse().unwrap()),
        };
        new_hand.calculate_hand_2();
        all_hands.push(new_hand)
    }

    all_hands.sort_by_key(|h| h.overall_strength);
    Some(
        all_hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i as u32 + 1) * h.bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }
    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(256448566));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(254412181));
    }
}
