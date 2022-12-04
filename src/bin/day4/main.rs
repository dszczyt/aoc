#[derive(Clone, Debug)]
pub struct Section {
    pub from: usize,
    pub to: usize,
}

impl From<&str> for Section {
    fn from(input: &str) -> Self {
        let values: Vec<usize> = input
            .split("-")
            .map(|input| usize::from_str_radix(input, 10).unwrap())
            .collect();
        Self {
            from: values.get(0).unwrap().clone(),
            to: values.get(1).unwrap().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Pair {
    pub first: Section,
    pub second: Section,
}

impl From<&str> for Pair {
    fn from(input: &str) -> Self {
        let sections: Vec<Section> = input.split(",").map(|input| input.into()).collect();
        Self {
            first: sections.get(0).unwrap().clone(),
            second: sections.get(1).unwrap().clone(),
        }
    }
}

impl Pair {
    pub fn overlaps(&self) -> bool {
        (self.first.from >= self.second.from && self.first.to <= self.second.to)
            || (self.second.from >= self.first.from && self.second.to <= self.first.to)
    }
}

pub fn count_overlaping_pairs(input: &str) -> usize {
    input
        .lines()
        .map(|line| Into::<Pair>::into(line))
        .filter(|pair| pair.overlaps())
        .count()
}

#[cfg(test)]
mod test {
    use crate::{count_overlaping_pairs, Pair, Section};

    #[test]
    fn test_section() {
        let section: Section = "2-4".into();
        assert_eq!(section.from, 2);
        assert_eq!(section.to, 4);
    }

    #[test]
    fn test_pair() {
        let pair: Pair = "2-4,6-8".into();
        assert_eq!(pair.first.from, 2);
        assert_eq!(pair.first.to, 4);
        assert_eq!(pair.second.from, 6);
        assert_eq!(pair.second.to, 8);
    }

    #[test]
    fn test_pair_overlaps_1() {
        assert!(!Into::<Pair>::into("2-4,6-8").overlaps());
    }

    #[test]
    fn test_pair_overlaps_2() {
        assert!(!Into::<Pair>::into("2-3,4-5").overlaps());
    }

    #[test]
    fn test_pair_overlaps_3() {
        assert!(!Into::<Pair>::into("5-7,7-9").overlaps());
    }

    #[test]
    fn test_pair_overlaps_4() {
        assert!(Into::<Pair>::into("2-8,3-7").overlaps());
    }

    #[test]
    fn test_pair_overlaps_5() {
        assert!(Into::<Pair>::into("6-6,4-6").overlaps());
    }

    #[test]
    fn test_pair_overlaps_6() {
        assert!(!Into::<Pair>::into("2-6,4-8").overlaps());
    }

    #[test]
    fn example_part1() {
        assert_eq!(count_overlaping_pairs(include_str!("example")), 2);
    }
}

fn main() {
    let input = include_str!("input");
    dbg!(count_overlaping_pairs(input));
}
