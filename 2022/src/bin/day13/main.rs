use std::cmp::Ordering;

use serde_json::json;

#[derive(PartialEq, Debug)]
pub enum Token {
    Number(usize),
    ParenOpen,
    ParenClose,
}

#[derive(PartialEq, Eq, Default, Debug)]
pub struct Line {
    pub input: String,
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
            ..Default::default()
        }
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let couple_of_lines = CoupleOfLines {
            line1: self.clone(),
            line2: other.clone(),
        };

        if couple_of_lines.compare() {
            return Some(Ordering::Less);
        }
        return Some(Ordering::Greater);
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        let couple_of_lines = CoupleOfLines {
            line1: self.clone(),
            line2: other.clone(),
        };

        if couple_of_lines.compare() {
            return Ordering::Less;
        }
        return Ordering::Greater;
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
    pub fn compare(&self) -> bool {
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

    let mut lines: Vec<Line> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|input| input.into())
        .collect();
    lines.push("[[2]]".into());
    lines.push("[[6]]".into());
    lines.sort();
    dbg!(&lines);

    let result: Vec<usize> = ["[[2]]", "[[6]]"]
        .iter()
        .map(|key| {
            lines
                .iter()
                .enumerate()
                .find(|(i, line)| &line.input == key)
                .map(|(i, _)| i + 1)
                .take()
                .unwrap()
        })
        .collect();
    dbg!(&result);
    dbg!((&result.get(0)).unwrap() * (&result.get(1)).unwrap());
}

#[cfg(test)]
mod test {
    use crate::CoupleOfLines;

    #[test]
    fn test_part1_1() {
        let couple_of_lines = CoupleOfLines {
            line1: "[1,1,3,1,1]".into(),
            line2: "[1,1,5,1,1]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_2() {
        let couple_of_lines = CoupleOfLines {
            line1: "[[1],[2,3,4]]".into(),
            line2: "[[1],4]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_3() {
        let couple_of_lines = CoupleOfLines {
            line1: "[9]".into(),
            line2: "[[8,7,6]]".into(),
        };
        assert!(!couple_of_lines.compare());
    }

    #[test]
    fn test_part1_4() {
        let couple_of_lines = CoupleOfLines {
            line1: "[[4,4],4,4]".into(),
            line2: "[[4,4],4,4]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_5() {
        let couple_of_lines = CoupleOfLines {
            line1: "[7,7,7,7]".into(),
            line2: "[7,7,7]".into(),
        };
        assert!(!couple_of_lines.compare());
    }

    #[test]
    fn test_part1_6() {
        let couple_of_lines = CoupleOfLines {
            line1: "[]".into(),
            line2: "[3]".into(),
        };
        assert!(couple_of_lines.compare());
    }

    #[test]
    fn test_part1_7() {
        let couple_of_lines = CoupleOfLines {
            line1: "[[[]]]".into(),
            line2: "[[]]".into(),
        };
        assert!(!couple_of_lines.compare());
    }

    #[test]
    fn test_part1_8() {
        let couple_of_lines = CoupleOfLines {
            line1: "[1,[2,[3,[4,[5,6,7]]]],8,9]".into(),
            line2: "[1,[2,[3,[4,[5,6,0]]]],8,9]".into(),
        };
        assert!(!couple_of_lines.compare());
    }
}
