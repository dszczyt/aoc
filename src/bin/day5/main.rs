pub type Crate = String;

#[derive(Debug, Clone)]
pub struct Crane {
    pub stacks: Vec<Stack>,
}

impl From<&str> for Crane {
    fn from(input: &str) -> Self {
        let stacks = input.lines().map(|line| line.into()).collect();
        Self { stacks }
    }
}

impl Crane {
    pub fn message(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| String::from(stack.crates.last().unwrap().clone()))
            .collect::<Vec<String>>()
            .concat()
    }
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub crates: Vec<char>,
}

impl From<&str> for Stack {
    fn from(input: &str) -> Self {
        let crates = input.chars().collect();
        Self { crates }
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub count: usize,
    pub from_stack_id: usize,
    pub to_stack_id: usize,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let tokens: Vec<&str> = input.split(" ").collect();
        let count = usize::from_str_radix(tokens.get(1).unwrap().clone(), 10).unwrap();
        let from_stack_id = usize::from_str_radix(tokens.get(3).unwrap().clone(), 10).unwrap();
        let to_stack_id = usize::from_str_radix(tokens.get(5).unwrap().clone(), 10).unwrap();
        Self {
            count,
            from_stack_id,
            to_stack_id,
        }
    }
}

impl Move {
    pub fn run(&self, crane: &mut Crane) {
        for _ in 0..self.count {
            let from = crane.stacks.get_mut(self.from_stack_id - 1).unwrap();
            if let Some(cr) = from.crates.pop() {
                let to = crane.stacks.get_mut(self.to_stack_id - 1).unwrap();
                to.crates.push(cr);
            } else {
                dbg!(&crane);
                panic!("failed");
            }
        }
    }

    pub fn run_9001(&self, crane: &mut Crane) {
        let mut pick: Vec<char> = (0..self.count)
            .map(|_| {
                crane
                    .stacks
                    .get_mut(self.from_stack_id - 1)
                    .unwrap()
                    .crates
                    .pop()
                    .unwrap()
            })
            .collect();
        pick.reverse();
        let to = crane.stacks.get_mut(self.to_stack_id - 1).unwrap();
        to.crates.append(&mut pick);
    }
}

// fn get_message(input_init: &str, input_moves: &str) -> String {}

fn main() {
    let mut crane: Crane = include_str!("input_init").into();
    let moves: Vec<Move> = include_str!("input_moves")
        .lines()
        .map(|line| line.into())
        .collect();
    moves.iter().for_each(|m| {
        m.run_9001(&mut crane);
    });

    dbg!(&crane.message());
}

#[cfg(test)]
mod test {
    use crate::{Crane, Move};

    #[test]
    fn check_example_init() {
        let init = include_str!("example_init");
        let crane: Crane = init.into();
        assert_eq!(crane.stacks.len(), 3);
        assert_eq!(
            crane
                .stacks
                .get(0)
                .unwrap()
                .crates
                .iter()
                .map(|c| String::from(c.clone()))
                .collect::<Vec<String>>()
                .concat(),
            "ZN"
        );
        assert_eq!(
            crane
                .stacks
                .get(1)
                .unwrap()
                .crates
                .iter()
                .map(|c| String::from(c.clone()))
                .collect::<Vec<String>>()
                .concat(),
            "MCD"
        );
        assert_eq!(
            crane
                .stacks
                .get(2)
                .unwrap()
                .crates
                .iter()
                .map(|c| String::from(c.clone()))
                .collect::<Vec<String>>()
                .concat(),
            "P"
        );
    }

    #[test]
    fn init_move() {
        let m: Move = "move 12 from 42 to 36".into();
        assert_eq!(m.count, 12);
        assert_eq!(m.from_stack_id, 42);
        assert_eq!(m.to_stack_id, 36);
    }

    #[test]
    fn test_example() {
        let mut crane: Crane = include_str!("example_init").into();
        let moves: Vec<Move> = include_str!("example_moves")
            .lines()
            .map(|line| line.into())
            .collect();
        moves.iter().for_each(|m| {
            dbg!(&m);
            m.run(&mut crane);
        });
        assert_eq!(crane.message(), "CMZ");
    }

    #[test]
    fn test_example_9001() {
        let mut crane: Crane = include_str!("example_init").into();
        let moves: Vec<Move> = include_str!("example_moves")
            .lines()
            .map(|line| line.into())
            .collect();
        moves.iter().for_each(|m| {
            dbg!(&m);
            m.run_9001(&mut crane);
        });
        assert_eq!(crane.message(), "MCD");
    }
}
