use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: String) -> Result<DayResult, String> {
        let connections: Vec<_> = input
            .lines()
            .map(|l| l.parse::<Connection>())
            .collect::<Result<_, _>>()?;
        let connections = Connections::from_slice(&connections);

        let part1 = connections.count_lans_start_t().to_string();
        let part2 = connections.biggest_lan_party();
        let part2 = Some(part2.join(","));
        Ok(DayResult { part1, part2 })
    }
}

struct Connection {
    from: String,
    to: String,
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");

        let from = parts.next().ok_or("missing from")?.to_owned();
        let to = parts.next().ok_or("missing to")?.to_owned();

        Ok(Self { from, to })
    }
}

struct Connections {
    map: HashMap<String, HashSet<String>>,
}

impl Connections {
    fn from_slice(conns: &[Connection]) -> Self {
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();

        for conn in conns {
            let from = map.entry(conn.from.to_string()).or_default();
            from.insert(conn.to.to_owned());
            let to = map.entry(conn.to.to_string()).or_default();
            to.insert(conn.from.to_owned());
        }

        Self { map }
    }

    fn count_lans_start_t(&self) -> usize {
        let mut lans = HashSet::new();
        for (a, bs) in &self.map {
            for b in bs {
                let cs = &self.map[b];
                for c in cs {
                    let has_a = &self.map[c];
                    if !has_a.contains(a) {
                        continue;
                    }
                    if a.starts_with("t") || b.starts_with("t") || c.starts_with("t") {
                        let mut party = vec![a, b, c];
                        party.sort();
                        party.dedup();
                        if party.len() == 3 {
                            lans.insert(party);
                        }
                    }
                }
            }
        }

        lans.len()
    }

    fn biggest_lan_party(&self) -> Vec<String> {
        let nodes: HashSet<&str> = self.map.keys().map(|s| s.as_str()).collect();

        let mut cs = HashMap::new();

        for (n, vs) in &self.map {
            let mut set = HashSet::new();
            for v in vs {
                let b = nodes.iter().find(|s| **s == v).unwrap();
                set.insert(*b);
            }

            let a = nodes.iter().find(|s| **s == n).unwrap();
            cs.insert(*a, set);
        }

        let empty = HashSet::new();
        let mut res = Vec::new();
        bron_kerbosch(&empty, &nodes, &empty, &cs, &mut res);

        let biggest = res.into_iter().max_by_key(|c| c.len()).unwrap();

        let mut c: Vec<String> = biggest.into_iter().map(|s| s.to_owned()).collect();
        c.sort();
        c
    }
}

fn bron_kerbosch<'a>(
    r: &HashSet<&'a str>,
    p: &HashSet<&'a str>,
    x: &HashSet<&'a str>,
    connections: &HashMap<&str, HashSet<&'a str>>,
    res: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        res.push(r.clone());
    } else {
        let mut p_rem = p.clone();
        let mut x_rem = x.clone();
        for v in p {
            let mut r_next = r.clone();
            r_next.insert(*v);
            let p_next = p_rem.intersection(&connections[v]).copied().collect();
            let x_next = x_rem.intersection(&connections[v]).copied().collect();
            bron_kerbosch(&r_next, &p_next, &x_next, connections, res);
            p_rem.remove(*v);
            x_rem.insert(*v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"
        .to_owned();
        assert_eq!(
            Instance.run(input),
            Ok(DayResult {
                part1: "7".to_owned(),
                part2: Some("co,de,ka,ta".to_owned())
            })
        );
    }
}
