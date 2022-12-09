use std::{cell::RefCell, rc::Rc};

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

#[derive(Debug)]
pub struct Move {
    pub dir: Dir,
    pub length: usize,
}

impl Move {
    pub fn apply(&self, board: &mut Board) {
        (0..self.length).for_each(|_| self.single_apply(board));
    }

    pub fn single_apply(&self, board: &mut Board) {
        match self.dir {
            Dir::Right => board.current_head.x += 1,
            Dir::Up => board.current_head.y += 1,
            Dir::Left => board.current_head.x -= 1,
            Dir::Down => board.current_head.y -= 1,
        }

        if board.current_tail.x < board.current_head.x - 1 {
            board.current_tail.x += 1;
            if board.current_tail.y != board.current_head.y {
                board.current_tail.y = board.current_head.y;
            }
        } else if board.current_tail.x > board.current_head.x + 1 {
            board.current_tail.x -= 1;
            if board.current_tail.y != board.current_head.y {
                board.current_tail.y = board.current_head.y;
            }
        }

        if board.current_tail.y < board.current_head.y - 1 {
            board.current_tail.y += 1;
            if board.current_tail.x != board.current_head.x {
                board.current_tail.x = board.current_head.x;
            }
        } else if board.current_tail.y > board.current_head.y + 1 {
            board.current_tail.y -= 1;
            if board.current_tail.x != board.current_head.x {
                board.current_tail.x = board.current_head.x;
            }
        }

        board.create_head_square_if_needed();
        board.create_tail_square_if_needed();
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

#[derive(Default, Debug)]
pub struct Square {
    pub x: isize,
    pub y: isize,
    pub visited: bool,
}

#[derive(Debug, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Default, Debug)]
pub struct Board {
    pub squares: Vec<Rc<RefCell<Square>>>,
    pub current_head: Coord,
    pub current_tail: Coord,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: vec![Rc::new(RefCell::new(Square::default()))],
            ..Default::default()
        }
    }

    pub fn create_square_if_needed(&mut self, x: isize, y: isize) {
        if self
            .squares
            .iter()
            .find(|square| square.borrow().x == x && square.borrow().y == y)
            .is_none()
        {
            self.squares.push(Rc::new(RefCell::new(Square {
                x,
                y,
                ..Default::default()
            })))
        }
    }

    pub fn create_head_square_if_needed(&mut self) {
        self.create_square_if_needed(self.current_head.x, self.current_head.y);
    }

    pub fn create_tail_square_if_needed(&mut self) {
        self.create_square_if_needed(self.current_tail.x, self.current_tail.y);
        self.get_square(self.current_tail.x, self.current_tail.y)
            .unwrap()
            .borrow_mut()
            .visited = true;
    }

    pub fn get_square(&self, x: isize, y: isize) -> Option<&Rc<RefCell<Square>>> {
        self.squares
            .iter()
            .find(|square| square.borrow().x == x && square.borrow().y == y)
    }

    pub fn get_head(&self) -> &Rc<RefCell<Square>> {
        self.get_square(self.current_head.x, self.current_head.y)
            .unwrap()
    }

    pub fn get_tail(&self) -> &Rc<RefCell<Square>> {
        self.get_square(self.current_tail.x, self.current_tail.y)
            .unwrap()
    }

    pub fn visited_squares(&self) -> Vec<&Rc<RefCell<Square>>> {
        self.squares
            .iter()
            .filter(|square| square.borrow().visited)
            .collect()
    }
}

pub fn walk(input: &str, board: &mut Board) {
    let moves: Vec<Move> = input.lines().map(Into::into).collect();
    moves.iter().for_each(|m| m.apply(board));
}

fn main() {
    let input = include_str!("input");
    let mut board = Board::new();
    walk(input, &mut board);
    dbg!(board.visited_squares().len());
}

#[cfg(test)]
mod test {
    use crate::{walk, Board, Square};

    #[test]
    fn part1() {
        let input = include_str!("example");
        let mut board = Board::new();
        walk(input, &mut board);
        assert_eq!(board.visited_squares().len(), 13);
    }
}
