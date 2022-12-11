use std::str::FromStr;

use color_eyre::{eyre::eyre, Report, Result};

fn main() -> Result<()> {
    let input = libaoc::init()?;

    let instructions = parse_input(&input).unwrap();
    let mut computer = Computer::new();
    let result: i32 = computer
        .execute(instructions.into_iter())
        .skip(19)
        .step_by(40)
        .map(|s| s.clock as i32 * s.during)
        .sum();

    println!("The sum of signal strengths is {result}");
    computer.display();

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(i32),
    Nop,
}

impl Instruction {
    fn cycle_time(&self) -> usize {
        match self {
            Instruction::Add(_) => 2,
            Instruction::Nop => 1,
        }
    }

    fn execute(&self, acc: &mut i32) {
        match self {
            Instruction::Add(op) => *acc += op,
            Instruction::Nop => (),
        }
    }
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s.trim().split_once(' ') {
            Some(("addx", a)) => Ok(Self::Add(a.parse()?)),
            None if s == "noop" => Ok(Self::Nop),
            _ => Err(eyre!("Invalid instruction '{s}'")),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    input.trim().lines().map(|l| l.parse()).collect()
}

struct Computer {
    clock: usize,
    acc: i32,
    current: Option<(usize, Instruction)>,
    display_buffer: [bool; 240],
}

impl Computer {
    fn new() -> Self {
        Self {
            clock: 0,
            acc: 1,
            current: None,
            display_buffer: [false; 240],
        }
    }

    fn execute<I>(&mut self, instructions: I) -> Program<I>
    where
        I: Iterator<Item = Instruction>,
    {
        Program {
            computer: self,
            instructions,
        }
    }

    fn display(&self) {
        for r in 0..6 {
            for p in 0..=39 {
                if self.display_buffer[r * 40 + p] {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

struct Program<'a, I: Iterator<Item = Instruction>> {
    computer: &'a mut Computer,
    instructions: I,
}

#[derive(Debug, PartialEq)]
struct State {
    clock: usize,
    during: i32,
    after: i32,
}

impl State {
    fn new(clock: usize, during: i32, after: i32) -> Self {
        Self {
            clock,
            during,
            after,
        }
    }
}

impl<'a, I: Iterator<Item = Instruction>> Iterator for Program<'a, I> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let computer = &mut self.computer;
        let (loaded_at, ins) = computer
            .current
            .take()
            .or_else(|| Some((computer.clock, self.instructions.next()?)))?;
        computer.display_buffer[computer.clock] =
            (computer.clock as i32 % 40 - computer.acc).abs() <= 1;
        computer.clock += 1;
        let during = computer.acc;
        if computer.clock - loaded_at == ins.cycle_time() {
            ins.execute(&mut computer.acc);
        } else {
            computer.current = Some((loaded_at, ins));
        }

        Some(State::new(computer.clock, during, computer.acc))
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use pretty_assertions::assert_eq;
    use rstest::*;

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        indoc! {"
            noop
            addx 3
            addx -5
        "}
    }

    #[fixture]
    fn instructions() -> Vec<Instruction> {
        vec![Instruction::Nop, Instruction::Add(3), Instruction::Add(-5)]
    }

    #[rstest]
    fn test_parsing(input: &str, instructions: Vec<Instruction>) {
        assert_eq!(parse_input(input).unwrap(), instructions);
    }

    #[rstest]
    fn test_execution(instructions: Vec<Instruction>) {
        let mut computer = Computer::new();
        let mut program = computer.execute(instructions.into_iter());

        assert_eq!(program.next().unwrap(), State::new(1, 1, 1));
        assert_eq!(program.next().unwrap(), State::new(2, 1, 1));
        assert_eq!(program.next().unwrap(), State::new(3, 1, 4));
        assert_eq!(program.next().unwrap(), State::new(4, 4, 4));
        assert_eq!(program.next().unwrap(), State::new(5, 4, -1));
    }

    #[rstest]
    fn test_signal_strength() {
        let input = indoc! {"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        "};
        let instructions = parse_input(input).unwrap();

        let mut computer = Computer::new();
        let result = computer
            .execute(instructions.into_iter())
            .skip(19)
            .step_by(40)
            .map(|s| (s.clock as i32 * s.during))
            .collect::<Vec<_>>();
        assert_eq!(result, [420, 1140, 1800, 2940, 2880, 3960]);
    }
}
