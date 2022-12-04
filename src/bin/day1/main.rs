use std::str::Lines;

fn max_sum_of_grouped_lines(lines: Lines) -> Option<usize> {
    lines
        .fold(Vec::new(), |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else {
                if let Some(last) = acc.last_mut() {
                    last.push(line);
                } else {
                    acc.push(vec![line])
                }
            }
            acc
        })
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|line| usize::from_str_radix(line, 10).unwrap())
                .sum::<usize>()
        })
        .max()
}

#[cfg(test)]
mod test {
    use std::str::Lines;

    use crate::max_sum_of_grouped_lines;

    #[test]
    fn test_when_empty() {
        assert!(max_sum_of_grouped_lines("".lines()).is_none());
    }

    #[test]
    fn test_when_empty_lines() {
        assert!(max_sum_of_grouped_lines("\n\n\n".lines()).unwrap() == 0)
    }

    #[test]
    fn test_simple_one() {
        assert_eq!(max_sum_of_grouped_lines("13".lines()).unwrap(), 13)
    }

    #[test]
    fn test_simple_two() {
        assert_eq!(max_sum_of_grouped_lines("13\n\n17".lines()).unwrap(), 17)
    }

    #[test]
    fn test_global() {
        assert_eq!(
            max_sum_of_grouped_lines("\n\n13\n23\n\n\n17\n21\n\n".lines()).unwrap(),
            38 // the sum of 17+21
        )
    }
}

fn main() {
    let input = include_str!("input");

    let result = max_sum_of_grouped_lines(input.lines());
    dbg!(&result);
}
