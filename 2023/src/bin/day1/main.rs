use anyhow::Context;
use core::str::Lines;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/part1").to_string();
    println!("{}", input.lines().calibration_value()?);

    let input = include_str!("inputs/part2").to_string();
    println!("{}", input.lines().calibration_value()?);

    Ok(())
}

//const NUMBERS: [&str; 20] = [
//    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
//    "five", "six", "seven", "eight", "nine",
//];

static NUMBERS: Lazy<HashMap<&str, i64>> = Lazy::new(|| {
    vec![
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .collect()
});

pub trait Calibrator {
    fn calibration_value(&self) -> anyhow::Result<i64>;
}

impl Calibrator for &str {
    fn calibration_value(&self) -> anyhow::Result<i64> {
        let mut numbers_first_positions: Vec<(&str, Option<usize>)> = NUMBERS
            .keys()
            .map(|number| (*number, self.find(number)))
            .filter(|(_, pos)| pos.is_some())
            .collect();

        numbers_first_positions.sort_by(|(_, pos1), (_, pos2)| pos1.cmp(pos2));
        let first_number = numbers_first_positions
            .first()
            .context("no numbers found")?;

        let mut numbers_last_positions: Vec<(&str, Option<usize>)> = NUMBERS
            .keys()
            .map(|number| (*number, self.rfind(number)))
            .filter(|(_, pos)| pos.is_some())
            .collect();

        numbers_last_positions.sort_by(|(_, pos1), (_, pos2)| pos1.cmp(pos2));
        let last_number = numbers_last_positions.last().context("no numbers found")?;

        Ok(NUMBERS.get(first_number.0).context("invalid number")? * 10
            + NUMBERS.get(last_number.0).context("invalid number")?)
    }
}

impl Calibrator for Lines<'_> {
    fn calibration_value(&self) -> anyhow::Result<i64> {
        let mut total = 0;
        for line in self.clone() {
            total += line.calibration_value()?;
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::Calibrator;

    #[test]
    fn example1() {
        let sample = include_str!("samples/part1").to_string();
        assert_eq!(sample.lines().calibration_value().unwrap(), 142);
    }

    #[test]
    fn example2() {
        let sample = include_str!("samples/part2").to_string();
        assert_eq!(sample.lines().calibration_value().unwrap(), 281);
    }
}
