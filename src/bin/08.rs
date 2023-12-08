use regex::Regex;
use std::collections::HashMap;
advent_of_code::solution!(8);
use num::integer::lcm;
pub struct Route {
    l: String,
    r: String,
    circular: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lookup: HashMap<&str, Route> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    let directions: Vec<char> = lines[0].chars().collect();
    let re_address = Regex::new(r"\w{3}").unwrap();
    for line in &lines[2..] {
        let route: Vec<&str> = re_address.find_iter(line).map(|m| m.as_str()).collect();

        lookup.insert(
            route[0],
            Route {
                l: route[1].to_string(),
                r: route[2].to_string(),
                circular: { route[0] == route[1] && route[1] == route[2] && route[0] == route[2] },
            },
        );
    }

    let mut steps: u32 = 0;
    let mut next: &str = "AAA";
    while next != "ZZZ" {
        for ch in &directions {
            next = match ch {
                'L' => &lookup[next].l,
                'R' => &lookup[next].r,
                _ => {
                    panic!()
                }
            };

            steps += 1;
            if next == "ZZZ" {
                break;
            }
            if lookup[next].circular {
                panic!()
            }
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lookup: HashMap<&str, Route> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    let directions: Vec<char> = lines[0].chars().collect();
    let re_address = Regex::new(r"\w{3}").unwrap();
    let mut starting_nodes: Vec<&str> = Vec::new();
    for line in &lines[2..] {
        let route: Vec<&str> = re_address.find_iter(line).map(|m| m.as_str()).collect();

        lookup.insert(
            route[0],
            Route {
                l: route[1].to_string(),
                r: route[2].to_string(),
                circular: { route[0] == route[1] && route[1] == route[2] && route[0] == route[2] },
            },
        );
        if route[0].ends_with('A') {
            starting_nodes.push(route[0])
        }
    }

    let mut steps: u64 = 0;
    let mut looparounds: Vec<Option<u64>> = starting_nodes.iter().map(|_| None).collect();
    let mut loops: Vec<Option<u64>> = looparounds.clone();
    'outer: loop {
        for ch in &directions {
            starting_nodes = starting_nodes
                .iter()
                .map(|n| {
                    (match ch {
                        'L' => &lookup[n].l,
                        'R' => &lookup[n].r,
                        _ => {
                            panic!()
                        }
                    })
                    .as_str()
                })
                .collect();

            steps += 1;
            for (i, node) in starting_nodes.iter().enumerate() {
                if node.ends_with('Z') {
                    if looparounds[i].is_none() {
                        looparounds[i] = Some(steps);
                    } else {
                        if loops[i].is_none() {
                            loops[i] = Some(steps - looparounds[i].unwrap());
                            if loops.iter().all(|e| e.is_some()) {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{looparounds:?}");

    let mut lcm_calculated: u64 = 1;
    for loop_ in loops {
        lcm_calculated = lcm(lcm_calculated, loop_.unwrap());
    }
    Some(lcm_calculated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_2", DAY));
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(21003205388413));
    }
}
