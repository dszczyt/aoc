use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Debug, Clone)]
pub enum Square {
    Beacon,
    Nothing,
}

#[derive(Debug, Clone)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn add(&self, v: (isize, isize)) -> Coord {
        Coord {
            x: self.x + v.0,
            y: self.y + v.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sensor {
    pub coord: Coord,
}

#[derive(Debug, Clone)]
pub struct Beacon {
    pub coord: Coord,
}

#[derive(Debug, Clone)]
pub struct SignalRange {
    pub start: isize,
    pub end: isize,
}

impl SignalRange {
    pub fn range(&self) -> RangeInclusive<isize> {
        self.start..=self.end
    }

    pub fn contains(&self, item: &isize) -> bool {
        self.range().contains(item)
    }
}

#[derive(Debug, Default)]
pub struct Row {
    pub beacons: HashSet<isize>,
    pub ranges: Vec<SignalRange>,
}

impl Row {
    pub fn add_range(&mut self, other: &SignalRange) {
        for mut r in &mut self.ranges {
            if r.start <= other.start && r.end >= other.end {
                return;
            }
            if r.start >= other.start && r.end <= other.end {
                r.start = other.start;
                r.end = other.end;
                return;
            }
            if r.contains(&other.start) {
                r.end = r.end.max(other.end);
                return;
            }
            if r.contains(&other.end) {
                r.start = r.start.min(other.start);
                return;
            }
        }
        self.ranges.push(other.clone());
    }

    pub fn remove_overlaps(&mut self) {
        let ranges = self.ranges.clone();
        self.ranges = vec![];
        for range in ranges {
            self.add_range(&range);
        }
    }

    pub fn count(&mut self) -> usize {
        self.remove_overlaps();
        self.ranges.iter().map(|r| r.end - r.start).sum::<isize>() as usize - self.beacons.len() + 1
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub sensor: Sensor,
    pub closest_beacon: Beacon,
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
            )
            .unwrap();
        }
        let captures = RE.captures(value).unwrap();

        Self {
            sensor: Sensor {
                coord: Coord {
                    x: captures[1].parse().unwrap(),
                    y: captures[2].parse().unwrap(),
                },
            },
            closest_beacon: Beacon {
                coord: Coord {
                    x: captures[3].parse().unwrap(),
                    y: captures[4].parse().unwrap(),
                },
            },
        }
    }
}

impl Report {
    pub fn mark(&self, row_num: isize, map: &mut HashMap<isize, Row>) {
        let distance = self.sensor.coord.distance(&self.closest_beacon.coord);
        let max_x = distance - (row_num - self.sensor.coord.y).abs();
        if max_x <= 0 {
            return;
        }
        let range = SignalRange {
            start: self.sensor.coord.x - max_x,
            end: self.sensor.coord.x + max_x,
        };

        dbg!(&range);

        match map.get_mut(&row_num) {
            Some(row) => {
                row.add_range(&range);
                if row_num == self.closest_beacon.coord.y {
                    row.beacons.insert(self.closest_beacon.coord.x);
                }
            }
            None => {
                let mut row = Row {
                    beacons: HashSet::new(),
                    ranges: vec![range],
                };
                if row_num == self.closest_beacon.coord.y {
                    row.beacons.insert(self.closest_beacon.coord.x);
                }
                map.insert(row_num, row);
            }
        };
    }
}

pub fn get_at(
    map: &mut HashMap<isize, HashMap<isize, Option<Square>>>,
    coord: Coord,
) -> Option<Square> {
    map.get(&coord.y)?.get(&coord.x)?.clone()
}

fn main() {
    let row = 2000000;

    let input = include_str!("input");
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        let report: Report = line.into();
        report.mark(row, &mut map);
    });

    dbg!(map.get_mut(&row).unwrap().count());
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{Report, Square};

    #[test]
    fn test_report() {
        let report: Report = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".into();
        assert_eq!(report.sensor.coord.x, 2);
        assert_eq!(report.sensor.coord.y, 18);
        assert_eq!(report.closest_beacon.coord.x, -2);
        assert_eq!(report.closest_beacon.coord.y, 15);
    }

    #[test]
    fn test_example1() {
        let row = 10;

        let input = include_str!("example");
        let mut map = HashMap::new();
        input.lines().for_each(|line| {
            let report: Report = line.into();
            report.mark(row, &mut map);
        });
        assert_eq!(map.get_mut(&row).unwrap().count(), 26);
    }
}
