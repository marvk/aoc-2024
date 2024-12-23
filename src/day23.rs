use crate::harness::Day;
use crate::harness::Part;
use std::collections::{HashMap, HashSet};

pub fn day23() -> Day<u32, String> {
    Day::new(23, Box::new(Part1 {}), Box::new(Part2 {}))
}

pub struct Part1;

impl Part<u32> for Part1 {
    fn expect_test(&self) -> u32 {
        7
    }

    fn solve(&self, input: &[String]) -> u32 {
        Input::from(input).clique_3().len() as u32
    }
}

pub struct Part2;

impl Part<String> for Part2 {
    fn expect_test(&self) -> String {
        "co,de,ka,ta".to_string()
    }

    fn solve(&self, input: &[String]) -> String {
        Input::from(input)
            .clique_max()
            .into_iter()
            .max_by_key(|vec| vec.len())
            .unwrap()
            .join(",")
            .to_string()
    }
}

struct Input<'a> {
    computers: Vec<&'a str>,
    connections: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> From<&'a [String]> for Input<'a> {
    fn from(value: &'a [String]) -> Self {
        let mut computers = HashSet::new();
        let connections = value
            .iter()
            .filter(|e| !e.is_empty())
            .map(|s| s.split("-"))
            .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, mut e| {
                let a = e.next().unwrap();
                let b = e.next().unwrap();

                computers.insert(a);
                computers.insert(b);

                acc.entry(a).or_default().insert(b);
                acc.entry(b).or_default().insert(a);

                acc
            });

        let mut computers = computers.into_iter().collect::<Vec<_>>();
        computers.sort();

        Self {
            computers,
            connections,
        }
    }
}

impl<'a> Input<'a> {
    fn clique_max(&'a self) -> HashSet<Vec<&'a str>> {
        let mut result = HashSet::new();

        for mut clique in self.clique_3() {
            // skip sub cliques that already have their maximal clique calculated
            if result
                .iter()
                .any(|max_clique: &Vec<_>| clique.iter().all(|e| max_clique.contains(e)))
            {
                continue;
            }

            for &candidate in &self.computers {
                if clique
                    .iter()
                    .all(|&computer| self.connections[computer].contains(candidate))
                {
                    clique.push(candidate);
                }
            }

            clique.sort();
            result.insert(clique);
        }

        result
    }

    fn clique_3(&'a self) -> HashSet<Vec<&'a str>> {
        let mut result = HashSet::new();

        for (&computer_1, connections_1) in &self.connections {
            for &computer_2 in connections_1 {
                let connections_2 = self.connections.get(computer_2).unwrap();

                for &computer_3 in connections_1.intersection(connections_2) {
                    let mut clique = vec![computer_1, computer_2, computer_3];

                    if clique.iter().any(|c| c.starts_with('t')) {
                        clique.sort();
                        result.insert(clique);
                    }
                }
            }
        }

        result
    }
}
