use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Debug, Clone, Copy)]
pub(crate) struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Coordinates {
    pub fn add(&self, (x, y): (isize, isize)) -> Coordinates {
        Coordinates {
            x: (self.x as isize + x) as usize,
            y: (self.y as isize + y) as usize,
        }
    }
}

const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);

#[derive(Debug)]
pub(crate) struct Square {
    pub elevation: usize,
    pub coordinates: Coordinates,
    pub weight: Option<usize>,
}

impl Square {
    pub fn squares_around<'a>(&'a self, map: &'a Map) -> Vec<Rc<RefCell<Square>>> {
        let mut squares = vec![];

        // fetch square up
        if self.coordinates.y != 0 {
            let coordinates = self.coordinates.add(UP);
            if let Some(square) = map.square_at(coordinates) {
                if (0..=self.elevation + 1).contains(&(&*square).borrow().elevation) {
                    squares.push(square);
                }
            }
        }

        // fetch square down
        let coordinates = self.coordinates.add(DOWN);
        if let Some(square) = map.square_at(coordinates) {
            if (0..=self.elevation + 1).contains(&(&*square).borrow().elevation) {
                squares.push(square);
            }
        }

        // fetch square left
        if self.coordinates.x != 0 {
            let coordinates = self.coordinates.add(LEFT);
            if let Some(square) = map.square_at(coordinates) {
                if (0..=self.elevation + 1).contains(&(&*square).borrow().elevation) {
                    squares.push(square);
                }
            }
        }

        // fetch square right
        let coordinates = self.coordinates.add(RIGHT);
        if let Some(square) = map.square_at(coordinates) {
            if (0..=self.elevation + 1).contains(&(&*square).borrow().elevation) {
                squares.push(square);
            }
        }

        squares
    }
}

#[derive(Clone)]
pub(crate) struct Map {
    pub width: usize,
    pub height: usize,
    pub start: Coordinates,
    pub end: Coordinates,
    pub squares: Vec<Rc<RefCell<Square>>>,
}

impl Map {
    pub fn fewest_steps(&self) -> usize {
        self.cross_forward(0, self.start);

        self.square_at(self.end)
            .unwrap()
            .borrow_mut()
            .weight
            .unwrap()
    }

    pub fn fewest_steps_part2(&self) -> usize {
        let starts: Vec<Coordinates> = (0..40).map(|y| Coordinates { x: 0, y }).collect();

        starts
            .iter()
            .cloned()
            .map(|start| {
                self.clone().cross_forward(0, start);

                self.square_at(self.end)
                    .unwrap()
                    .borrow_mut()
                    .weight
                    .unwrap()
            })
            .min()
            .unwrap()
    }

    pub fn square_at(&self, coordinates: Coordinates) -> Option<Rc<RefCell<Square>>> {
        if coordinates.x >= self.width {
            return None;
        }
        if coordinates.y >= self.height {
            return None;
        }
        self.squares
            .get(coordinates.y * self.width + coordinates.x)
            .map(Clone::clone)
    }

    pub(crate) fn cross_forward(&self, weight: usize, coordinates: Coordinates) {
        {
            let square = self.square_at(coordinates).unwrap();
            let mut square = square.borrow_mut();

            if square.weight.unwrap_or(usize::MAX) <= weight {
                return;
            }

            square.weight = Some(weight);
        }

        let around_coordinates;
        {
            let square = self.square_at(coordinates).unwrap().clone();
            let square = (&*square).borrow();

            around_coordinates = square.squares_around(self);
        }

        let coordinates: Vec<Coordinates>;
        {
            coordinates = around_coordinates
                .iter()
                .cloned()
                .map(|square| {
                    let square = (&*square).borrow();
                    square.coordinates
                })
                .collect();
        }

        coordinates.iter().cloned().for_each(|coordinates| {
            self.cross_forward(weight + 1, coordinates);
        });
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut start = Coordinates { x: 0, y: 0 };
        let mut end = Coordinates { x: 0, y: 0 };

        let squares = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                height += 1;
                width = line.len();
                line.char_indices()
                    .map(|(x, c)| {
                        let elevation = match c {
                            c @ ('a'..='z') => c as usize - 'a' as usize,
                            'S' => {
                                start = Coordinates { x, y };
                                0
                            }
                            'E' => {
                                end = Coordinates { x, y };
                                25
                            }
                            _ => unreachable!(),
                        };
                        Rc::new(RefCell::new(Square {
                            elevation,
                            coordinates: Coordinates { x, y },
                            weight: None,
                        }))
                    })
                    .collect::<Vec<Rc<RefCell<Square>>>>()
            })
            .flatten()
            .collect();

        Self {
            squares,
            width,
            height,
            start,
            end,
        }
    }
}

fn main() {
    let input = include_str!("input");
    let map: Map = input.into();

    map.cross_forward(0, map.start);

    map.squares
        .iter()
        .cloned()
        .enumerate()
        .for_each(|(i, square)| {
            let square = (&*square).borrow();

            if i > 0 && i % 113 == 0 {
                print!("\n");
            }
            if i % 113 == 0 {
                print!("{:>2} ", i / 113);
            }

            if square.coordinates == map.start {
                print!("S  ");
            } else if square.coordinates == map.end {
                print!("E  ");
            } else if square.weight.is_some() {
                print!("#{} ", (square.elevation as u8 + 'a' as u8) as char);
            } else {
                print!(" {} ", (square.elevation as u8 + 'a' as u8) as char);
            }
        });
    print!("\n");
    dbg!(&map.fewest_steps());
    dbg!(&map.fewest_steps_part2());
}

#[cfg(test)]
mod test {
    use crate::Map;

    #[test]
    fn example_part1() {
        let input = include_str!("example_part1");
        let example_map: Map = input.into();
        assert_eq!(example_map.fewest_steps(), 31);
    }

    #[test]
    fn example_part2() {
        let input = include_str!("example_part1");
        let example_map: Map = input.into();
        assert_eq!(example_map.fewest_steps(), 29);
    }
}
