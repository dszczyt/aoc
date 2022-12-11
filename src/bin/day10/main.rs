use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct CPU {
    pub cycle: usize,
    pub x: Register,
    pub current_instruction: Box<dyn Instruction>,
}

impl CPU {
    pub(crate) fn signal_strength(&self) -> i32 {
        self.x.value * self.cycle as i32
    }

    pub(crate) fn prepare_cycle(&mut self) {
        self.cycle += 1;
    }

    pub(crate) fn finalize_cycle(&mut self, program: &mut Program) -> Option<()> {
        let cycles_left_for_instruction = self.current_instruction.consume_cycle(&mut self.x);

        if cycles_left_for_instruction == 0 {
            if let Some(next_instruction) = program.next_instruction() {
                self.current_instruction = next_instruction.instruction();
            } else {
                return None;
            }
        }
        Some(())
    }

    pub(crate) fn next_cycle(&mut self, program: &mut Program) -> Option<()> {
        self.prepare_cycle();
        self.finalize_cycle(program)
    }
}

#[derive(Debug)]
pub(crate) struct Register {
    pub value: i32,
}

impl Default for Register {
    fn default() -> Self {
        Self { value: 1 }
    }
}

pub(crate) trait Instruction
where
    Self: Debug,
{
    fn consume_cycle(&mut self, register: &mut Register) -> usize;
    fn set_result(&self, _: &mut CPU) {}
}

#[derive(Debug)]
pub(crate) struct Noop {
    pub cycles_left: usize,
}

impl Default for Noop {
    fn default() -> Self {
        Self { cycles_left: 1 }
    }
}

impl Instruction for Noop {
    fn consume_cycle(&mut self, register: &mut Register) -> usize {
        self.cycles_left -= 1;
        self.cycles_left
    }
}

#[derive(Debug)]
pub(crate) struct AddX {
    pub value: i32,
    pub cycles_left: usize,
}

impl Default for AddX {
    fn default() -> Self {
        Self {
            value: 0,
            cycles_left: 2,
        }
    }
}

impl Instruction for AddX {
    fn set_result(&self, cpu: &mut CPU) {
        cpu.x.value += self.value;
    }

    fn consume_cycle(&mut self, register: &mut Register) -> usize {
        self.cycles_left -= 1;
        if self.cycles_left == 0 {
            register.value += self.value;
        }
        self.cycles_left
    }
}

pub(crate) struct ProgramLine {
    line: String,
}

impl From<&str> for ProgramLine {
    fn from(input: &str) -> Self {
        Self {
            line: input.to_string(),
        }
    }
}

impl ProgramLine {
    pub(crate) fn instruction(&self) -> Box<dyn Instruction> {
        let parts: Vec<&str> = self.line.splitn(2, " ").collect();
        match *parts.get(0).unwrap() {
            "noop" => Box::new(Noop::default()),
            "addx" => Box::new(AddX {
                value: i32::from_str_radix(parts.get(1).unwrap(), 10).unwrap(),
                ..Default::default()
            }),
            _ => unreachable!(),
        }
    }
}

pub(crate) struct Program {
    lines: Vec<ProgramLine>,
    instruction_pointer: usize,
}

impl From<&str> for Program {
    fn from(input: &str) -> Self {
        Self {
            lines: input.lines().map(Into::into).collect(),
            instruction_pointer: 0,
        }
    }
}

impl Program {
    pub(crate) fn next_instruction(&mut self) -> Option<&ProgramLine> {
        let instruction = self.lines.get(self.instruction_pointer);
        self.instruction_pointer += 1;
        instruction
    }
}

fn main() {
    let mut sum = 0;
    let input = include_str!("input");
    let mut program: Program = input.into();
    let mut cpu = CPU {
        cycle: 0,
        x: Register::default(),
        current_instruction: program
            .next_instruction()
            .map(|instruction| instruction.instruction())
            .unwrap(),
    };

    (0..19).for_each(|_| {
        cpu.next_cycle(&mut program);
    });
    cpu.prepare_cycle();
    dbg!(&cpu);
    sum += cpu.signal_strength();
    cpu.finalize_cycle(&mut program);

    (0..39).for_each(|_| {
        cpu.next_cycle(&mut program);
    });
    cpu.prepare_cycle();
    dbg!(&cpu);
    sum += cpu.signal_strength();
    cpu.finalize_cycle(&mut program);

    (0..39).for_each(|_| {
        cpu.next_cycle(&mut program);
    });
    cpu.prepare_cycle();
    dbg!(&cpu);
    sum += cpu.signal_strength();
    cpu.finalize_cycle(&mut program);

    (0..39).for_each(|_| {
        cpu.next_cycle(&mut program);
    });
    cpu.prepare_cycle();
    dbg!(&cpu);
    sum += cpu.signal_strength();
    cpu.finalize_cycle(&mut program);

    (0..39).for_each(|_| {
        cpu.next_cycle(&mut program);
    });
    cpu.prepare_cycle();
    dbg!(&cpu);
    sum += cpu.signal_strength();
    cpu.finalize_cycle(&mut program);

    (0..39).for_each(|_| {
        cpu.next_cycle(&mut program);
    });
    cpu.prepare_cycle();
    dbg!(&cpu);
    sum += cpu.signal_strength();

    dbg!(&sum);

    let input = include_str!("input");
    let mut program: Program = input.into();
    let mut cpu = CPU {
        cycle: 0,
        x: Register::default(),
        current_instruction: program
            .next_instruction()
            .map(|instruction| instruction.instruction())
            .unwrap(),
    };

    (0..(40 * 6)).for_each(|i| {
        if i > 0 && i % 40 == 0 {
            print!("\n");
        }
        // cpu.next_cycle(&mut program);
        cpu.prepare_cycle();
        if ((cpu.x.value - 1)..(cpu.x.value + 2)).contains(&(i % 40)) {
            print!("#");
        } else {
            print!(" ");
        }
        cpu.finalize_cycle(&mut program);
    });
}

#[cfg(test)]
mod test {
    use crate::{Program, Register, CPU};

    #[test]
    fn part1_1() {
        let input = include_str!("example1_1");
        let mut program: Program = input.into();
        let mut cpu = CPU {
            cycle: 0,
            x: Register::default(),
            current_instruction: program
                .next_instruction()
                .map(|instruction| instruction.instruction())
                .unwrap(),
        };

        cpu.next_cycle(&mut program);
        assert_eq!(cpu.x.value, 1);
        assert_eq!(cpu.cycle, 1);

        cpu.next_cycle(&mut program);
        assert_eq!(cpu.x.value, 1);
        assert_eq!(cpu.cycle, 2);

        cpu.next_cycle(&mut program);
        assert_eq!(cpu.x.value, 4);
        assert_eq!(cpu.cycle, 3);

        cpu.next_cycle(&mut program);
        assert_eq!(cpu.x.value, 4);
        assert_eq!(cpu.cycle, 4);

        cpu.next_cycle(&mut program);
        assert_eq!(cpu.x.value, -1);
        assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn part1_2() {
        let mut sum = 0;
        let input = include_str!("example1_2");
        let mut program: Program = input.into();
        let mut cpu = CPU {
            cycle: 0,
            x: Register::default(),
            current_instruction: program
                .next_instruction()
                .map(|instruction| instruction.instruction())
                .unwrap(),
        };

        (0..19).for_each(|_| {
            cpu.next_cycle(&mut program);
        });
        cpu.prepare_cycle();
        dbg!(&cpu);
        assert_eq!(cpu.signal_strength(), 420);
        sum += cpu.signal_strength();
        cpu.finalize_cycle(&mut program);

        (0..39).for_each(|_| {
            cpu.next_cycle(&mut program);
        });
        cpu.prepare_cycle();
        dbg!(&cpu);
        assert_eq!(cpu.signal_strength(), 1140);
        sum += cpu.signal_strength();
        cpu.finalize_cycle(&mut program);

        (0..39).for_each(|_| {
            cpu.next_cycle(&mut program);
        });
        cpu.prepare_cycle();
        dbg!(&cpu);
        assert_eq!(cpu.signal_strength(), 1800);
        sum += cpu.signal_strength();
        cpu.finalize_cycle(&mut program);

        (0..39).for_each(|_| {
            cpu.next_cycle(&mut program);
        });
        cpu.prepare_cycle();
        dbg!(&cpu);
        assert_eq!(cpu.signal_strength(), 2940);
        sum += cpu.signal_strength();
        cpu.finalize_cycle(&mut program);

        (0..39).for_each(|_| {
            cpu.next_cycle(&mut program);
        });
        cpu.prepare_cycle();
        dbg!(&cpu);
        assert_eq!(cpu.signal_strength(), 2880);
        sum += cpu.signal_strength();
        cpu.finalize_cycle(&mut program);

        (0..39).for_each(|_| {
            cpu.next_cycle(&mut program);
        });
        cpu.prepare_cycle();
        dbg!(&cpu);
        assert_eq!(cpu.signal_strength(), 3960);
        sum += cpu.signal_strength();

        assert_eq!(sum, 13140);
    }

    #[test]
    fn part2() {
        let mut display = "".to_owned();

        let input = include_str!("example1_2");
        let mut program: Program = input.into();
        let mut cpu = CPU {
            cycle: 0,
            x: Register::default(),
            current_instruction: program
                .next_instruction()
                .map(|instruction| instruction.instruction())
                .unwrap(),
        };

        (0..(40 * 6)).for_each(|i| {
            if i > 0 && i % 40 == 0 {
                display.push('\n');
            }
            // cpu.next_cycle(&mut program);
            cpu.prepare_cycle();

            let mut crt_row = "".to_owned();
            (0..(cpu.x.value - 1)).for_each(|_| {
                crt_row.push('.');
            });
            ((cpu.x.value - 1)..(cpu.x.value + 2)).for_each(|_| {
                crt_row.push('#');
            });
            dbg!(&i, &crt_row);

            if ((cpu.x.value - 1)..(cpu.x.value + 2)).contains(&(i % 40)) {
                display.push('#');
            } else {
                display.push('.');
            }
            cpu.finalize_cycle(&mut program);
        });

        assert_eq!(display, include_str!("expected_part2").to_owned());
    }
}
