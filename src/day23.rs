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
        let input = Input::from(input);

        input
            .clique_3()
            .iter()
            .filter(|vec| {
                vec.iter()
                    .any(|id| input.computers_reverse[id].starts_with('t'))
            })
            .count() as u32
    }
}

pub struct Part2;

impl Part<String> for Part2 {
    fn expect_test(&self) -> String {
        "co,de,ka,ta".to_string()
    }

    fn solve(&self, input: &[String]) -> String {
        let input = Input::from(input);

        let mut result = input
            .clique_max()
            .into_iter()
            .max_by_key(|vec| vec.len())
            .unwrap()
            .iter()
            .map(|id| input.computers_reverse[id])
            .collect::<Vec<_>>();
        
        result.sort();
        
        result
            .join(",")
            .to_string()
    }
}

struct Input<'a> {
    computers_reverse: HashMap<usize, &'a str>,
    connections: HashMap<usize, HashSet<usize>>,
}

impl<'a> From<&'a [String]> for Input<'a> {
    fn from(value: &'a [String]) -> Self {
        let mut id = 0_usize;

        let mut computers = HashMap::new();
        let mut computers_reverse = HashMap::new();

        let connections = value
            .iter()
            .filter(|e| !e.is_empty())
            .map(|s| s.split("-"))
            .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, mut e| {
                let a = e.next().unwrap();
                let b = e.next().unwrap();

                let a_id = *computers.entry(a).or_insert_with(|| {
                    let r = id;
                    computers_reverse.insert(r, a);
                    id += 1;
                    r
                });

                let b_id = *computers.entry(b).or_insert_with(|| {
                    let r = id;
                    computers_reverse.insert(r, b);
                    id += 1;
                    r
                });

                acc.entry(a_id).or_default().insert(b_id);
                acc.entry(b_id).or_default().insert(a_id);

                acc
            });

        Self {
            computers_reverse,
            connections,
        }
    }
}

impl<'a> Input<'a> {
    fn num_computers(&self) -> usize {
        self.computers_reverse.len()
    }
    
    fn clique_max(&'a self) -> HashSet<Vec<usize>> {
        let mut result = HashSet::new();

        for mut clique in self.clique_3() {
            // skip sub cliques that already have their maximal clique calculated
            if result
                .iter()
                .any(|max_clique: &Vec<_>| clique.iter().all(|e| max_clique.contains(e)))
            {
                continue;
            }

            for candidate in 0..self.num_computers() {
                if clique
                    .iter()
                    .all(|&computer| self.connections[&computer].contains(&candidate))
                {
                    clique.push(candidate);
                }
            }

            clique.sort();
            result.insert(clique);
        }

        result
    }

    fn clique_3(&'a self) -> HashSet<Vec<usize>> {
        let mut result = HashSet::new();

        for (&computer_1, connections_1) in &self.connections {
            for &computer_2 in connections_1 {
                let connections_2 = self.connections.get(&computer_2).unwrap();

                for &computer_3 in connections_1.intersection(connections_2) {
                    let mut clique = vec![computer_1, computer_2, computer_3];

                    clique.sort();
                    result.insert(clique);
                }
            }
        }

        result
    }
}
