advent_of_code::solution!(10);

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Pos {
    y: usize,
    x: usize,
}

fn decrement_coordinate(c: usize) -> Result<usize, usize> {
    match c {
        0 => Err(0),
        _ => Ok(c - 1),
    }
}

fn increment_coordinate(c: usize, max: &usize) -> Result<usize, usize> {
    if c == *max {
        Err(*max)
    } else {
        Ok(c + 1)
    }
}

fn get_coordinates(
    pipe: char,
    starting_pos: &Pos,
    max_x: &usize,
    max_y: &usize,
) -> Result<(Pos, Pos), ()> {
    let y = starting_pos.y;
    let x = starting_pos.x;
    let point_1 = match pipe {
        '|' | 'L' | 'J' => Pos {
            y: match decrement_coordinate(y) {
                Ok(c) => c,
                Err(_) => return Err(()),
            },
            x,
        },
        '-' | '7' => Pos {
            y,
            x: match decrement_coordinate(x) {
                Ok(c) => c,
                Err(_) => return Err(()),
            },
        },
        'F' => Pos {
            y,
            x: match increment_coordinate(x, max_x) {
                Ok(c) => c,
                Err(_) => return Err(()),
            },
        },
        '.' => return Err(()),
        'S' => return Err(()),
        _ => panic!(),
    };
    let point_2 = match pipe {
        '|' | '7' | 'F' => Pos {
            y: match increment_coordinate(y, max_y) {
                Ok(c) => c,
                Err(_) => return Err(()),
            },
            x,
        },
        '-' | 'L' => Pos {
            y,
            x: match increment_coordinate(x, max_x) {
                Ok(c) => c,
                Err(_) => return Err(()),
            },
        },
        'J' => Pos {
            y,
            x: match decrement_coordinate(x) {
                Ok(c) => c,
                Err(_) => return Err(()),
            },
        },
        _ => panic!(),
    };
    Ok((point_1, point_2))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut starting_position = Pos::default();
    for (y, line) in input.lines().enumerate() {
        matrix.push(line.chars().collect());
        match line.find('S') {
            None => {}
            Some(x) => {
                starting_position.x = x;
                starting_position.y = y
            }
        }
    }

    let mut loop_: Vec<Pos> = vec![];
    let mut next_position: Pos = Pos::default();
    let max_y: usize = matrix.len() - 1;
    let max_x: usize = matrix[0].len() - 1;

    let up_left = Pos {
        y: match decrement_coordinate(starting_position.y) {
            Ok(c) => c,
            Err(c) => c,
        },

        x: match decrement_coordinate(starting_position.x) {
            Ok(c) => c,
            Err(c) => c,
        },
    };
    let down_right = Pos {
        y: match increment_coordinate(starting_position.y, &max_y) {
            Ok(c) => c,
            Err(c) => c,
        },
        x: match increment_coordinate(starting_position.x, &max_x) {
            Ok(c) => c,
            Err(c) => c,
        },
    };
    let mut previous_pos = Pos::default();
    'outer: for y in up_left.y..=down_right.y {
        for x in up_left.x..=down_right.x {
            let pipe = matrix[y][x];
            let pipe_position = Pos { y, x };
            let points = match get_coordinates(pipe, &pipe_position, &max_x, &max_y) {
                Ok(x) => x,
                Err(_) => continue,
            };
            if points.0 == starting_position {
                next_position = points.1;
                previous_pos = pipe_position;
                break 'outer;
            } else if points.1 == starting_position {
                next_position = points.0;
                previous_pos = pipe_position;
                break 'outer;
            }
        }
    }
    loop_.push(starting_position.clone());
    loop_.push(previous_pos.clone());
    loop_.push(next_position.clone());
    while next_position != starting_position {
        let pipe = matrix[next_position.y][next_position.x];
        let points = get_coordinates(pipe, &next_position, &max_x, &max_y).unwrap();
        if points.0 == previous_pos {
            previous_pos = next_position.clone();
            next_position = points.1;
        } else if points.1 == previous_pos {
            previous_pos = next_position.clone();
            next_position = points.0;
        } else {
            panic!()
        }
        if next_position == starting_position {
            break;
        }

        loop_.push(next_position.clone());
    }

    Some((loop_.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut starting_position = Pos::default();
    for (y, line) in input.lines().enumerate() {
        matrix.push(line.chars().collect());
        match line.find('S') {
            None => {}
            Some(x) => {
                starting_position.x = x;
                starting_position.y = y
            }
        }
    }

    let mut loop_: Vec<Pos> = vec![];
    let mut next_position: Pos = Pos::default();
    let max_y: usize = matrix.len() - 1;
    let max_x: usize = matrix[0].len() - 1;

    let up_left = Pos {
        y: match decrement_coordinate(starting_position.y) {
            Ok(c) => c,
            Err(c) => c,
        },

        x: match decrement_coordinate(starting_position.x) {
            Ok(c) => c,
            Err(c) => c,
        },
    };
    let down_right = Pos {
        y: match increment_coordinate(starting_position.y, &max_y) {
            Ok(c) => c,
            Err(c) => c,
        },
        x: match increment_coordinate(starting_position.x, &max_x) {
            Ok(c) => c,
            Err(c) => c,
        },
    };
    let mut previous_pos = Pos::default();
    'outer: for y in up_left.y..=down_right.y {
        for x in up_left.x..=down_right.x {
            let pipe = matrix[y][x];
            let pipe_position = Pos { y, x };
            let points = match get_coordinates(pipe, &pipe_position, &max_x, &max_y) {
                Ok(x) => x,
                Err(_) => continue,
            };
            if points.0 == starting_position {
                next_position = points.1;
                previous_pos = pipe_position;
                break 'outer;
            } else if points.1 == starting_position {
                next_position = points.0;
                previous_pos = pipe_position;
                break 'outer;
            }
        }
    }
    loop_.push(starting_position.clone());
    loop_.push(previous_pos.clone());
    loop_.push(next_position.clone());
    while next_position != starting_position {
        let pipe = matrix[next_position.y][next_position.x];
        let points = get_coordinates(pipe, &next_position, &max_x, &max_y).unwrap();
        if points.0 == previous_pos {
            previous_pos = next_position.clone();
            next_position = points.1;
        } else if points.1 == previous_pos {
            previous_pos = next_position.clone();
            next_position = points.0;
        } else {
            panic!()
        }
        if next_position == starting_position {
            break;
        }

        loop_.push(next_position.clone());
    }
    let mut new_matrix: Vec<Vec<char>> = vec![];
    for _ in 0..max_y + 1 {
        new_matrix.push(vec!['N'; max_x + 1])
    }
    for point in loop_ {
        new_matrix[point.y][point.x] = match matrix[point.y][point.x] {
            'F' | '7' | '|' | 'S' => 'V',
            'J' | 'L' | '-' => 'H',
            _ => 'N',
        }
    }
    let mut sum: u32 = 0;
    let mut inside = false;
    for line in new_matrix {
        for character in line {
            match character {
                'V' => inside = !inside,
                'H' => continue,
                'N' => {
                    if inside {
                        sum += 1
                    }
                }
                _ => panic!(),
            }
        }
        if inside {
            panic!()
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples_2", DAY));
        assert_eq!(result, Some(10));
    }
}
