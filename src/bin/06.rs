use regex::Regex;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<u64> = re.find_iter(lines[0]).map(|n| n.as_str().parse().unwrap()).into_iter().collect();
    let distances: Vec<u64> = re.find_iter(lines[1]).map(|n| n.as_str().parse().unwrap()).into_iter().collect();

    return get_possible_wins(times, distances)
}

fn get_possible_wins(times: Vec<u64>, distances: Vec<u64>) -> Option<u64> {
    let mut possible_wins: u64 = 1;
    for (i, time) in times.iter().enumerate() {
        let distance = distances[i] as f64;
        let time = -1.0 * *time as f64;
        let left_term: f64 = -(time / 2.0);
        let right_term: f64 = ((time / 2.0).powi(2) - distance).sqrt();
        let left_border = (left_term - right_term).floor();
        let right_border = (left_term + right_term).ceil() - 1.0;
        possible_wins *= (left_border as i64 - right_border as i64).abs() as u64;
    }

    Some(possible_wins)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<u64> = re.find_iter(&*lines[0].replace(' ', "")).map(|n| n.as_str().parse().unwrap()).into_iter().collect();
    let distances: Vec<u64> = re.find_iter(&*lines[1].replace(' ', "")).map(|n| n.as_str().parse().unwrap()).into_iter().collect();

    return get_possible_wins(times, distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
