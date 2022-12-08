#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Tree {
    pub height: usize,
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize, char)> for Tree {
    fn from((x, y, c): (usize, usize, char)) -> Self {
        Self {
            height: c.to_digit(10).unwrap() as usize,
            x,
            y,
            ..Default::default()
        }
    }
}

impl Tree {
    pub fn trees_to_the_edge<'a>(&'a self, direction: Direction, map: &'a Map) -> Vec<&Tree> {
        let mut trees = vec![];
        let mut current_tree = self;
        while let Some(tree) = map.get_tree(current_tree.x, current_tree.y, Some(direction)) {
            trees.push(tree);
            current_tree = tree;
        }
        trees
    }

    pub fn is_visible_from(&self, direction: Direction, map: &Map) -> bool {
        if let Some(highest_tree) = self
            .trees_to_the_edge(direction, map)
            .iter()
            .max_by(|tree_a, tree_b| usize::cmp(&tree_a.height, &tree_b.height))
        {
            highest_tree.height < self.height
        } else {
            true
        }
    }

    pub fn is_visible(&self, map: &Map) -> bool {
        self.is_visible_from(Direction::Top, map)
            || self.is_visible_from(Direction::Bottom, map)
            || self.is_visible_from(Direction::Left, map)
            || self.is_visible_from(Direction::Right, map)
    }
}

#[derive(Debug)]
pub struct Row {
    pub trees: Vec<Tree>,
}

impl From<(usize, &str)> for Row {
    fn from((y, input): (usize, &str)) -> Self {
        Self {
            trees: input
                .chars()
                .enumerate()
                .map(|(x, c)| (x, y, c).into())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub rows: Vec<Row>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        Self {
            rows: input.lines().enumerate().map(|line| line.into()).collect(),
        }
    }
}

impl Map {
    pub fn get_tree(&self, x: usize, y: usize, direction: Option<Direction>) -> Option<&Tree> {
        let (mut fetch_x, mut fetch_y) = (x, y);
        if let Some(direction) = direction {
            match direction {
                Direction::Top => {
                    if fetch_y == 0 {
                        return None;
                    }
                    fetch_y -= 1;
                }
                Direction::Bottom => {
                    if fetch_y == self.rows.len() - 1 {
                        return None;
                    }
                    fetch_y += 1;
                }
                Direction::Left => {
                    if fetch_x == 0 {
                        return None;
                    }
                    fetch_x -= 1;
                }
                Direction::Right => {
                    if fetch_x == self.rows.get(y).unwrap().trees.len() - 1 {
                        return None;
                    }
                    fetch_x += 1;
                }
            }
        }
        self.rows.get(fetch_y)?.trees.get(fetch_x)
    }

    pub fn visible_trees(&self) -> Vec<&Tree> {
        self.rows
            .iter()
            .map(|row| row.trees.iter().filter(|tree| tree.is_visible(self)))
            .flatten()
            .collect()
    }
}

fn main() {
    let map: Map = include_str!("input").into();
    dbg!(map.visible_trees().len());
}

#[cfg(test)]
mod test {
    use crate::{Direction, Map};

    #[test]
    fn part1_1() {
        let map: Map = include_str!("example").into();
        dbg!(&map);

        // The top-left 5 is visible from the left
        assert!(map
            .get_tree(1, 1, None)
            .unwrap()
            .is_visible_from(Direction::Left, &map));
    }

    #[test]
    fn part1_2() {
        let map: Map = include_str!("example").into();

        // The top-left 5 is visible from the top
        assert!(map
            .get_tree(1, 1, None)
            .unwrap()
            .is_visible_from(Direction::Top, &map));
    }

    #[test]
    fn part1_3() {
        let map: Map = include_str!("example").into();

        // The top-left 5 is visible from the top
        assert!(map
            .get_tree(2, 1, None)
            .unwrap()
            .is_visible_from(Direction::Top, &map));
    }

    #[test]
    fn part1_4() {
        let map: Map = include_str!("example").into();

        // The top-left 5 is visible from the right
        assert!(map
            .get_tree(2, 1, None)
            .unwrap()
            .is_visible_from(Direction::Right, &map));
    }

    #[test]
    fn part1_5() {
        let map: Map = include_str!("example").into();

        // The top-right 1 is not visible from any direction
        let tree = map.get_tree(3, 1, None).unwrap();
        assert!(!tree.is_visible_from(Direction::Right, &map));
        assert!(!tree.is_visible_from(Direction::Left, &map));
        assert!(!tree.is_visible_from(Direction::Top, &map));
        assert!(!tree.is_visible_from(Direction::Bottom, &map));
    }

    #[test]
    fn part1_6() {
        let map: Map = include_str!("example").into();

        // The left-middle 5 is visible, but only from the right.
        let tree = map.get_tree(1, 2, None).unwrap();
        assert!(tree.is_visible_from(Direction::Right, &map));
        assert!(!tree.is_visible_from(Direction::Left, &map));
        assert!(!tree.is_visible_from(Direction::Top, &map));
        assert!(!tree.is_visible_from(Direction::Bottom, &map));
    }

    #[test]
    fn part1_7() {
        let map: Map = include_str!("example").into();

        // The center 3 is not visible from any direction
        let tree = map.get_tree(2, 2, None).unwrap();
        assert!(!tree.is_visible_from(Direction::Right, &map));
        assert!(!tree.is_visible_from(Direction::Left, &map));
        assert!(!tree.is_visible_from(Direction::Top, &map));
        assert!(!tree.is_visible_from(Direction::Bottom, &map));
    }

    #[test]
    fn part1_8() {
        let map: Map = include_str!("example").into();

        // The right-middle 3 is visible from the right.
        let tree = map.get_tree(3, 2, None).unwrap();
        assert!(tree.is_visible_from(Direction::Right, &map));
        assert!(!tree.is_visible_from(Direction::Left, &map));
        assert!(!tree.is_visible_from(Direction::Top, &map));
        assert!(!tree.is_visible_from(Direction::Bottom, &map));
    }

    #[test]
    fn part1_10() {
        let map: Map = include_str!("example").into();

        // In the bottom row, the middle 5 is visible,
        let tree = map.get_tree(3, 2, None).unwrap();
        assert!(tree.is_visible_from(Direction::Right, &map));
        assert!(!tree.is_visible_from(Direction::Left, &map));
        assert!(!tree.is_visible_from(Direction::Top, &map));
        assert!(!tree.is_visible_from(Direction::Bottom, &map));
    }

    #[test]
    fn part1_9() {
        let map: Map = include_str!("example").into();
        assert_eq!(map.visible_trees().len(), 21);
    }
}
