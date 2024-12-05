use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let manual: Manual = input.parse()?;

        let part1 = manual.ordeded().to_string();
        let part2 = Some(manual.fix_unordered().to_string());
        Ok(DayResult { part1, part2 })
    }
}

#[derive(Debug)]
struct Manual {
    rules: Vec<Rule>,
    pages: Vec<Pages>,
}

impl FromStr for Manual {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = Vec::new();
        let mut pages = Vec::new();

        let mut lines = s.lines();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            rules.push(line.parse()?);
        }

        for line in lines {
            pages.push(line.parse()?);
        }

        Ok(Manual { rules, pages })
    }
}

#[derive(Debug)]
struct Rule {
    before: u8,
    after: u8,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("|");
        let before = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|e| format!("failed to parse before in '{}': {}", s, e))?;
        let after = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|e| format!("failed to parse after in '{}': {}", s, e))?;
        Ok(Rule { before, after })
    }
}

#[derive(Debug, Clone)]
struct Pages {
    order: Vec<u8>,
}

impl FromStr for Pages {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let order = s
            .split(",")
            .map(|n| {
                n.parse::<u8>()
                    .map_err(|e| format!("failed to parse page in '{}': {}", s, e))
            })
            .collect::<Result<_, _>>()?;
        Ok(Pages { order })
    }
}

impl Manual {
    fn ordeded(&self) -> usize {
        let mut total = 0;

        for pages in &self.pages {
            if pages.is_ordered(&self.rules) {
                total += pages.middle();
            }
        }

        total
    }

    fn fix_unordered(&self) -> usize {
        let mut total = 0;

        for pages in &self.pages {
            if !pages.is_ordered(&self.rules) {
                let mut to_fix = pages.clone();
                to_fix.fix(&self.rules);
                total += to_fix.middle();
            }
        }

        total
    }
}

impl Pages {
    fn is_ordered(&self, rules: &[Rule]) -> bool {
        for rule in rules {
            let before = self.order.iter().position(|&x| x == rule.before);
            let after = self.order.iter().position(|&x| x == rule.after);

            if before.is_some() && after.is_some() && before > after {
                return false;
            }
        }
        true
    }

    fn fix(&mut self, rules: &[Rule]) {
        self.order.sort_by(|a, b| {
            if rules
                .iter()
                .find(|r| r.before == *a && r.after == *b)
                .is_some()
            {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
    }

    fn middle(&self) -> usize {
        self.order[self.order.len() / 2] as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_owned();
        let result = Instance.run(input).unwrap();
        assert_eq!(result.part1, "143");
        assert_eq!(result.part2, Some("123".to_owned()));
    }
}
