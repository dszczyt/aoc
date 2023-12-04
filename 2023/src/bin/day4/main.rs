use std::str::Lines;

pub struct Numbers(Vec<u64>);

impl From<&str> for Numbers {
    fn from(s: &str) -> Self {
        let numbers = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        Self(numbers)
    }
}

pub struct Card {
    pub numbers_you_have: Numbers,
    pub winning_numbers: Numbers,
}

impl From<&str> for Card {
    fn from(s: &str) -> Self {
        let mut parts = s.split(':').nth(1).unwrap().split('|');
        let numbers_you_have = parts.next().unwrap().trim_start().into();
        let winning_numbers = parts.next().unwrap().trim_start().into();
        Self {
            numbers_you_have,
            winning_numbers,
        }
    }
}

struct Cards(Vec<Card>);

impl From<Lines<'_>> for Cards {
    fn from(lines: Lines<'_>) -> Self {
        Self(lines.map(|line| line.into()).collect())
    }
}

impl Cards {
    pub fn score(&self) -> u64 {
        self.0.iter().map(|card| card.score()).sum()
    }
}

impl Card {
    fn matches(&self) -> Vec<u64> {
        let mut matches = vec![];
        for number in self.numbers_you_have.0.iter() {
            if self.winning_numbers.0.contains(number) {
                matches.push(*number);
            }
        }
        matches
    }

    pub fn score(&self) -> u64 {
        let length = self.matches().len();
        if length == 0 {
            return 0;
        }
        u64::pow(2, (self.matches().len() - 1) as u32)
    }
}

fn main() {
    let cards: Cards = include_str!("inputs/part1").lines().into();
    println!("part1: {}", cards.score());
}

#[cfg(test)]
mod tests {
    use super::Cards;

    #[test]
    fn test_score() {
        let cards: Cards = include_str!("samples/part1").lines().into();
        assert_eq!(cards.score(), 13);
    }
}
