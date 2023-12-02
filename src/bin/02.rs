use std::collections::HashMap;
use regex::Regex;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut id_sum = 0;
    let mut maximum = HashMap::new();
    maximum.insert("red", 12);
    maximum.insert("green", 13);
    maximum.insert("blue", 14);
    let game_id_re = Regex::new(r"^Game (\d+):.*").unwrap();
    for line in input.lines() {
        let mut game_id = 0;
        for (_, [c1]) in game_id_re.captures_iter(line).map(|c| c.extract()) {
            game_id = c1.parse().unwrap();
        }
        let mut impossible = false;
        for colors in line.split(':').collect::<Vec<_>>()[1].split(';') {
            for color in colors.split(',') {
                let color_info: Vec<&str> = color.split_ascii_whitespace().collect();
                let number: i32 = color_info[0].parse().unwrap();
                let color = color_info[1];
                if number > *maximum.get(color).unwrap() {
                    impossible = true;
                    break;
                }
            }
            if impossible { break; }
        }
        if !impossible {
            id_sum += game_id
        }
    }
    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut power_sum = 0;
    for line in input.lines() {
        let mut minimum: HashMap<&str, u32> = HashMap::new();
        for round in line.split(':').collect::<Vec<_>>()[1].split(';') {
            for color in round.split(',') {
                let color_info: Vec<&str> = color.split_ascii_whitespace().collect();
                let number: u32 = color_info[0].parse().unwrap();
                let color = color_info[1];
                match minimum.get(color) {
                    Some(n) => {
                        if n < &number { minimum.insert(color, number);}
                    },
                    _ => {minimum.insert(color, number);},
                }
            }
        }
        power_sum += minimum.values().product::<u32>();
    }

    Some(power_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
