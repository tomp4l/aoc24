use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let code = lines.join("\n").parse::<CorruptedCode>()?;

        let part1 = code.mul_always().to_string();
        let part2 = Some(code.mul_if_enabled().to_string());
        Ok(DayResult { part1, part2 })
    }
}

struct CorruptedCode {
    line: String,
}

impl FromStr for CorruptedCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CorruptedCode { line: s.to_owned() })
    }
}

struct Line<'a> {
    remaining: &'a str,
}

impl<'a> Line<'a> {
    fn new(s: &'a str) -> Self {
        Line { remaining: s }
    }

    fn match_str(&mut self, s: &str) -> bool {
        if self.remaining.starts_with(s) {
            self.remaining = &self.remaining[s.len()..];
            true
        } else {
            false
        }
    }

    fn match_num(&mut self) -> usize {
        let digits = self
            .remaining
            .chars()
            .take_while(|c| c.is_digit(10))
            .count();
        let num = self.remaining[..digits]
            .parse::<usize>()
            .unwrap_or_default();
        self.remaining = &self.remaining[digits..];
        num
    }

    fn next_char(&mut self) {
        self.remaining = &self.remaining[1..];
    }

    fn has_remaining(&self) -> bool {
        self.remaining.len() > 0
    }
}

impl CorruptedCode {
    fn mul_always(&self) -> usize {
        self.mul(false)
    }

    fn mul_if_enabled(&self) -> usize {
        self.mul(true)
    }

    fn mul(&self, respect_enabled: bool) -> usize {
        let mut result = 0;
        let mut line = Line::new(&self.line);
        let mut enabled = true;

        while line.has_remaining() {
            if respect_enabled && enabled && line.match_str("don't()") {
                enabled = false;
            } else if line.match_str("do()") {
                enabled = true;
            } else if enabled && line.match_str("mul(") {
                let num1 = line.match_num();
                if !line.match_str(",") {
                    continue;
                }

                let num2 = line.match_num();
                if !line.match_str(")") {
                    continue;
                }

                result += num1 * num2;
            } else {
                line.next_char();
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corrupted_code() {
        let code = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            .parse::<CorruptedCode>()
            .unwrap();
        assert_eq!(code.mul_always(), 161);
        assert_eq!(code.mul_if_enabled(), 48);
    }

    #[test]
    fn test_double_mul() {
        let code = "mul(mul(3,7)mul(4,mul(1,2)"
            .parse::<CorruptedCode>()
            .unwrap();
        assert_eq!(code.mul_always(), 23);
    }

    #[test]
    fn test_empty_mul() {
        let code = "mul(,),mul(1,)mul(,1)mul(1,1)"
            .parse::<CorruptedCode>()
            .unwrap();
        assert_eq!(code.mul_always(), 1);
    }
}
