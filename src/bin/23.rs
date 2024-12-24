advent_of_code::solution!(23);
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

struct NodeReference {
    node_reference: Rc<RefCell<Node>>,
}

impl std::fmt::Debug for NodeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.node_reference.deref().borrow().name)
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    connections: HashSet<NodeReference>,
}

#[derive(Debug)]
struct Cluster {
    cluster: HashSet<String>,
}

impl Cluster {
    fn new() -> Self {
        Self {
            cluster: HashSet::new(),
        }
    }
    fn insert(&mut self, node: String) {
        if self.cluster.len() > 3 {
            panic!("Cluster with more than 3 nodes")
        } else {
            self.cluster.insert(node);
        }
    }
}

impl Hash for Cluster {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut vector: Vec<String> = self.cluster.clone().into_iter().collect::<Vec<String>>();
        vector.sort();
        vector.join("").hash(state)
    }
}

impl PartialEq for Cluster {
    fn eq(&self, other: &Self) -> bool {
        self.cluster == other.cluster
    }
}

impl Eq for Cluster {}

impl Node {
    fn get_connections(&self) -> Vec<String> {
        self.connections
            .iter()
            .map(|connection| connection.node_reference.deref().borrow().name.clone())
            .collect()
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Node {}

impl Hash for NodeReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_reference.deref().borrow().name.hash(state);
    }
}

impl PartialEq for NodeReference {
    fn eq(&self, other: &Self) -> bool {
        self.node_reference.deref().borrow().name == other.node_reference.deref().borrow().name
    }
}

impl Eq for NodeReference {}

fn get_graph(input: &str) -> HashMap<String, Rc<RefCell<Node>>> {
    let mut nodes = HashMap::new();
    input
        .lines()
        .map(|line| {
            line.split("-")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>()
        })
        .for_each(|connection| {
            let first_node = Node {
                name: connection[0].clone(),
                connections: HashSet::new(),
            };
            let second_node = Node {
                name: connection[0].clone(),
                connections: HashSet::new(),
            };
            nodes.insert(connection[0].clone(), Rc::new(RefCell::new(first_node)));
            nodes.insert(connection[0].clone(), Rc::new(RefCell::new(second_node)));
        });

    input
        .lines()
        .map(|line| {
            line.split("-")
                .map(|str| str.to_owned())
                .collect::<Vec<String>>()
        })
        .for_each(|connection| {
            let first_node = NodeReference {
                node_reference: Rc::clone(nodes.get(&connection[0]).unwrap()),
            };
            let second_node = NodeReference {
                node_reference: Rc::clone(nodes.get(&connection[1]).unwrap()),
            };
            nodes.entry(connection[1].clone()).and_modify(|node| {
                node.deref().borrow_mut().connections.insert(first_node);
            });
            nodes.entry(connection[0].clone()).and_modify(|node| {
                node.deref().borrow_mut().connections.insert(second_node);
            });
        });
    nodes
}

fn print_graph(graph: &HashMap<String, Rc<RefCell<Node>>>) {
    for (name, connections) in graph {
        println!(
            "{name} -> {:?}",
            connections
                .deref()
                .borrow()
                .connections
                .iter()
                .map(|connection| connection.node_reference.deref().borrow().name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}

fn find_clusters(graph: &HashMap<String, Rc<RefCell<Node>>>) -> HashSet<Cluster> {
    let mut clusters = HashSet::new();
    for (_name, node) in graph.iter().take(1) {
        let connections_origin = node.deref().borrow().get_connections();
        for connection_origin in connections_origin.iter() {
            for connection_first in graph[connection_origin]
                .deref()
                .borrow()
                .get_connections()
                .iter()
                .filter(|connection_first| *connection_first != connection_origin)
            {
                for third_node in graph[connection_first]
                    .deref()
                    .borrow()
                    .connections
                    .intersection(&node.deref().borrow().connections)
                    .filter(|third_node| {
                        let name = third_node.node_reference.deref().borrow().name.clone();
                        name != *connection_origin && name != *connection_first
                    })
                {
                    let mut cluster = Cluster::new();
                    cluster.insert(connection_origin.clone());
                    cluster.insert(connection_first.clone());
                    cluster.insert(third_node.node_reference.deref().borrow().name.clone());
                    clusters.insert(cluster);
                }
            }
        }
    }
    clusters
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = get_graph(input);
    print_graph(&graph);
    let clusters = find_clusters(&graph);
    println!("{:?}", clusters);
    // clusters
    //     .iter()
    //     .filter(|cluster| {
    //         cluster
    //             .cluster
    //             .iter()
    //             .filter(|node| node.chars().take(1).any(|letter| letter == 't'))
    //             .count()
    //             > 0
    //     })
    //     .for_each(|cluster| println!("{cluster:?}"));
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
