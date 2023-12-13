advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrizes: Vec<Vec<Vec<char>>> = vec![];
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut sum: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            matrizes.push(matrix.clone());
            matrix.clear()
        } else {
            matrix.push(line.chars().collect::<Vec<char>>())
        }
    }
    matrizes.push(matrix);

    'matrix: for matrix in matrizes {
        let max_y = matrix.len();
        let max_x = matrix[0].len();

        for y in 1..max_y {
            if matrix[y - 1] == matrix[y] {
                if matrix[0..y]
                    .iter()
                    .rev()
                    .zip(matrix[y..max_y].iter())
                    .all(|(l, r)| l == r)
                {
                    sum += y as u32 * 100;
                    continue 'matrix;
                }
            }
        }
        let mut matrix_iters: Vec<_> = matrix.into_iter().map(|v| v.into_iter()).collect();
        let matrix_transposed: Vec<Vec<char>> = (0..max_x)
            .map(|_| {
                matrix_iters
                    .iter_mut()
                    .map(|v| v.next().unwrap())
                    .collect::<Vec<char>>()
            })
            .collect();

        for x in 1..max_x {
            if matrix_transposed[x - 1] == matrix_transposed[x] {
                if matrix_transposed[0..x]
                    .iter()
                    .rev()
                    .zip(matrix_transposed[x..max_x].iter())
                    .all(|(l, r)| l == r)
                {
                    sum += x as u32;
                    continue 'matrix;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrizes: Vec<Vec<Vec<char>>> = vec![];
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut sum: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            matrizes.push(matrix.clone());
            matrix.clear()
        } else {
            matrix.push(line.chars().collect::<Vec<char>>())
        }
    }
    matrizes.push(matrix);

    'matrix: for matrix in matrizes {
        let max_y = matrix.len();
        let max_x = matrix[0].len();
        let mut smudge = 0;
        for y in 1..max_y {
            smudge = matrix[y - 1]
                .iter()
                .zip(&matrix[y])
                .map(|c| if c.0 == c.1 { 0 } else { 1 })
                .sum();

            if smudge == 1 {
                if matrix[0..y - 1]
                    .iter()
                    .rev()
                    .zip(matrix[y + 1..max_y].iter())
                    .all(|(l, r)| l == r)
                {
                    sum += y as u32 * 100;
                    continue 'matrix;
                }
            } else if smudge == 0 {
                if 1 == matrix[0..y - 1]
                    .iter()
                    .rev()
                    .zip(matrix[y + 1..max_y].iter())
                    .map(|(l, r)| {
                        l.iter()
                            .zip(r)
                            .map(|c| if c.0 == c.1 { 0 } else { 1 })
                            .sum::<i32>()
                    })
                    .sum()
                {
                    sum += y as u32 * 100;
                    continue 'matrix;
                }
            }
        }
        let mut matrix_iters: Vec<_> = matrix.into_iter().map(|v| v.into_iter()).collect();
        let matrix_transposed: Vec<Vec<char>> = (0..max_x)
            .map(|_| {
                matrix_iters
                    .iter_mut()
                    .map(|v| v.next().unwrap())
                    .collect::<Vec<char>>()
            })
            .collect();
        let matrix = matrix_transposed;
        for x in 1..max_x {
            smudge = matrix[x - 1]
                .iter()
                .zip(&matrix[x])
                .map(|c| if c.0 == c.1 { 0 } else { 1 })
                .sum();

            if smudge == 1 {
                if matrix[0..x - 1]
                    .iter()
                    .rev()
                    .zip(matrix[x + 1..max_x].iter())
                    .all(|(l, r)| l == r)
                {
                    sum += x as u32;
                    continue 'matrix;
                }
            } else if smudge == 0 {
                if 1 == matrix[0..x - 1]
                    .iter()
                    .rev()
                    .zip(matrix[x + 1..max_x].iter())
                    .map(|(l, r)| {
                        l.iter()
                            .zip(r)
                            .map(|c| if c.0 == c.1 { 0 } else { 1 })
                            .sum::<i32>()
                    })
                    .sum()
                {
                    sum += x as u32;
                    continue 'matrix;
                }
            }
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
