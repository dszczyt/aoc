use std::collections::HashMap;

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
    pub fn mark(&self, row: isize, map: &mut HashMap<isize, HashMap<isize, Square>>) {
        if self.closest_beacon.coord.y == row {
            if let Some(row) = map.get_mut(&self.closest_beacon.coord.y) {
                if let Some(_) = row.get(&self.closest_beacon.coord.x) {
                    row.remove(&self.closest_beacon.coord.x);
                }
                row.insert(self.closest_beacon.coord.x, Square::Beacon);
            }
        }
        let distance = self.sensor.coord.distance(&self.closest_beacon.coord);
        let tmp = (distance - (row - self.sensor.coord.y));
        dbg!(&distance, &tmp);
        (-distance..=distance)
            .map(|d1| vec![d1, row - self.sensor.coord.y])
            .filter(|x| (x.get(0).unwrap().abs() + x.get(1).unwrap().abs()) <= distance)
            .map(|x| (x.get(0).unwrap().clone(), x.get(1).unwrap().clone()))
            .map(|v| self.sensor.coord.add(v))
            .for_each(|coord| {
                if let Some(row) = map.get_mut(&coord.y) {
                    if row.get(&coord.x).is_none() {
                        row.insert(coord.x, Square::Nothing);
                    }
                } else {
                    let mut row = HashMap::new();
                    row.insert(coord.x, Square::Nothing);
                    map.insert(coord.y, row);
                }
            });
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

    // let y_range: (isize, isize) = map
    //     .keys()
    //     .minmax()
    //     .into_option()
    //     .map(|x| (x.0.clone(), x.1.clone()))
    //     .unwrap();

    // let x_range: (isize, isize) = map
    //     .values()
    //     .map(|x| x.keys().minmax())
    //     .map(|x| x.into_option().unwrap())
    //     .fold((0 as isize, 0 as isize), |x, z| {
    //         (x.0.min(z.0.clone()), x.1.max(z.1.clone()))
    //     });
    // dbg!(&x_range, &y_range);

    // (y_range.0..=y_range.1).for_each(|y| {
    //     (x_range.0..=x_range.1).for_each(|x| match map.get(&y) {
    //         Some(row) => match row.get(&x) {
    //             Some(square) => match square {
    //                 Square::Beacon => print!("B"),
    //                 Square::Nothing => print!("#"),
    //             },
    //             None => print!("."),
    //         },
    //         None => print!("."),
    //     });
    //     print!("\n");
    // });
    // print!("\n");

    dbg!(map
        .get(&row)
        .unwrap()
        .values()
        .cloned()
        .filter(|elt| {
            return elt.clone() == Square::Nothing;
        })
        .count());
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
        assert_eq!(
            map.get(&row)
                .unwrap()
                .values()
                .cloned()
                .filter(|elt| {
                    return elt.clone() == Square::Nothing;
                })
                .count(),
            26
        );
    }
}
