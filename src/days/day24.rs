use std::{
    collections::{HashMap, HashSet},
    mem::{swap, take},
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let mut wires: Wires = input.parse()?;

        let part1 = wires.z_value().to_string();
        let part2 = Some(part2(wires));

        Ok(DayResult { part1, part2 })
    }
}

fn part2(mut wires: Wires) -> String {
    let mut swaps: Vec<(usize, usize)> = Vec::new();
    let mut correct: HashSet<usize> = HashSet::new();
    let mut inputs = HashSet::new();

    fn has_error(wires: &mut Wires, i: usize) -> bool {
        let mut has_error = false;
        for j in 0..=i {
            let a = 1 << i;
            let b = 1 << j;
            let c = a - b;

            if b + c != wires.add_x_y(b, c) {
                has_error = true;
            }
        }
        has_error
    }

    let mut i = 0;
    let mut last_attempt = 0;
    let mut last_skip = 0;
    let mut last_correct = HashSet::new();
    while i < MAX_BIT_IN {
        inputs.insert(format!("x{:0>2}", i));
        inputs.insert(format!("y{:0>2}", i));

        let out = format!("z{:0>2}", i);

        if has_error(&mut wires, i) {
            let mut to_swap = Vec::new();
            for i in 0..wires.gates.len() {
                if correct.contains(&i) {
                    continue;
                }
                let g = &wires.gates[i];
                let ins = wires.linked_inputs(&g.out);
                if inputs.is_superset(&ins) {
                    to_swap.push(i);
                }
            }

            let mut new_swaps = vec![];

            for x in 0..to_swap.len() - 1 {
                for y in x + 1..to_swap.len() {
                    let x = to_swap[x];
                    let y = to_swap[y];

                    wires.swap(x, y);
                    if !has_error(&mut wires, i) {
                        new_swaps.push((x, y));
                    }
                    wires.swap(x, y);
                }
            }

            let swap_count = new_swaps.len();

            if swap_count >= 1 {
                if i != last_attempt {
                    last_skip = 0;
                    last_correct = correct.clone();
                }
                last_attempt = i;
                let (x, y) = new_swaps[last_skip];
                swaps.push((x, y));
                wires.swap(x, y);
            } else {
                i = last_attempt;
                last_skip += 1;
                let (x, y) = swaps.pop().unwrap();
                wires.swap(x, y);
                correct = last_correct.clone();
                continue;
            }
        }

        correct.extend(wires.linked_gates(&out));
        i += 1;
    }

    let mut swapped = vec![];
    for (x, y) in swaps {
        swapped.push(wires.gates[x].out.to_owned());
        swapped.push(wires.gates[y].out.to_owned());
    }
    swapped.sort();
    swapped.join(",")
}

struct Wires {
    state: HashMap<String, bool>,
    gates: Vec<LogicGate>,
}

const MAX_BIT_IN: usize = 45;
const MAX_XY: usize = 1 << (MAX_BIT_IN + 1);

impl Wires {
    fn z_value(&mut self) -> usize {
        let mut remaining: Vec<_> = self.gates.iter().collect();
        let mut remaining_len = remaining.len();

        while !remaining.is_empty() {
            let attempts = take(&mut remaining);
            for attempt in attempts {
                if let (Some(l), Some(r)) = (
                    self.state.get(&attempt.left),
                    self.state.get(&attempt.right),
                ) {
                    let out = attempt.operator.evaluate(*l, *r);

                    self.state.insert(attempt.out.to_owned(), out);
                } else {
                    remaining.push(attempt);
                }
            }
            if remaining.len() == remaining_len {
                return usize::MAX;
            }
            remaining_len = remaining.len();
        }

        let mut z_keys: Vec<_> = self.state.keys().filter(|k| k.starts_with("z")).collect();
        z_keys.sort();
        let mut i: usize = 1;
        let mut t = 0;
        for k in z_keys {
            if self.state[k] {
                t |= i;
            }
            i <<= 1;
        }
        t
    }

    fn add_x_y(&mut self, x: usize, y: usize) -> usize {
        assert!(x < MAX_XY);
        assert!(y < MAX_XY);
        self.state.clear();
        let mut mask: usize = 1;
        for i in 0..MAX_BIT_IN {
            let xid = format!("x{:0>2}", i);
            let yid = format!("y{:0>2}", i);
            let xbit = x & mask != 0;
            let ybit = y & mask != 0;
            self.state.insert(xid, xbit);
            self.state.insert(yid, ybit);
            mask <<= 1;
        }

        self.z_value()
    }

    fn swap(&mut self, x: usize, y: usize) {
        let (xs, ys) = self.gates.split_at_mut(x + 1);
        swap(&mut xs[x].out, &mut ys[y - x - 1].out);
    }

    fn linked_gates(&self, out: &str) -> Vec<usize> {
        if let Some((i, g)) = self.gates.iter().enumerate().find(|(_, l)| l.out == out) {
            let mut out = vec![i];
            out.extend(self.linked_gates(&g.left));
            out.extend(self.linked_gates(&g.right));
            out
        } else {
            vec![]
        }
    }

    fn linked_inputs(&self, out: &str) -> HashSet<String> {
        if let Some(g) = self.gates.iter().find(|l| l.out == out) {
            let mut out = HashSet::new();
            out.extend(self.linked_inputs(&g.left));
            out.extend(self.linked_inputs(&g.right));
            out
        } else {
            HashSet::from_iter([out.to_owned()])
        }
    }
}

impl FromStr for Wires {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let initial_state = parts.next().ok_or("missing initial state")?;
        let gates = parts.next().ok_or("missing gates")?;

        let mut state = HashMap::new();
        for s in initial_state.lines() {
            let mut splits = s.split(": ");
            let wire = splits.next().ok_or("missing wire")?;
            let on_off = splits.next().ok_or("missing on off")?;

            state.insert(wire.to_owned(), on_off == "1");
        }
        let gates = gates
            .lines()
            .map(|l| l.parse::<LogicGate>())
            .collect::<Result<_, _>>()?;

        Ok(Self { state, gates })
    }
}

#[derive(Debug)]
struct LogicGate {
    left: String,
    right: String,
    out: String,
    operator: Operator,
}

#[derive(Debug)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn evaluate(&self, l: bool, r: bool) -> bool {
        match self {
            Operator::And => l && r,
            Operator::Or => l || r,
            Operator::Xor => l != r,
        }
    }
}

impl FromStr for LogicGate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let left = parts.next().ok_or("missing left")?.to_owned();
        let operator = parts.next().ok_or("missing left")?.parse()?;
        let right = parts.next().ok_or("missing left")?.to_owned();
        parts.next();
        let out = parts.next().ok_or("missing left")?.to_owned();

        Ok(Self {
            left,
            right,
            out,
            operator,
        })
    }
}

impl FromStr for Operator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OR" => Ok(Operator::Or),
            "XOR" => Ok(Operator::Xor),
            "AND" => Ok(Operator::And),
            unknown => Err(format!("Unknown operator: {}", unknown)),
        }
    }
}
