use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let mut computer: Computer = input.parse()?;
        let output = computer.run();

        let part1 = output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let part2 = Some(find_output(computer.program).to_string());
        Ok(DayResult { part1, part2 })
    }
}

const SEARCHES: usize = 16;

fn find_output(program: Vec<usize>) -> usize {
    let mut computer = Computer {
        registers: [0, 0, 0],
        program,
    };

    let mut last = 0;
    let mut next = 1;
    let mut search_start = 0;
    let mut target = computer.program.len();
    loop {
        let output = run_computer(&mut computer, next);
        if output.len() == target {
            if last == next - 1 {
                if search_start == 0 {
                    search_start = next;
                    target = target + 1;
                } else {
                    break;
                }
            }
            let test = (last + next) / 2;
            let output = run_computer(&mut computer, test);
            if output.len() == target {
                next = test;
            } else {
                last = test;
            }
        } else {
            last = next;
            next = next << 1;
        }
    }

    let search_end = last;
    let mut next_ranges = vec![(search_start, search_end)];

    for index in (0..computer.program.len()).rev() {
        let mut new_next_ranges = Vec::new();
        for next_range in next_ranges {
            let (search_start, search_end) = next_range;
            let diff = search_end - search_start;
            let searches = SEARCHES.min(diff);
            let inc = diff / searches;

            for i in 0..searches {
                let start = search_start + i * inc;
                let output = run_computer(&mut computer, start);
                let target = computer.program[index];
                if output[index] == target {
                    if index == 0 {
                        return start;
                    }
                    new_next_ranges.push(find_range(
                        &mut computer,
                        index,
                        target,
                        start,
                        start - inc,
                        start + inc,
                    ));
                }
            }
        }

        let mut deduped: Vec<(usize, usize)> = Vec::new();

        for range in new_next_ranges {
            if let Some(last) = deduped.last_mut() {
                if range.0 >= last.0 && range.0 <= last.1 {
                    last.1 = range.1.max(last.1);
                } else {
                    deduped.push(range);
                }
            } else {
                deduped.push(range);
            }
        }

        next_ranges = deduped;
    }
    panic!("no solution found");
}

fn find_range(
    computer: &mut Computer,
    target_index: usize,
    target_value: usize,
    start: usize,
    min: usize,
    max: usize,
) -> (usize, usize) {
    if min == max {
        return (min, max);
    }
    let mut low = min;
    let mut high = start;
    loop {
        if low == high - 1 {
            break;
        }
        let test = (low + high) / 2;
        let output = run_computer(computer, test);
        if output.len() <= target_index || output[target_index] == target_value {
            high = test;
        } else {
            low = test;
        }
    }
    let range_start = high;

    let mut low = start;
    let mut high = max;
    loop {
        if low == high - 1 {
            break;
        }
        let test = (low + high) / 2;
        let output = run_computer(computer, test);
        if output[target_index] == target_value {
            low = test;
        } else {
            high = test;
        }
    }
    let range_end: usize = low;

    (range_start, range_end)
}

fn run_computer(computer: &mut Computer, i: usize) -> Vec<usize> {
    computer.registers = [i, 0, 0];
    computer.run()
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
enum Combo {
    Literal(usize),
    Register(usize),
}

impl Combo {
    fn from_usize(v: usize) -> Self {
        if v >= 7 {
            panic!("invalid combo");
        }
        if v < 4 {
            Self::Literal(v)
        } else {
            Self::Register(v - 4)
        }
    }
}

#[derive(Debug)]
struct Computer {
    registers: [usize; 3],
    program: Vec<usize>,
}

impl Instruction {
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("invalid instruction"),
        }
    }
}

impl FromStr for Computer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut registers = [0; 3];

        for i in 0..3 {
            let line = lines.next().ok_or("missing register")?;
            let parts = line.split_whitespace().collect::<Vec<_>>();
            if parts.len() != 3 {
                return Err(format!("invalid register line '{}'", line));
            }

            registers[i] = parts[2]
                .parse()
                .map_err(|e| format!("failed to parse register '{}': {}", parts[1], e))?;
        }

        lines.next().ok_or("missing program")?;

        let program = lines
            .next()
            .ok_or("missing program")?
            .strip_prefix("Program: ")
            .ok_or("missing program prefix")?
            .split(",")
            .map(|i| {
                i.parse::<usize>()
                    .map_err(|e| format!("failed to parse code '{}': {}", i, e))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { registers, program })
    }
}

impl Computer {
    fn run(&mut self) -> Vec<usize> {
        let mut output = Vec::new();
        let mut pc = 0;
        while pc < self.program.len() - 1 {
            let instruction = Instruction::from_u8(self.program[pc] as u8);
            let operand = self.program[pc + 1];
            match instruction {
                Instruction::Adv => {
                    let a = self.registers[0];
                    self.registers[0] = a >> self.get_combo(operand);
                }
                Instruction::Bxl => {
                    let b = self.registers[1];
                    self.registers[1] = b ^ operand;
                }
                Instruction::Bst => {
                    self.registers[1] = self.get_combo(operand) % 8;
                }
                Instruction::Jnz => {
                    if self.registers[0] != 0 {
                        pc = operand;
                        continue;
                    }
                }
                Instruction::Bxc => {
                    let b = self.registers[1];
                    let c = self.registers[2];
                    self.registers[1] = b ^ c;
                }
                Instruction::Out => {
                    let v = self.get_combo(operand);
                    output.push(v % 8);
                }
                Instruction::Bdv => {
                    let a = self.registers[0];
                    self.registers[1] = a >> self.get_combo(operand);
                }
                Instruction::Cdv => {
                    let a = self.registers[0];
                    self.registers[2] = a >> self.get_combo(operand);
                }
            }
            pc += 2;
        }
        output
    }

    fn get_combo(&self, operand: usize) -> usize {
        match Combo::from_usize(operand) {
            Combo::Literal(v) => v,
            Combo::Register(v) => self.registers[v],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
"
        .to_owned();
        let day = Instance;
        let result = day.run(input).unwrap();
        assert_eq!(result.part1, "5,7,3,0");
        assert_eq!(result.part2, Some("117440".to_owned()));
    }

    #[test]
    fn small_examples() {
        let mut computer = Computer {
            registers: [0, 0, 9],
            program: vec![2, 6],
        };
        assert!(computer.run().is_empty());
        assert!(computer.registers[1] == 1);

        let mut computer = Computer {
            registers: [10, 0, 0],
            program: vec![5, 0, 5, 1, 5, 4],
        };
        assert_eq!(computer.run(), vec![0, 1, 2]);

        let mut computer = Computer {
            registers: [2024, 0, 0],
            program: vec![0, 1, 5, 4, 3, 0],
        };
        assert_eq!(computer.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert!(computer.registers[0] == 0);

        let mut computer = Computer {
            registers: [0, 29, 0],
            program: vec![1, 7],
        };

        assert!(computer.run().is_empty());
        assert!(computer.registers[1] == 26);

        let mut computer = Computer {
            registers: [0, 2024, 43690],
            program: vec![4, 0],
        };

        assert!(computer.run().is_empty());
        assert!(computer.registers[1] == 44354);
    }
}
