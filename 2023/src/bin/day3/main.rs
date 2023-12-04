pub fn extract_number_and_positions(line: &str) -> Vec<(u64, Vec<usize>)> {
    let mut result = vec![];

    let mut current_number = 0;
    let mut current_positions = vec![];

    line.char_indices().for_each(|(idx, ch)| {
        if ch.is_numeric() {
            current_number = current_number * 10 + ch.to_digit(10).unwrap() as u64;
            current_positions.push(idx);
        } else if current_number != 0 {
            result.push((current_number, current_positions.clone()));
            current_number = 0;
            current_positions = vec![];
        }
    });

    if current_number != 0 {
        result.push((current_number, current_positions.clone()));
    }

    result
}

pub fn symbol_coordinates(input: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];

    input.lines().enumerate().for_each(|(j, line)| {
        line.chars().enumerate().for_each(|(i, ch)| {
            if !ch.is_numeric() && ch != '.' {
                result.push((i, j));
            }
        })
    });

    result
}

pub fn numbers(input: &str) -> Vec<u64> {
    let mut result = vec![];

    let symbols = symbol_coordinates(input);

    input.lines().enumerate().for_each(|(j, line)| {
        extract_number_and_positions(line)
            .iter()
            .for_each(|(number, positions)| {
                let min_j = if j == 0 { 0 } else { j - 1 };
                for y in min_j..=j + 1 {
                    let min_pos = if positions[0] == 0 {
                        0
                    } else {
                        positions[0] - 1
                    };
                    for x in min_pos..=positions.last().unwrap() + 1 {
                        for (symbol_x, symbol_y) in &symbols {
                            if x == *symbol_x && y == *symbol_y {
                                result.push(*number);
                                return;
                            }
                        }
                    }
                }
            });
    });

    result
}

fn find_stars(input: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];

    input.lines().enumerate().for_each(|(j, line)| {
        line.chars().enumerate().for_each(|(i, ch)| {
            if ch == '*' {
                result.push((i, j));
            }
        })
    });

    result
}

fn numbers_next_to_star(
    star: (usize, usize),
    row: usize,
    numbers: Vec<(u64, Vec<usize>)>,
) -> Vec<u64> {
    let mut result = vec![];

    let min_i = if star.0 == 0 { 0 } else { star.0 - 1 };
    let min_j = if star.1 == 0 { 0 } else { star.1 - 1 };

    numbers.iter().for_each(|(number, positions)| {
        for y in min_j..=star.1 + 1 {
            for x in min_i..=star.0 + 1 {
                for position in positions {
                    if x == *position && y == row {
                        result.push(*number);
                        return;
                    }
                }
            }
        }
    });

    result
}

pub fn gear_ratios(input: &str) -> Vec<u64> {
    let mut result = vec![];

    let stars = find_stars(input);

    stars.iter().for_each(|star| {
        let mut numbers = vec![];
        for (row, line) in input.lines().enumerate() {
            let numbers_and_positions = extract_number_and_positions(line);
            numbers.extend(numbers_next_to_star(*star, row, numbers_and_positions));
        }
        if numbers.len() == 2 {
            result.push(numbers[0] * numbers[1]);
        }
    });

    result
}

fn main() {
    let input = include_str!("inputs/part1");

    let numbers = numbers(input);

    let total: u64 = numbers.iter().sum();
    println!("part1: {}", total);

    let gear_ratios = gear_ratios(input);
    let total: u64 = gear_ratios.iter().sum();

    println!("part2: {}", total);
}

#[cfg(test)]
mod tests {
    use super::numbers;

    #[test]
    fn test_numbers() {
        let input = include_str!("samples/part1");

        let numbers = numbers(input);

        assert_eq!(numbers, vec![467, 35, 633, 617, 592, 755, 664, 598]);

        let total: u64 = numbers.iter().sum();
        assert_eq!(total, 4361);
    }

    #[test]
    fn test_gear_ratios() {
        let input = include_str!("samples/part1");

        let gear_ratios = super::gear_ratios(input);

        assert_eq!(gear_ratios, vec![16345, 451490]);
    }
}
