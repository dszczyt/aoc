use anyhow::Context;
use core::str::Lines;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = include_str!("inputs/part1").to_string();
    println!("{}", input.lines().calibration_value()?);
    Ok(())
}

pub trait Calibrator {
    fn calibration_value(&self) -> anyhow::Result<i64>;
}

impl Calibrator for &str {
    fn calibration_value(&self) -> anyhow::Result<i64> {
        let first_char = self
            .chars()
            .find(|c| c.is_numeric())
            .context("can't find a numeric charactor")?;
        let last_char = self
            .chars()
            .rev()
            .find(|c| c.is_numeric())
            .context("can't find a numeric character")?;

        Ok(first_char.to_digit(10).unwrap() as i64 * 10 + last_char.to_digit(10).unwrap() as i64)
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
}
