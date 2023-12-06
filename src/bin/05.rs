advent_of_code::solution!(5);

pub struct Transform {
    start: u64,
    length: u64,
    destination: u64,
}

impl Transform {
    pub fn transform(&self, v: &u64) -> Result<u64, ()> {
        if (self.start..self.start + self.length).contains(&v) {
            Ok((*v) + (self.destination) - (self.start))
        } else {
            Err(())
        }
    }
}

pub struct Map {
    transformations: Vec<Transform>,
}

impl Map {
    pub fn apply_on_seeds(&self, seeds: Vec<u64>) -> Vec<u64> {
        let mut new_seeds: Vec<u64> = Vec::new();
        let mut new_seed: Option<u64> = None;
        for seed in seeds {
            for transformation in &self.transformations {
                match transformation.transform(&seed) {
                    Ok(x) => {
                        new_seed = Some(x);
                        break;
                    }
                    Err(_) => { new_seed = None }
                }
            }
            match new_seed {
                None => { new_seeds.push(seed) }
                Some(x) => { new_seeds.push(x) }
            }
        }
        new_seeds
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds") {
            for seed in line.strip_prefix("seeds: ").unwrap().split_ascii_whitespace() {
                seeds.push(seed.parse().unwrap())
            }
        } else if line.ends_with("map:") {
            maps.push(Map { transformations: Vec::new() })
        } else if line.is_empty() {} else {
            let transform_line: Vec<u64> = line.split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
            maps.last_mut().unwrap().transformations.push(Transform {
                start: transform_line[1],
                length: transform_line[2],
                destination: transform_line[0],
            });
        }
    }
    for map in maps {
        seeds = map.apply_on_seeds(seeds);
    }


    Some(*seeds.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut seed_ranges: Vec<u64> = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds") {
            for seed in line.strip_prefix("seeds: ").unwrap().split_ascii_whitespace() {
                seed_ranges.push(seed.parse().unwrap())
            }
        } else if line.ends_with("map:") {
            maps.push(Map { transformations: Vec::new() })
        } else if line.is_empty() {} else {
            let transform_line: Vec<u64> = line.split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
            maps.last_mut().unwrap().transformations.push(Transform {
                start: transform_line[1],
                length: transform_line[2],
                destination: transform_line[0],
            });
        }
    }
    let mut seeds = Vec::new();
    for i in (0..seed_ranges.len()).step_by(2) {
        for seed in seed_ranges[i]..seed_ranges[i] + seed_ranges[i + 1] {
            seeds.push(seed)
        }
    }

    for map in maps {
        seeds = map.apply_on_seeds(seeds);
    }


    Some(*seeds.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
