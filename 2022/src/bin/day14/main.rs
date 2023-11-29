use std::{collections::HashMap, ops::RangeInclusive};

use itertools::Itertools; // for the `collect_tuple`

#[derive(PartialEq, Debug, Clone)]
pub enum Square {
    Block,
    Sand,
}

#[derive(Debug, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let (x, y): (usize, usize) = value
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub from: Coord,
    pub to: Coord,
}

impl From<(Coord, Coord)> for Path {
    fn from((from, to): (Coord, Coord)) -> Self {
        Self { from, to }
    }
}

impl Path {
    pub fn x_iter(&self) -> RangeInclusive<usize> {
        if self.from.x < self.to.x {
            self.from.x..=self.to.x
        } else {
            self.to.x..=self.from.x
        }
    }

    pub fn y_iter(&self) -> RangeInclusive<usize> {
        if self.from.y < self.to.y {
            self.from.y..=self.to.y
        } else {
            self.to.y..=self.from.y
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Map {
    pub board: HashMap<usize, Vec<Option<Square>>>,
    pub min_x: usize,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn get_at(&self, x: usize, y: usize) -> Option<Square> {
        self.board.get(&x)?.get(y)?.clone()
    }

    pub fn put(&mut self, x: usize, y: usize, square_type: Square) {
        if x < self.min_x {
            self.min_x = x;
        }
        if x + 1 > self.width {
            self.width = x + 1;
        }
        if square_type != Square::Sand && y + 1 > self.height {
            self.height = y + 1;
        }
        if let Some(col) = self.board.get_mut(&x) {
            if let Some(_) = col.get_mut(y) {
                let mut new_col: Vec<Option<Square>> = col.iter().take(y).cloned().collect(); // first_part.clone().to_vec();
                new_col.push(Some(square_type));
                new_col.append(&mut col.iter().skip(y + 1).cloned().collect());
                self.board.remove(&x);
                self.board.insert(x, new_col);
            } else {
                (col.len()..y).for_each(|_| col.push(None));
                col.push(Some(square_type));
            }
        } else {
            let mut col = vec![];
            (0..y).for_each(|_| col.push(None));
            col.push(Some(square_type));
            self.board.insert(x, col);
        }
    }

    pub fn print(&self) {
        (0..self.height + 15).for_each(|y| {
            (self.min_x..self.width + 5).for_each(|x| match self.get_at(x, y) {
                None => print!("."),
                Some(square) => match square {
                    Square::Block => print!("#"),
                    Square::Sand => print!("o"),
                },
            });
            print!("\n");
        });
        print!("\n");
    }

    pub fn produce_sand(&mut self, x: usize) -> Option<()> {
        let mut x = x;
        let mut y = 0;
        loop {
            if let Some(col) = self.board.get(&x) {
                if let Some(below) = col.get(y + 1) {
                    match below {
                        Some(_) => {
                            if let Some(left) = self.board.get(&(x - 1)) {
                                if let Some(left_square) = left.get(y + 1) {
                                    match left_square {
                                        Some(_) => {
                                            if let Some(right) = self.board.get(&(x + 1)) {
                                                if let Some(right_square) = right.get(y + 1) {
                                                    match right_square {
                                                        Some(_) => {
                                                            let mut new_col: Vec<Option<Square>> =
                                                                col.iter()
                                                                    .take(y)
                                                                    .cloned()
                                                                    .collect();
                                                            new_col.push(Some(Square::Sand));
                                                            new_col.append(
                                                                &mut col
                                                                    .iter()
                                                                    .skip(y + 1)
                                                                    .cloned()
                                                                    .collect(),
                                                            );
                                                            self.board.remove(&x);
                                                            self.board.insert(x, new_col);
                                                            return Some(());
                                                        }
                                                        None => x += 1,
                                                    }
                                                } else {
                                                    break;
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                        None => x -= 1,
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        None => y += 1,
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        return None;
    }

    pub fn produce_sand2(&mut self, x: usize) -> Option<()> {
        let mut x = x;
        let mut y = 0;
        {
            if let Some(col) = self.board.get(&x) {
                if let Some(square) = col.get(0) {
                    if let Some(square) = square {
                        if square.clone() == Square::Sand {
                            return None;
                        }
                    }
                }
            }
        }
        loop {
            if let Some(col) = self.board.get(&x) {
                if let Some(below) = col.get(y + 1) {
                    match below {
                        Some(_) => {
                            if let Some(left) = self.board.get(&(x - 1)) {
                                if let Some(left_square) = left.get(y + 1) {
                                    match left_square {
                                        Some(_) => {
                                            if let Some(right) = self.board.get(&(x + 1)) {
                                                if let Some(right_square) = right.get(y + 1) {
                                                    match right_square {
                                                        Some(_) => {
                                                            self.put(x, y, Square::Sand);
                                                            return Some(());
                                                        }
                                                        None => x += 1,
                                                    }
                                                } else {
                                                    x += 1;
                                                    y += 1;
                                                    if y == self.height {
                                                        self.put(x, y, Square::Sand);
                                                        return Some(());
                                                    }
                                                    // dbg!("break2");
                                                    // break;
                                                }
                                            } else {
                                                let new_col: Vec<Option<Square>> =
                                                    (0..self.height).map(|_| None).collect();
                                                self.board.insert(x + 1, new_col);
                                                x += 1;
                                                y += 1;
                                                if y == self.height {
                                                    self.put(x, y, Square::Sand);
                                                    return Some(());
                                                }
                                            }
                                        }
                                        None => x -= 1,
                                    }
                                } else {
                                    x -= 1;
                                    y += 1;
                                    if y == self.height {
                                        self.put(x, y, Square::Sand);
                                        return Some(());
                                    }

                                    // dbg!("break1");
                                    // break;
                                }
                            } else {
                                let new_col: Vec<Option<Square>> =
                                    (0..self.height).map(|_| None).collect();
                                // new_col.push(Some(Square::Block));
                                self.board.insert(x - 1, new_col);
                                x -= 1;
                                y += 1;
                                if y == self.height {
                                    self.put(x, y, Square::Sand);
                                    return Some(());
                                }

                                // break;
                            }
                        }
                        None => y += 1,
                    }
                } else {
                    // dbg!(&x, &y);
                    if y >= self.height {
                        self.put(x, y, Square::Sand);
                        return Some(());
                    } else {
                        y += 1;
                    }
                }
            } else {
                dbg!("break3");
                break;
            }
        }
        return None;
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let board = HashMap::<usize, Vec<Option<Square>>>::new();
        let mut map = Self {
            board,
            min_x: 500,
            ..Default::default()
        };
        value.lines().for_each(|line| {
            line.split("->")
                .map(|part| part.trim())
                .map(Into::<Coord>::into)
                .tuple_windows::<(_, _)>()
                .map(Into::<Path>::into)
                .for_each(|path| {
                    path.x_iter()
                        .for_each(|x| path.y_iter().for_each(|y| map.put(x, y, Square::Block)))
                });
        });
        map
    }
}

fn main() {
    let input = include_str!("input");
    let mut map: Map = input.into();

    {
        let mut map = map.clone();
        let mut i = 0;
        loop {
            if map.produce_sand(500).is_none() {
                break;
            }
            i += 1;
        }
        map.print();
        dbg!(&i);
    }

    {
        let mut map = map.clone();
        let mut i = 0;
        loop {
            if map.produce_sand2(500).is_none() {
                break;
            }
            i += 1;
        }
        map.print();
        dbg!(&i);
    }
}
