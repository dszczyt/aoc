use std::cmp::Ordering;

use serde_json::json;

#[derive(PartialEq, Debug)]
pub enum Token {
    Number(usize),
    ParenOpen,
    ParenClose,
}

#[derive(Default, Debug)]
pub struct Line {
    pub input: String,
    pub paren_level: usize,
    current_char: usize,
    previous_chars: Vec<usize>,
}

impl Line {
    pub fn next_token(&mut self) -> Option<Token> {
        self.previous_chars.push(self.current_char);
        match self.input.chars().nth(self.current_char) {
            Some(',') => {
                self.current_char += 1;
                self.next_token()
            }
            Some('[') => {
                self.current_char += 1;
                Some(Token::ParenOpen)
            }
            Some(']') => {
                self.current_char += 1;
                Some(Token::ParenClose)
            }
            Some(ch @ ('0'..='9')) => {
                let mut s = ch.to_string();
                loop {
                    self.current_char += 1;
                    match self.input.chars().nth(self.current_char) {
                        Some(ch @ '0'..='9') => s.push(ch),
                        _ => break,
                    }
                }

                Some(Token::Number(s.parse().unwrap()))
            }
            _ => None,
        }
    }

    pub fn cancel_token(&mut self) {
        self.current_char = self.previous_chars.pop().unwrap();
    }
}

impl From<&str> for Line {
    fn from(input: &str) -> Self {
        Self {
            input: input.to_string(),
            ..Default::default()
        }
    }
}

pub struct CoupleOfLines {
    pub line1: Line,
    pub line2: Line,
}

pub fn compare_values(left: &serde_json::Value, right: &serde_json::Value) -> Option<bool> {
    match left {
        serde_json::Value::Number(i) => match right {
            serde_json::Value::Number(j) => {
                // both values are integers
                if i.as_u64() < j.as_u64() {
                    return Some(true);
                }
                if i.as_u64() > j.as_u64() {
                    return Some(false);
                }
            }
            serde_json::Value::Array(_) => {
                return compare_values(&json!([i]), right);
            }
            _ => unreachable!(),
        },
        serde_json::Value::Array(arr1) => match right {
            serde_json::Value::Array(arr2) => {
                let x: Vec<Option<bool>> = arr1
                    .iter()
                    .zip(arr2)
                    .map(|values| compare_values(values.0, values.1))
                    .filter(|result| result.is_some())
                    .take(1)
                    .collect();
                if let Some(result) = x.get(0) {
                    return result.clone();
                } else {
                    match PartialOrd::partial_cmp(&arr1.len(), &arr2.len()) {
                        Some(Ordering::Less) => return Some(true),
                        Some(Ordering::Greater) => return Some(false),
                        Some(Ordering::Equal) => return None,
                        None => unreachable!(),
                    };
                    // return None;
                }
            }
            serde_json::Value::Number(n) => {
                return compare_values(left, &json!([n]));
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    None
}

impl CoupleOfLines {
    pub fn compare(&mut self) -> bool {
        let line1: serde_json::Value = serde_json::from_str(&self.line1.input).unwrap();
        let line2: serde_json::Value = serde_json::from_str(&self.line2.input).unwrap();

        let result = compare_values(&line1, &line2);
        if let Some(result) = result {
            return result;
        }
        return true;
    }
}

fn main() {
    let input = include_str!("input");
    let lines = input.lines();
    let splitted_lines: Vec<Vec<&str>> =
        lines.enumerate().fold(Vec::new(), |mut acc, (i, line)| {
            if i % 3 == 0 {
                acc.push(vec![line]);
            } else {
                let last = acc.last_mut().unwrap();
                last.push(line);
            }
            acc
        });

    let result: usize = splitted_lines
        .iter()
        .enumerate()
        .map(|(i, group)| {
            let mut couple_of_lines = CoupleOfLines {
                line1: group.get(0).unwrap().clone().into(),
                line2: group.get(1).unwrap().clone().into(),
            };

            let result;
            if couple_of_lines.compare() {
                result = i + 1;
            } else {
                result = 0;
            }
            result
        })
        .sum();

    dbg!(&result);
}

#[cfg(test)]
mod test {
    use crate::CoupleOfLines;

    #[test]
    fn test_part1_1() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[1,1,3,1,1]".into(),
            line2: "[1,1,5,1,1]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_2() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[[1],[2,3,4]]".into(),
            line2: "[[1],4]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_3() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[9]".into(),
            line2: "[[8,7,6]]".into(),
        };
        assert!(!couple_of_lines.compare());
    }

    #[test]
    fn test_part1_4() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[[4,4],4,4]".into(),
            line2: "[[4,4],4,4]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_5() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[7,7,7,7]".into(),
            line2: "[7,7,7]".into(),
        };
        assert!(!couple_of_lines.compare());
    }

    #[test]
    fn test_part1_6() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[]".into(),
            line2: "[3]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_7() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[[[]]]".into(),
            line2: "[[]]".into(),
        };
        assert!(!couple_of_lines.compare());
    }

    #[test]
    fn test_part1_8() {
        let mut couple_of_lines = CoupleOfLines {
            line1: "[1,[2,[3,[4,[5,6,7]]]],8,9]".into(),
            line2: "[1,[2,[3,[4,[5,6,0]]]],8,9]".into(),
        };
        assert!(!couple_of_lines.compare());
    }
}
