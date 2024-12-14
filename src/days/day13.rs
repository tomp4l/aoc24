use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let mut claw_machines = input
            .split("\n\n")
            .map(|s| s.parse::<ClawMachine>())
            .collect::<Result<Vec<_>, _>>()?;

        let part1 = claw_machines
            .iter()
            .filter_map(|c| c.min_tokens())
            .sum::<usize>()
            .to_string();

        let part2 = claw_machines
            .iter_mut()
            .filter_map(|c| {
                c.recalibrate();
                c.min_tokens()
            })
            .sum::<usize>()
            .to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_button(s: &str) -> Result<(i64, i64), String> {
    let mut parts = s.split(", ");
    let x = parts
        .next()
        .ok_or("no x")?
        .split('+')
        .nth(1)
        .ok_or("no x")?
        .parse()
        .map_err(|e| format!("failed to parse x: {}", e))?;
    let y = parts
        .next()
        .ok_or("no y")?
        .split('+')
        .nth(1)
        .ok_or("no y")?
        .parse()
        .map_err(|e| format!("failed to parse y: {}", e))?;
    Ok((x, y))
}

fn parse_prize(s: &str) -> Result<(i64, i64), String> {
    let mut parts = s.split(", ");
    let x = parts
        .next()
        .ok_or("no x")?
        .split('=')
        .nth(1)
        .ok_or("no x")?
        .parse()
        .map_err(|e| format!("failed to parse x: {}", e))?;
    let y = parts
        .next()
        .ok_or("no y")?
        .split('=')
        .nth(1)
        .ok_or("no y")?
        .parse()
        .map_err(|e| format!("failed to parse y: {}", e))?;
    Ok((x, y))
}
impl ClawMachine {
    fn min_tokens(&self) -> Option<usize> {
        let (ax, ay) = self.button_a;
        let (bx, by) = self.button_b;
        let (px, py) = self.prize;

        let ax_py = ax * py;
        let ay_px = ay * px;
        let ax_by = ax * by;
        let ay_bx = ay * bx;

        let l = ax_py - ay_px;
        let r = ax_by - ay_bx;

        if l % r == 0 {
            let b = l / r;
            let rem = px - b * bx;
            if rem % ax == 0 {
                let a = rem / ax;
                if a >= 0 && b >= 0 {
                    return Some((a * 3 + b) as usize);
                }
            }
        }
        None
    }

    fn recalibrate(&mut self) {
        self.prize.0 += 10000000000000;
        self.prize.1 += 10000000000000;
    }
}

impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let button_a = parse_button(lines.next().ok_or("missing button_a")?)
            .map_err(|e| format!("failed to parse button_a: {}\ninput:\n{}", e, s))?;
        let button_b = parse_button(lines.next().ok_or("missing button_b")?)
            .map_err(|e| format!("failed to parse button_b: {}\ninput:\n{}", e, s))?;
        let prize = parse_prize(lines.next().ok_or("missing prize")?)
            .map_err(|e| format!("failed to parse prize: {}\ninput:\n{}", e, s))?;
        Ok(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "480".to_owned(),
                part2: Some("875318608908".to_owned())
            })
        );
    }
}
