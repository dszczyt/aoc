use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    rc::Rc,
};

//use rayon::prelude::*;

use derivative::Derivative;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Derivative, Default, Clone)]
#[derivative(Debug, PartialEq)]
pub struct Valve {
    pub name: String,
    #[derivative(PartialEq = "ignore")]
    pub flow_rate: usize,
    // pub open: bool,
    // #[derivative(Debug = "ignore", PartialEq = "ignore")]
    // pub tunnels: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Tunnel {
    pub from: Valve,
    pub to: Valve,
}

#[derive(Debug)]
pub struct Distance {
    pub from: Rc<RefCell<Valve>>,
    pub to: Rc<RefCell<Valve>>,
    pub distance: usize,
}

#[derive(Debug)]
pub struct Network {
    pub valves: Vec<Rc<RefCell<Valve>>>,
}

impl Network {
    pub fn open(&self) -> Vec<Rc<RefCell<Valve>>> {
        self.valves
            .iter()
            .filter(|&&valve| {
                let v: &Valve = valve.borrow();
                false
                // return valve.clone().borrow().open;
            })
            .cloned()
            .collect()
    }

    /*pub fn useful_and_open(&self) -> Vec<Rc<RefCell<Valve>>> {
        // dbg!(&self);
        self.valves
            .iter()
            .filter(|valve| valve.borrow().flow_rate > 0 && valve.borrow().open)
            .cloned()
            .collect()
    }

    pub fn useful_and_closed(&self) -> Vec<Rc<RefCell<Valve>>> {
        // dbg!(&self);
        self.valves
            .iter()
            .filter(|valve| valve.borrow().flow_rate > 0 && !valve.borrow().open)
            .cloned()
            .collect()
    }*/

    pub fn find_by_name(&self, name: String) -> Rc<RefCell<Valve>> {
        self.valves
            .iter()
            .find(|&valve| (&*valve).borrow().name == name)
            .unwrap()
            .clone()
    }

    pub fn release_pressure(&self) -> usize {
        self.open().iter().map(|&v| v.borrow().flow_rate).sum()
    }

    pub fn shortest_distances_to_closed_valves(&self, from: Rc<RefCell<Valve>>) {
        let mut distances: Vec<Distance> = vec![];

        let mut start = vec![from];
        let mut n = 1;
        loop {
            start
                .iter()
                .for_each(|&valve| valve.borrow().tunnels.iter().for_each(|valve_name| {}));
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Move(Rc<RefCell<Valve>>),
    Open(Rc<RefCell<Valve>>),
    Wait(Rc<RefCell<Valve>>),
}

#[derive(Debug, Default)]
pub struct Step {
    pub action: Option<Action>,
    pub total_released: usize,
    // pub next: Vec<Rc<RefCell<Self>>>,
}

impl Step {
    /*pub fn compute_next(
        &mut self,
        network: &Network,
        mut path: Vec<String>,
        mut open_valves: Vec<Rc<RefCell<Valve>>>,
        level: usize,
    ) -> usize {
        // dbg!(&level, &self, &open_valves);
        // let mut open_valves: Vec<Valve> = open_valves.clone(); //.iter().cloned().collect();

        // dbg!(&level, &network.useful_and_open());
        if level == 30 {
            // println!(
            //     "[{}] {} = {}",
            //     open_valves.iter().map(|v| v.name.clone()).join("+"),
            //     path.join("->"),
            //     self.total_released
            // );

            return self.total_released;
        }
        let action = self.action.as_ref().unwrap();
        let current_valve = match action {
            Action::Move(valve) | Action::Wait(valve) => valve,
            Action::Open(valve) => {
                // dbg!("opening valve", &valve.borrow().name);
                open_valves.push(valve.clone());
                /*let mut v = (&*valve).borrow_mut();
                v.open = true;*/
                valve
            }
        };

        let releasing = open_valves
            .iter()
            .map(|&valve| valve.borrow().flow_rate)
            .sum::<usize>();
        self.total_released += releasing;

        path.push(format!(
            "{}(+{})",
            current_valve.borrow().name.clone(),
            releasing
        ));

        let mut max_released = 0;

        if current_valve.borrow().flow_rate > 0
            && !open_valves
                .iter()
                .any(|v1| v1.borrow().name == current_valve.borrow().name)
        {
            let mut step2 = Step {
                action: Some(Action::Open(current_valve.clone())),
                total_released: self.total_released,
                // next: vec![],
            };
            max_released = max_released.max(step2.compute_next(
                network,
                path.clone(),
                open_valves.clone(),
                level + 1,
            ));
            // self.next.push(Rc::new(RefCell::new(step2)));
        } else if network.valves.iter().any(|v1| {
            v1.clone().borrow().flow_rate > 0
                && !open_valves
                    .iter()
                    .any(|v2| v2.borrow().name == v1.clone().borrow().name)
        }) {
            {
                max_released = current_valve
                    .borrow()
                    .tunnels
                    .iter()
                    .map(|name| {
                        let valve = network.find_by_name(name.clone());
                        let mut step1 = Step {
                            action: Some(Action::Move(valve.clone())),
                            total_released: self.total_released,
                            // next: vec![],
                        };
                        let released = step1.compute_next(
                            network,
                            path.clone(),
                            open_valves.clone(),
                            level + 1,
                        );
                        // self.next.push(Rc::new(RefCell::new(step1)));
                        released
                    })
                    .max()
                    .unwrap_or_default();
            }
        } else {
            let mut step3 = Step {
                action: Some(Action::Wait(current_valve.clone())),
                total_released: self.total_released,
                // next: vec![],
            };
            max_released = max_released.max(step3.compute_next(
                network,
                path.clone(),
                open_valves.clone(),
                level + 1,
            ));
            // self.next.push(Rc::new(RefCell::new(step3)));
        }

        max_released
        // let valve = self.action;
    }*/
}

pub fn parse_and_run(input: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:(?:\w+), )*(?:\w+))$"
        )
        .unwrap();
    }

    let mut network = Network { valves: vec![] };

    let mut distances: Vec<Distance> = vec![];

    input
        .lines()
        .map(|line| RE.captures(line))
        .for_each(|capture| {
            let capture = capture.unwrap();
            let valve = Valve {
                name: capture[1].to_string(),
                flow_rate: capture[2].parse().unwrap(),
                // tunnels: capture[3].split(", ").map(|s| s.to_string()).collect(),
                // ..Default::default()
            };
            // dbg!(&capture);
            network.valves.push(Rc::new(RefCell::new(valve)));
        });
    // dbg!(&network);

    input
        .lines()
        .map(|line| RE.captures(line))
        .enumerate()
        .for_each(|(i, capture)| {
            let capture = capture.unwrap();
            capture[3].split(", ").for_each(|value| {
                let from = network.find_by_name(capture[1].to_string());
                let to = network.find_by_name(value.to_string());

                distances.push(Distance {
                    from: from.clone(),
                    to: to.clone(),
                    distance: 1,
                });

                for distance in distances {
                    if (&*distance.to).borrow().name == (&*from).borrow().name {
                        distances.push(Distance {
                            from: from.clone(),
                            to: distance.to.clone(),
                            distance: distance.borrow().distance + 1,
                        })
                    }
                }
            })
        });

    /*let mut root = Step {
        action: Some(Action::Move(network.find_by_name("AA".to_string()))),
        ..Default::default()
    };*/

    // root.compute_next(&network, vec![], vec![], 0)

    // dbg!(&network.open());
    // let mut current_valve = network.find_by_name("AA".to_string());
    // loop {
    //     for actions
    // }

    // dbg!(&root);
    0
}

fn main() {
    let input = include_str!("example");
    dbg!(parse_and_run(input));
}

#[cfg(test)]
mod test {}
