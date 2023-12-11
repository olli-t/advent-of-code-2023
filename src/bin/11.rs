advent_of_code::solution!(11);

pub struct Galaxy {
    y: usize,
    x: usize,
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut only_points = true;
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        let mut current_line: Vec<char> = vec![];

        for char in line.chars() {
            match char {
                '#' => only_points = false,
                '.' => {}
                _ => panic!(),
            }
            current_line.push(char)
        }
        matrix.push(current_line.clone());
        if only_points {
            matrix.push(vec!['.'; current_line.len()])
        }
        only_points = true
    }
    let mut inserts: Vec<usize> = vec![];
    for column in 0..matrix[0].len() {
        for row in &matrix {
            if row[column] != '.' {
                only_points = false
            }
        }
        if only_points {
            inserts.push(column)
        }
        only_points = true
    }
    for (i, insert) in inserts.iter().enumerate() {
        for line in &mut matrix {
            line.insert(i + insert, '.');
        }
    }

    let mut galaxies: Vec<Galaxy> = vec![];

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if matrix[y][x] == '#' {
                galaxies.push(Galaxy { y, x })
            }
        }
    }
    let mut sum_: u32 = 0;
    loop {
        let galaxy = galaxies.pop().unwrap();
        for galaxy_2 in &galaxies {
            sum_ += (((galaxy.y as i32) - (galaxy_2.y as i32)).abs()
                + ((galaxy.x as i32) - (galaxy_2.x as i32)).abs()) as u32
        }
        if galaxies.is_empty() {
            break;
        }
    }

    Some(sum_)
}

fn get_in_range(a: &usize, b: &usize, elems: &[usize]) -> u32 {
    let lowerbound: usize;
    let upperbound: usize;
    if a > b {
        lowerbound = *b;
        upperbound = *a;
    } else {
        lowerbound = *a;
        upperbound = *b;
    }
    elems
        .iter()
        .filter(|d| (lowerbound..upperbound).contains(d))
        .count() as u32
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_parameterized(input, 1000000)
}

fn part_two_parameterized(input: &str, expansion: u32) -> Option<u64> {
    let mut only_points = true;
    let mut expand_columns: Vec<usize> = vec![];
    let mut expand_rows: Vec<usize> = vec![];
    let mut galaxies: Vec<Galaxy> = vec![];
    let lines: Vec<&str> = input.lines().collect();

    for (y, line) in lines.iter().enumerate() {
        for (x, char_) in line.chars().enumerate() {
            match char_ {
                '#' => {
                    only_points = false;
                    galaxies.push(Galaxy { y, x })
                }
                '.' => {}
                _ => panic!(),
            }
        }
        if only_points {
            expand_rows.push(y)
        }
        only_points = true
    }
    for column in 0..lines[0].len() {
        for row in &lines {
            if row.chars().collect::<Vec<char>>()[column] != '.' {
                only_points = false
            }
        }
        if only_points {
            expand_columns.push(column)
        }
        only_points = true
    }
    println!("{:?}", &expand_columns);

    println!("{:?}", &expand_rows);
    let mut sum_: u64 = 0;
    loop {
        let galaxy = galaxies.pop().unwrap();
        for galaxy_2 in &galaxies {
            let add_distance_x: u64 =
                (get_in_range(&galaxy.x, &galaxy_2.x, &expand_columns.clone()) * (expansion - 1))
                    as u64;
            let add_distance_y: u64 = (get_in_range(&galaxy.y, &galaxy_2.y, &expand_rows.clone())
                * (expansion - 1)) as u64;
            sum_ += (((galaxy.y as i32) - (galaxy_2.y as i32)).abs()
                + ((galaxy.x as i32) - (galaxy_2.x as i32)).abs()) as u64
                + add_distance_x
                + add_distance_y
        }
        if galaxies.is_empty() {
            break;
        }
    }

    Some(sum_)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two_parameterized(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(8410));
        let result =
            part_two_parameterized(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1030));
    }
}
