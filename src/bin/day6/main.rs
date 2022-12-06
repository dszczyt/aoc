pub const PART1: usize = 4;
pub const PART2: usize = 14;

pub fn find_marker(input: &str, n: usize) -> usize {
    let mut stack: Vec<char> = vec![];
    let result = input.char_indices().find(|(_, c)| {
        while stack.contains(c) {
            stack.remove(0);
        }
        stack.push(c.clone());
        stack.len() == n
    });
    result.unwrap().0 + 1
}

#[cfg(test)]
mod test {
    use crate::{find_marker, PART1, PART2};

    #[test]
    fn example() {
        let input = include_str!("example");
        assert_eq!(find_marker(input, PART1), 7);
    }

    #[test]
    fn example2_1() {
        let input = include_str!("example2_1");
        assert_eq!(find_marker(input, PART2), 19);
    }

    #[test]
    fn example2_2() {
        let input = include_str!("example2_2");
        assert_eq!(find_marker(input, PART2), 23);
    }

    #[test]
    fn example2_3() {
        let input = include_str!("example2_3");
        assert_eq!(find_marker(input, PART2), 23);
    }

    #[test]
    fn example2_4() {
        let input = include_str!("example2_4");
        assert_eq!(find_marker(input, PART2), 29);
    }

    #[test]
    fn example2_5() {
        let input = include_str!("example2_5");
        assert_eq!(find_marker(input, PART2), 26);
    }
}

fn main() {
    let input = include_str!("input");
    dbg!(find_marker(input, PART2));
}
