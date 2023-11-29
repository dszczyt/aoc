use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub(crate) struct Item {
    pub value: u64,
}

impl Item {
    pub(crate) fn inspect(&mut self, monkey: &Monkey, worry_levels_divider: u64) -> usize {
        self.value = monkey.operation.run(&self.value);
        self.value /= worry_levels_divider;
        if self.value % monkey.test_divisible_by == 0 {
            monkey.if_true_throw_to
        } else {
            monkey.if_false_throw_to
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperationType {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
pub(crate) struct Operation {
    pub value: Option<u64>,
    pub operation_type: OperationType,
}

impl Operation {
    fn run(&self, old: &u64) -> u64 {
        match self.operation_type {
            OperationType::Add => old + self.value.unwrap_or(*old),
            OperationType::Mul => old * self.value.unwrap_or(*old),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Monkey {
    pub items: Vec<Item>,
    pub activity: usize,
    pub operation: Operation,
    pub test_divisible_by: u64,
    pub if_true_throw_to: usize,
    pub if_false_throw_to: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Monkeys {
    pub monkeys: Vec<Rc<RefCell<Monkey>>>,
}

impl From<&str> for Monkeys {
    fn from(input: &str) -> Self {
        let monkeys = input
            .lines()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, line)| {
                if i % 7 == 0 {
                    acc.push(vec![line]);
                } else {
                    let last = acc.last_mut().unwrap();
                    last.push(line);
                }
                acc
            })
            .iter()
            .map(|lines| {
                let items = lines
                    .get(1)
                    .unwrap()
                    .split(":")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .split(",")
                    .map(|s| Item {
                        value: u64::from_str_radix(s.trim(), 10).unwrap(),
                    })
                    .collect();

                let operation_parts: Vec<&str> = lines.get(2).unwrap().rsplitn(3, " ").collect();
                let operation_type;
                if operation_parts.get(1).unwrap() == &"+" {
                    operation_type = OperationType::Add;
                } else {
                    operation_type = OperationType::Mul;
                }
                let operation = Operation {
                    value: u64::from_str_radix(operation_parts.get(0).unwrap(), 10)
                        .map_or(None, |op| Some(op)),
                    operation_type,
                };

                Rc::new(RefCell::new(Monkey {
                    items,
                    activity: 0,
                    operation,
                    test_divisible_by: u64::from_str_radix(
                        lines
                            .get(3)
                            .unwrap()
                            .rsplitn(2, " ")
                            .collect::<Vec<&str>>()
                            .get(0)
                            .unwrap()
                            .trim(),
                        10,
                    )
                    .unwrap(),
                    if_true_throw_to: usize::from_str_radix(
                        lines
                            .get(4)
                            .unwrap()
                            .rsplitn(2, " ")
                            .collect::<Vec<&str>>()
                            .get(0)
                            .unwrap()
                            .trim(),
                        10,
                    )
                    .unwrap(),
                    if_false_throw_to: usize::from_str_radix(
                        lines
                            .get(5)
                            .unwrap()
                            .rsplitn(2, " ")
                            .collect::<Vec<&str>>()
                            .get(0)
                            .unwrap()
                            .trim(),
                        10,
                    )
                    .unwrap(),
                }))
            })
            .collect();

        Self { monkeys }
    }
}

impl Monkeys {
    pub(crate) fn round(&mut self, worry_levels_divider: u64) {
        let mut multipliers: Vec<u64> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.borrow().test_divisible_by)
            .collect();
        multipliers.dedup();
        let ppcm = multipliers.iter().fold(1, |acc, n| acc * n);
        self.monkeys.iter().for_each(|monkey| {
            let monkey = &mut monkey.borrow_mut();
            monkey.items.iter().for_each(|item| {
                let mut item = item.clone();
                let target_monkey = item.inspect(monkey, worry_levels_divider);
                let mut target_monkey = self.monkeys.get(target_monkey).unwrap().borrow_mut();

                item.value = item.value % ppcm;
                target_monkey.items.push(item);
            });
            monkey.activity += monkey.items.len();
            monkey.items = vec![];
        })
    }

    pub(crate) fn business(&self) -> usize {
        let mut activities: Vec<usize> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.borrow().activity)
            .collect();
        activities.sort();
        let activities: Vec<&usize> = activities.iter().rev().take(2).collect();
        *activities.get(0).unwrap() * *activities.get(1).unwrap()
    }
}

pub(crate) fn main() {
    let input = include_str!("input");
    let mut monkeys: Monkeys = input.into();
    (0..20).for_each(|_| {
        monkeys.round(3);
    });
    dbg!(&monkeys);

    dbg!(monkeys.business());

    let mut monkeys: Monkeys = input.into();
    (0..10000).for_each(|_| {
        monkeys.round(1);
    });

    dbg!(monkeys.business());
}

#[cfg(test)]
mod test {
    use crate::Monkeys;

    #[test]
    fn test_part1() {
        let input = include_str!("ex_part1");
        let mut monkeys: Monkeys = input.into();
        (0..20).for_each(|_| {
            monkeys.round(3);
        });
        dbg!(&monkeys);

        assert_eq!(monkeys.business(), 10605);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("ex_part1");
        let mut monkeys: Monkeys = input.into();
        (0..10000).for_each(|_| {
            monkeys.round(1);
        });

        assert_eq!(monkeys.business(), 2713310158);
    }
}
