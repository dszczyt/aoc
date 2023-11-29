#[derive(Clone, Debug)]
pub struct Rucksack {
    pub items: String,
}

impl Rucksack {
    pub fn common_in_compartments(self) -> String {
        let compartments: Vec<Compartment> = self.into();
        common_chars(
            compartments.get(0).unwrap().items.clone(),
            compartments.get(1).unwrap().items.clone(),
        )
    }

    pub fn priority(self) -> usize {
        priority(
            self.common_in_compartments()
                .chars()
                .nth(0)
                .unwrap_or_default(),
        )
    }
}

pub fn priority(c: char) -> usize {
    match c {
        c @ ('a'..='z') => c as usize - ('a' as usize) + 1,
        c @ ('A'..='Z') => c as usize - ('A' as usize) + 27,
        _ => 0,
    }
}

pub fn rucksacks_priority(rucksacks: Vec<Rucksack>) -> usize {
    priority(
        common_chars_3(
            rucksacks.get(0).unwrap().items.clone(),
            rucksacks.get(1).unwrap().items.clone(),
            rucksacks.get(2).unwrap().items.clone(),
        )
        .chars()
        .nth(0)
        .unwrap_or_default(),
    )
}

#[derive(Clone, Debug)]
struct Compartment {
    pub items: String,
}

impl From<Rucksack> for Vec<Compartment> {
    fn from(rucksack: Rucksack) -> Self {
        let chars = rucksack.items.chars();
        let count = chars.clone().count();
        let compartment1: Vec<char> = chars.clone().take(count / 2).collect(); //.as_str().into();
        let compartment1: String = compartment1.iter().collect();
        let compartment1 = Compartment {
            items: compartment1.clone(),
        };
        let compartment2: Vec<char> = chars.skip(count / 2).collect();
        let compartment2: String = compartment2.iter().collect();
        let compartment2 = Compartment {
            items: compartment2.clone(),
        };
        vec![compartment1, compartment2]
    }
}

pub fn common_chars(s1: String, s2: String) -> String {
    let mut chars: Vec<char> = s1
        .chars()
        .filter(|c1| s2.chars().any(|c2| c2.eq(&c1)))
        .collect();
    chars.dedup();
    String::from_iter(chars)
}

pub fn common_chars_3(s1: String, s2: String, s3: String) -> String {
    let mut chars: Vec<char> = s1
        .chars()
        .filter(|c1| s2.chars().any(|c2| c2.eq(&c1)))
        .filter(|c1| s3.chars().any(|c3| c3.eq(&c1)))
        .collect();
    chars.dedup();
    String::from_iter(chars)
}

#[derive(Clone, Debug)]
pub struct Rucksacks {
    pub rucksacks: Vec<Rucksack>,
}

impl From<&str> for Rucksacks {
    fn from(input: &str) -> Self {
        Self {
            rucksacks: input
                .lines()
                .map(|line| Rucksack {
                    items: line.to_string(),
                })
                .collect(),
        }
    }
}

impl Rucksacks {
    pub fn sum_priorities(self) -> usize {
        self.rucksacks
            .iter()
            .map(|rucksack| rucksack.clone().priority())
            .sum()
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    pub rucksacks: Rucksacks,
}

impl Group {
    pub fn sum_priorities(self) -> usize {
        rucksacks_priority(self.rucksacks.rucksacks)
    }
}

pub struct Groups {
    pub groups: Vec<Group>,
}

impl Groups {
    pub fn sum_priorities(self) -> usize {
        self.groups
            .iter()
            .map(|group| group.clone().sum_priorities())
            .sum()
    }
}

impl From<&str> for Groups {
    fn from(input: &str) -> Self {
        let groups: Vec<Group> = input
            .lines()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, line)| {
                if i % 3 == 0 {
                    acc.push(vec![line]);
                } else {
                    let last = acc.last_mut().unwrap();
                    last.push(line);
                }
                acc
            })
            .iter()
            .map(|elems| Group {
                rucksacks: Rucksacks {
                    rucksacks: elems
                        .iter()
                        .map(|elem| Rucksack {
                            items: elem.to_string(),
                        })
                        .collect(),
                },
            })
            .collect();
        groups.iter().for_each(|e| {
            dbg!(&e);
        });
        Self { groups }
    }
}

#[cfg(test)]
mod test {
    use crate::{common_chars, common_chars_3, Compartment, Groups, Rucksack, Rucksacks};

    #[test]
    fn test_rucksack_to_vec_of_compartments_1() {
        let compartments: Vec<Compartment> = Rucksack {
            items: "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
        }
        .into();
        assert_eq!(
            compartments.get(0).unwrap().items,
            "vJrwpWtwJgWr".to_string()
        );
        assert_eq!(
            compartments.get(1).unwrap().items,
            "hcsFMMfFFhFp".to_string()
        );
    }

    #[test]
    fn test_rucksack_to_vec_of_compartments_2() {
        let compartments: Vec<Compartment> = Rucksack {
            items: "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
        }
        .into();
        assert_eq!(
            compartments.get(0).unwrap().items,
            "jqHRNqRjqzjGDLGL".to_string()
        );
        assert_eq!(
            compartments.get(1).unwrap().items,
            "rsFMfFZSrLrFZsSL".to_string()
        );
    }

    macro_rules! test_common_chars {
        ($($name:ident: ($a:expr, $b:expr, $expected:expr))*) => {$(
          #[test]
          fn $name() {
            assert_eq!(common_chars($a.to_string(), $b.to_string()), $expected.to_string())
          }
        )*};
    }

    test_common_chars! {
      test_common_chars_empty:("", "", "")
      test_common_chars_none:("a", "b", "")
      test_common_chars_one_only:("a", "a", "a")
      test_common_chars_one:("ab", "a", "a")
      test_common_chars_many:("abcx", "xbd", "bx")
      test_common_chars_duplicates:("aaaab", "aac", "a")
    }

    macro_rules! test_common_in_compartments {
      ($($name:ident: ($items:expr, $expected:expr))*) => {$(
        #[test]
        fn $name() {
          let rucksack = Rucksack{items: $items.to_string()};
          assert_eq!(rucksack.common_in_compartments(), $expected.to_string())
        }
      )*};
    }

    test_common_in_compartments! {
      test_common_in_compartments_empty: ("", "")
      test_common_in_compartments_none: ("ab", "")
      test_common_in_compartments_one_only: ("aa", "a")
      test_common_in_compartments_one: ("abcade", "a")
      test_common_in_compartments_many: ("abcdacef", "ac")
      test_common_in_compartments_1: ("vJrwpWtwJgWrhcsFMMfFFhFp", "p")
      test_common_in_compartments_2: ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "L")
      test_common_in_compartments_3: ("PmmdzqPrVvPwwTWBwg", "P")
      test_common_in_compartments_4: ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "v")
      test_common_in_compartments_5: ("ttgJtRGJQctTZtZT", "t")
      test_common_in_compartments_6: ("CrZsJsPPZsGzwwsLwLmpwMDw", "s")
    }

    macro_rules! test_priority {
      ($($name:ident: ($items:expr, $expected:expr))*) => {$(
        #[test]
        fn $name() {
          let rucksack = Rucksack{items: $items.to_string()};
          assert_eq!(rucksack.priority(), $expected)
        }
      )*};
    }

    test_priority! {
      test_priority_empty: ("", 0)
      test_priority_1: ("vJrwpWtwJgWrhcsFMMfFFhFp", 16)
      test_priority_2: ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38)
      test_priority_3: ("PmmdzqPrVvPwwTWBwg", 42)
      test_priority_4: ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 22)
      test_priority_5: ("ttgJtRGJQctTZtZT", 20)
      test_priority_6: ("CrZsJsPPZsGzwwsLwLmpwMDw", 19)
    }

    #[test]
    fn test_rucksacks_priority() {
        let rucksacks: Rucksacks = include_str!("example").into();
        assert_eq!(rucksacks.sum_priorities(), 157);
    }

    macro_rules! test_common_chars_3 {
      ($($name:ident: ($a:expr, $b:expr, $c:expr, $expected:expr))*) => {$(
        #[test]
        fn $name() {
          assert_eq!(common_chars_3($a.to_string(), $b.to_string(), $c.to_string()), $expected.to_string())
        }
      )*};
  }

    test_common_chars_3! {
      test_common_chars_3_1: ("vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg", "r")
      test_common_chars_3_2: ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw", "Z")
    }

    #[test]
    fn test_groups_sum_priotities() {
        let input = include_str!("example");
        let rucksacks: Groups = input.into();
        assert_eq!(rucksacks.sum_priorities(), 70);
    }
}

fn main() {
    let input = include_str!("input");
    let rucksacks: Groups = input.into();
    dbg!(rucksacks.sum_priorities());
}
