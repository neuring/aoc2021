use std::collections::HashMap;

use anyhow::{anyhow, ensure};

use crate::graph::{Graph, GraphAttribute, NodeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaveSize {
    Large,
    Small,
}

struct ParseResult<'a> {
    graph: Graph,
    cave_sizes: GraphAttribute<CaveSize>,
    names: GraphAttribute<&'a str>,
}

fn parse<'a>(input: &'a str) -> anyhow::Result<ParseResult<'a>> {
    let mut result = Graph::new();
    let mut cave_sizes = GraphAttribute::new();

    let mut name_to_id = HashMap::new();
    let mut names = GraphAttribute::new();

    let mut get_node_id = |node_name: &'a str, graph: &mut Graph| {
        if let Some(node_id) = name_to_id.get(node_name) {
            *node_id
        } else {
            let cave_size = if node_name.chars().all(|c| c.is_ascii_uppercase()) {
                CaveSize::Large
            } else {
                CaveSize::Small
            };

            let node_id = graph.add_node();

            cave_sizes.insert(node_id, cave_size);
            name_to_id.insert(node_name, node_id);
            names.insert(node_id, node_name);
            node_id
        }
    };

    for line in input.trim().lines() {
        let mut parts = line.trim().split('-');

        let start = parts.next().ok_or(anyhow!("Missing start of edge"))?;
        let end = parts.next().ok_or(anyhow!("Missing end of edge"))?;

        ensure!(parts.next().is_none(), "Unexpected next part of edge");

        let start_id = get_node_id(start, &mut result);
        let end_id = get_node_id(end, &mut result);
        result.add_edge(start_id, end_id);
    }

    Ok(ParseResult {
        graph: result,
        cave_sizes,
        names,
    })
}

struct Solver<'a> {
    graph: &'a Graph,
    cave_sizes: &'a GraphAttribute<CaveSize>,
    end_node: NodeId,
    start_node: NodeId,
    visited: HashMap<NodeId, u32>,
    have_visited_small_cave_twice: bool,

    can_visit_single_cave_twice: bool,
}

impl<'a> Solver<'a> {
    fn count_paths_recursive(&mut self, node: NodeId) -> u32 {
        let mut paths = 0;

        if self.cave_sizes[node] == CaveSize::Small {
            if let Some(number_visited) = self.visited.get_mut(&node) {
                let max_visit_allowed = if self.can_visit_single_cave_twice
                    && !self.have_visited_small_cave_twice
                {
                    2
                } else {
                    1
                };

                if *number_visited >= max_visit_allowed {
                    return 0;
                } else {
                    *number_visited += 1;

                    if *number_visited == 2 {
                        self.have_visited_small_cave_twice = true;
                    }
                }
            } else {
                self.visited.insert(node, 1);
            }
        }

        for &neighbor_id in self.graph.get_node(node).neighbors() {
            if neighbor_id == self.end_node {
                paths += 1;
            } else if neighbor_id == self.start_node {
                paths += 0;
            } else {
                paths += self.count_paths_recursive(neighbor_id);
            }
        }

        if self.cave_sizes[node] == CaveSize::Small {
            let number_visited = self.visited.get_mut(&node).unwrap();
            if *number_visited == 2 {
                assert!(self.can_visit_single_cave_twice);
                self.have_visited_small_cave_twice = false;
            }
            *number_visited -= 1;
        }

        paths
    }
}

fn solve(input: &str, can_visit_single_cave_twice: bool) -> anyhow::Result<u32> {
    let ParseResult {
        graph,
        cave_sizes,
        names,
    } = parse(input)?;

    let start_node = names
        .iter()
        .find(|&(_, name)| *name == "start")
        .map(|(id, _)| id)
        .ok_or(anyhow!("Missing start node"))?;
    let end_node = names
        .iter()
        .find(|&(_, name)| *name == "end")
        .map(|(id, _)| id)
        .ok_or(anyhow!("Missing end node"))?;

    let mut solver = Solver {
        graph: &graph,
        cave_sizes: &cave_sizes,
        end_node,
        start_node,
        visited: HashMap::new(),
        have_visited_small_cave_twice: false,
        can_visit_single_cave_twice,
    };

    let result = solver.count_paths_recursive(start_node);

    Ok(result)
}

pub fn part1(text: &str) -> anyhow::Result<u32> {
    solve(text, false)
}

pub fn part2(text: &str) -> anyhow::Result<u32> {
    solve(text, true)
}
