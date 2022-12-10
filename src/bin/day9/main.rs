use std::collections::HashSet;

#[derive(Debug)]
pub enum Dir {
    Right,
    Up,
    Left,
    Down,
}

impl From<&str> for Dir {
    fn from(input: &str) -> Self {
        match input {
            "R" => Self::Right,
            "U" => Self::Up,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => unreachable!(),
        }
    }
}

impl From<&Dir> for Coord {
    fn from(dir: &Dir) -> Self {
        match dir {
            Dir::Right => Coord { x: -1, y: 0 },
            Dir::Up => Coord { x: 0, y: -1 },
            Dir::Left => Coord { x: 1, y: 0 },
            Dir::Down => Coord { x: 0, y: 1 },
        }
    }
}

impl Dir {
    pub fn apply(&self, board: &mut Board) {
        let rope = &mut board.rope;
        let rope_len = rope.len();
        rope.get_mut(0).unwrap().add(&self.into());

        for i in 1..rope_len {
            if rope.get(i - 1).unwrap().dist(rope.get(i).unwrap()) > 1.5 {
                let mut x = rope.get(i - 1).unwrap().x - rope.get(i).unwrap().x;
                if x != 0 {
                    x = x / x.abs();
                }

                let mut y = rope.get(i - 1).unwrap().y - rope.get(i).unwrap().y;
                if y != 0 {
                    y = y / y.abs();
                }

                rope.get_mut(i).unwrap().add(&Coord { x, y });
            }
        }

        board.create_tail_square_if_needed();
    }
}

#[derive(Debug)]
pub struct Move {
    pub dir: Dir,
    pub length: usize,
}

impl Move {
    pub fn apply(&self, board: &mut Board) {
        (0..self.length).for_each(|_| self.dir.apply(board));
    }
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let (dir, length) = input.split_once(" ").unwrap();
        Self {
            dir: dir.into(),
            length: usize::from_str_radix(length, 10).unwrap(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Square {
    pub x: isize,
    pub y: isize,
    // pub visited: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn dist(&self, other: &Self) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }

    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Default, Debug)]
pub struct Board {
    pub squares: HashSet<Square>,
    pub rope: Vec<Coord>,
}

impl Board {
    pub fn new(length: usize) -> Board {
        let mut squares = HashSet::new();
        squares.insert(Square::default());
        Board {
            rope: (0..length).map(|_| Coord::default()).collect(),
            squares,
        }
    }

    pub fn create_square_if_needed(&mut self, x: isize, y: isize) {
        if !self.squares.contains(&Square { x, y }) {
            self.squares.insert(Square { x, y });
        }
    }

    pub fn create_tail_square_if_needed(&mut self) {
        let len = self.rope.len();
        let square = self.rope.get(len - 1).unwrap();
        self.create_square_if_needed(square.x, square.y);
    }

    pub fn get_square(&self, x: isize, y: isize) -> Option<&Square> {
        self.squares
            .iter()
            .find(|square| square.x == x && square.y == y)
    }

    pub fn show(&self, size: isize) {
        (0..size).rev().for_each(|row| {
            (0..size).for_each(|col| {
                match self
                    .rope
                    .iter()
                    .enumerate()
                    .find(|(_, square)| square.x == col && square.y == row)
                {
                    None => print!("."),
                    Some((i, _)) => {
                        if i == 0 {
                            print!("H");
                        } else {
                            print!("{}", i);
                        }
                    }
                }
            });
            print!("\n")
        });
        print!("\n");
    }

    pub fn show_visited(&self) {
        let x = self.squares.iter().map(|square| square.x).max().unwrap();
        let y = self.squares.iter().map(|square| square.y).max().unwrap();

        (0..y).rev().for_each(|row| {
            (0..x).for_each(|col| {
                if self.squares.contains(&Square { x: row, y: col }) {
                    print!("#");
                } else {
                    print!(".");
                }
            });
            print!("\n")
        });
        print!("\n");
    }
}

pub fn walk(input: &str, board: &mut Board) {
    let moves: Vec<Move> = input.lines().map(Into::into).collect();
    moves.iter().for_each(|m| m.apply(board));
}

fn main() {
    let input = include_str!("input");
    let mut board = Board::new(2);
    walk(input, &mut board);
    // board.show_visited();
    dbg!(board.squares.len());

    let mut board = Board::new(10);
    walk(input, &mut board);
    // board.show_visited();
    dbg!(board.squares.len());
}

#[cfg(test)]
mod test {
    use crate::{walk, Board, Coord, Dir, Move};

    #[test]
    fn part1() {
        let input = include_str!("example");
        let mut board = Board::new(2);
        board.show(6);
        walk(input, &mut board);
        board.show(6);
        assert_eq!(board.squares.len(), 13);
    }

    #[test]
    fn part2() {
        let input = include_str!("example_part2");
        let mut board = Board::new(10);
        walk(input, &mut board);
        board.show(36);
        assert_eq!(board.squares.len(), 36);
    }

    #[test]
    fn part2_11() {
        let mut board = Board::new(10);
        board.rope = vec![
            Coord { x: 3, y: 4 },
            Coord { x: 4, y: 3 },
            Coord { x: 4, y: 2 },
            Coord { x: 3, y: 2 },
            Coord { x: 2, y: 2 },
            Coord { x: 1, y: 1 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
        ];
        board.show(6);
        Dir::Left.apply(&mut board);
        board.show(6);
        assert_eq!(board.rope.len(), 10);
        assert_eq!(board.rope.get(0).unwrap(), &Coord { x: 2, y: 4 });
        assert_eq!(board.rope.get(1).unwrap(), &Coord { x: 3, y: 4 });
        assert_eq!(board.rope.get(2).unwrap(), &Coord { x: 3, y: 3 });
        assert_eq!(board.rope.get(3).unwrap(), &Coord { x: 3, y: 2 });
        assert_eq!(board.rope.get(4).unwrap(), &Coord { x: 2, y: 2 });
        assert_eq!(board.rope.get(5).unwrap(), &Coord { x: 1, y: 1 });
        assert_eq!(board.rope.get(6).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(7).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(8).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(9).unwrap(), &Coord { x: 0, y: 0 });
    }

    #[test]
    fn part2_12() {
        let mut board = Board::new(10);
        board.rope = vec![
            Coord { x: 2, y: 4 },
            Coord { x: 3, y: 4 },
            Coord { x: 3, y: 3 },
            Coord { x: 3, y: 2 },
            Coord { x: 2, y: 2 },
            Coord { x: 1, y: 1 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
        ];
        board.show(27);
        Dir::Left.apply(&mut board);
        board.show(27);
        assert_eq!(board.rope.len(), 10);
        assert_eq!(board.rope.get(0).unwrap(), &Coord { x: 1, y: 4 });
        assert_eq!(board.rope.get(1).unwrap(), &Coord { x: 2, y: 4 });
        assert_eq!(board.rope.get(2).unwrap(), &Coord { x: 3, y: 3 });
        assert_eq!(board.rope.get(3).unwrap(), &Coord { x: 3, y: 2 });
        assert_eq!(board.rope.get(4).unwrap(), &Coord { x: 2, y: 2 });
        assert_eq!(board.rope.get(5).unwrap(), &Coord { x: 1, y: 1 });
        assert_eq!(board.rope.get(6).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(7).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(8).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(9).unwrap(), &Coord { x: 0, y: 0 });
    }

    #[test]
    fn part2_13() {
        let mut board = Board::new(10);
        board.rope = vec![
            Coord { x: 1, y: 4 },
            Coord { x: 2, y: 4 },
            Coord { x: 3, y: 3 },
            Coord { x: 3, y: 2 },
            Coord { x: 2, y: 2 },
            Coord { x: 1, y: 1 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 0 },
        ];
        board.show(6);
        Dir::Down.apply(&mut board);
        board.show(6);
        assert_eq!(board.rope.len(), 10);
        assert_eq!(board.rope.get(0).unwrap(), &Coord { x: 1, y: 3 });
        assert_eq!(board.rope.get(1).unwrap(), &Coord { x: 2, y: 4 });
        assert_eq!(board.rope.get(2).unwrap(), &Coord { x: 3, y: 3 });
        assert_eq!(board.rope.get(3).unwrap(), &Coord { x: 3, y: 2 });
        assert_eq!(board.rope.get(4).unwrap(), &Coord { x: 2, y: 2 });
        assert_eq!(board.rope.get(5).unwrap(), &Coord { x: 1, y: 1 });
        assert_eq!(board.rope.get(6).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(7).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(8).unwrap(), &Coord { x: 0, y: 0 });
        assert_eq!(board.rope.get(9).unwrap(), &Coord { x: 0, y: 0 });
    }

    #[test]
    fn part2_2_3() {
        let mut board = Board::new(10);
        board.rope = vec![
            Coord { x: 16, y: 5 },
            Coord { x: 15, y: 5 },
            Coord { x: 14, y: 5 },
            Coord { x: 13, y: 5 },
            Coord { x: 12, y: 5 },
            Coord { x: 11, y: 5 },
            Coord { x: 11, y: 5 },
            Coord { x: 11, y: 5 },
            Coord { x: 11, y: 5 },
            Coord { x: 11, y: 5 },
        ];
        board.show(20);
        Into::<Move>::into("U 8").apply(&mut board);
        board.show(20);
        assert_eq!(board.rope.len(), 10);
        assert_eq!(board.rope.get(0).unwrap(), &Coord { x: 16, y: 13 });
        assert_eq!(board.rope.get(1).unwrap(), &Coord { x: 16, y: 12 });
        assert_eq!(board.rope.get(2).unwrap(), &Coord { x: 16, y: 11 });
        assert_eq!(board.rope.get(3).unwrap(), &Coord { x: 16, y: 10 });
        assert_eq!(board.rope.get(4).unwrap(), &Coord { x: 16, y: 9 });
        assert_eq!(board.rope.get(5).unwrap(), &Coord { x: 15, y: 9 });
        assert_eq!(board.rope.get(6).unwrap(), &Coord { x: 14, y: 8 });
        assert_eq!(board.rope.get(7).unwrap(), &Coord { x: 13, y: 7 });
        assert_eq!(board.rope.get(8).unwrap(), &Coord { x: 12, y: 6 });
        assert_eq!(board.rope.get(9).unwrap(), &Coord { x: 11, y: 5 });
    }
}
