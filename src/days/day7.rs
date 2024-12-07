use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let parsed: Vec<Equation> = input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()?;

        let part1 = parsed
            .iter()
            .filter(|eq| eq.solves(false))
            .map(|eq| eq.answer)
            .sum::<usize>()
            .to_string();
        let part2 = parsed
            .iter()
            .filter(|eq| eq.solves(true))
            .map(|eq| eq.answer)
            .sum::<usize>()
            .to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

struct Equation {
    answer: usize,
    numbers: Vec<usize>,
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (answer_str, numbers_str) = s.split_once(": ").ok_or("missing ': '")?;

        let mut numbers = Vec::new();
        for part in numbers_str.split_whitespace() {
            numbers.push(
                part.parse()
                    .map_err(|e| format!("bad number format for {}: {}", part, e))?,
            );
        }

        let answer = answer_str
            .parse()
            .map_err(|e| format!("bad answer format for {}: {}", answer_str, e))?;

        Ok(Equation { answer, numbers })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Equation {
    fn solves(&self, with_concat: bool) -> bool {
        let mut operators = vec![Operator::Add, Operator::Multiply];

        if with_concat {
            if self.solves(false) {
                return true;
            }
            operators.push(Operator::Concat);
        }

        let mut operator_chain: Vec<_> = operators.iter().map(|op| vec![*op]).collect();

        while operator_chain[0].len() < self.numbers.len() - 1 {
            operator_chain = operator_chain
                .iter()
                .flat_map(|chain| {
                    operators.iter().map(|&op| {
                        let mut new_chain = chain.clone();
                        new_chain.push(op);
                        new_chain
                    })
                })
                .collect();
        }

        for chain in operator_chain {
            let mut result = self.numbers[0];
            for (i, &num) in self.numbers.iter().enumerate().skip(1) {
                match chain[i - 1] {
                    Operator::Add => {
                        result += num;
                    }
                    Operator::Multiply => {
                        result *= num;
                    }
                    Operator::Concat => {
                        result = format!("{}{}", result, num).parse().unwrap();
                    }
                }
            }

            if result == self.answer {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation() {
        let equation: Equation = "5: 1 2 3".parse().unwrap();
        assert_eq!(equation.answer, 5);
        assert_eq!(equation.numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_solve() {
        let equation: Equation = "190: 10 19".parse().unwrap();
        assert!(equation.solves(false));
    }

    #[test]
    fn test_example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let day = Instance;
        let result = day.run(input.to_owned()).unwrap();
        assert_eq!(result.part1, "3749");
        assert_eq!(result.part2, Some("11387".to_owned()));
    }
}
