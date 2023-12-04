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

fn main() {
    let input = include_str!("inputs/part1");

    let numbers = numbers(input);

    let total: u64 = numbers.iter().sum();
    println!("part1: {}", total);
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
}
