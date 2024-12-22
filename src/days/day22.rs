use std::collections::HashMap;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let numbers = parse_input(input.as_str())?;
        let part1 = part1(numbers.as_slice()).to_string();
        let part2 = Some(part2(&numbers).to_string());
        Ok(DayResult { part1, part2 })
    }
}

fn parse_input(input: &str) -> Result<Vec<usize>, String> {
    input
        .lines()
        .map(|line| line.parse::<usize>().map_err(|e| e.to_string()))
        .collect()
}

fn part1(numbers: &[usize]) -> usize {
    numbers.iter().map(|n| secret_2000(*n)).sum()
}

fn next_number(secret: usize) -> usize {
    let secret = mix(secret, secret * 64);
    let secret = prune(secret);
    let secret = mix(secret, secret / 32);
    let secret = prune(secret);
    let secret = mix(secret, secret * 2048);
    let secret = prune(secret);

    secret
}

fn mix(secret: usize, number: usize) -> usize {
    secret ^ number
}

fn prune(secret: usize) -> usize {
    secret % 16_777_216
}

fn secret_2000(initial: usize) -> usize {
    let mut secret = initial;
    for _ in 0..2000 {
        secret = next_number(secret);
    }
    secret
}

fn part2(numbers: &[usize]) -> usize {
    let mut runs = HashMap::new();

    for number in numbers {
        let runs_for_number = four_runs_from_initial(*number);
        for (run, price) in runs_for_number {
            runs.entry(run)
                .and_modify(|p| *p += price as usize)
                .or_insert(price as usize);
        }
    }

    *runs.values().max().unwrap()
}

fn prices(number: usize) -> Vec<i8> {
    let mut prices = Vec::new();
    let mut secret = number;

    for _ in 0..=2000 {
        prices.push((secret % 10) as i8);
        secret = next_number(secret);
    }

    prices
}

fn add_price_deltas(prices: &[i8]) -> Vec<(i8, i8)> {
    prices
        .iter()
        .zip(prices.iter().skip(1))
        .map(|(a, b)| (*b, b - a))
        .collect()
}

fn four_runs(prices_and_deltas: &[(i8, i8)]) -> HashMap<(i8, i8, i8, i8), i8> {
    let mut runs = HashMap::new();

    for i in 0..prices_and_deltas.len() - 3 {
        let run = (
            prices_and_deltas[i].1,
            prices_and_deltas[i + 1].1,
            prices_and_deltas[i + 2].1,
            prices_and_deltas[i + 3].1,
        );

        let price = prices_and_deltas[i + 3].0;

        if !runs.contains_key(&run) {
            runs.insert(run, price);
        }
    }

    runs
}

fn four_runs_from_initial(number: usize) -> HashMap<(i8, i8, i8, i8), i8> {
    let prices = prices(number);
    let deltas = add_price_deltas(&prices);
    four_runs(&deltas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "1
10
100
2024"
            .to_owned();
        let day = Instance;
        let result = day.run(input);
        assert_eq!(
            result,
            Ok(DayResult {
                part1: "37327623".to_owned(),
                part2: Some("24".to_owned())
            })
        );
    }

    #[test]
    fn first_ten() {
        let mut secret = 123;

        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for e in expected {
            secret = next_number(secret);
            assert_eq!(secret, e);
        }
    }

    #[test]
    fn test_secret_2000() {
        assert_eq!(secret_2000(1), 8685429);
    }

    #[test]
    fn test_deltas() {
        let prices = prices(123);

        let deltas = add_price_deltas(&prices);

        let first_9: Vec<_> = deltas.iter().take(9).cloned().collect();

        assert_eq!(
            first_9,
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2),
            ]
        );
    }

    #[test]
    fn four_runs() {
        let a = four_runs_from_initial(1);
        let b = four_runs_from_initial(2);
        let c = four_runs_from_initial(3);
        let d = four_runs_from_initial(2024);

        assert_eq!(a.get(&(-2, 1, -1, 3)), Some(&7));
        assert_eq!(b.get(&(-2, 1, -1, 3)), Some(&7));
        assert_eq!(c.get(&(-2, 1, -1, 3)), None);
        assert_eq!(d.get(&(-2, 1, -1, 3)), Some(&9));
    }
}
