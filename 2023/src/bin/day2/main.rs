use std::collections::HashMap;

use anyhow::Context;
use once_cell::sync::Lazy;

static CONFIGURATION: Lazy<HashMap<&str, usize>> = Lazy::new(|| {
    vec![("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect()
});

pub struct Cube {
    pub color: String,
    pub size: usize,
}

impl TryFrom<&str> for Cube {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        let mut parts = value.split(' ');
        let size: usize = parts.next().context("missing size part")?.parse()?;
        let color = parts.next().context("missing color part")?.to_string();
        Ok(Self { color, size })
    }
}

pub struct Subset(Vec<Cube>);

impl Subset {
    pub fn counts(&self) -> HashMap<&str, usize> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for cube in self.0.iter() {
            let count = counts.entry(cube.color.as_str()).or_insert(0);
            *count += cube.size;
        }
        counts
    }

    pub fn possible(&self) -> bool {
        self.counts().iter().all(|(color, count)| {
            let max = CONFIGURATION.get(color).unwrap_or(&0);
            count <= max
        })
    }
}

impl TryFrom<&str> for Subset {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        let mut cubes: Vec<Cube> = vec![];

        for cube in value.split(',') {
            cubes.push(cube.trim().try_into()?);
        }
        Ok(Self(cubes))
    }
}

pub struct Game {
    pub id: i64,
    pub subsets: Vec<Subset>,
}

impl Game {
    pub fn possible(&self) -> bool {
        self.subsets.iter().all(|subset| subset.possible())
    }

    fn fewest_number_of_cubes(&self) -> HashMap<&str, usize> {
        let mut counts: HashMap<&str, usize> = HashMap::new();

        for subset in self.subsets.iter() {
            for (color, count) in subset.counts().iter() {
                let current_count = counts.entry(color).or_insert(0);
                if *current_count < *count {
                    *current_count = *count;
                }
            }
        }

        counts
    }

    pub fn power(&self) -> usize {
        self.fewest_number_of_cubes()
            .values()
            .cloned()
            .reduce(|a, b| a * b)
            .unwrap_or(0)
    }
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> anyhow::Result<Self> {
        let mut parts = value.split(':');
        let mut game_part = parts.next().context("missing game part")?.split(' ');
        game_part.next().context("missing label part")?;
        let id: i64 = game_part.next().context("missing id part")?.parse()?;
        let mut subsets: Vec<Subset> = vec![];
        for subset in parts.next().context("missing subsets part")?.split(';') {
            subsets.push(subset.trim().try_into()?);
        }
        Ok(Self { id, subsets })
    }
}

fn possible_games(input: &str) -> anyhow::Result<Vec<Game>> {
    let mut games = vec![];
    for line in input.lines() {
        let game: Game = line.try_into()?;
        if game.possible() {
            games.push(game);
        }
    }
    Ok(games)
}

pub fn sum_of_ids_of_possible_games(input: &str) -> anyhow::Result<i64> {
    Ok(possible_games(input)?.iter().map(|game| game.id).sum())
}

pub fn power(input: &str) -> anyhow::Result<usize> {
    let mut power = 0;

    for line in input.lines() {
        let game: Game = line.try_into()?;
        power += game.power();
    }

    Ok(power)
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/part1");
    println!("part1: {}", sum_of_ids_of_possible_games(input)?);
    println!("part2: {}", power(input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{power, sum_of_ids_of_possible_games};

    #[test]
    fn example1() {
        let sample = include_str!("samples/part1");

        assert_eq!(sum_of_ids_of_possible_games(sample).unwrap(), 8);
    }

    #[test]
    fn example2() {
        let sample = include_str!("samples/part2");

        assert_eq!(power(sample).unwrap(), 2286);
    }
}
