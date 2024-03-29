use std::cmp::Ord;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use util::Graph;

fn find_path_dfs(from: usize, to: usize, g: &Graph) -> bool {
    let mut stack = vec![];
    stack.push(from);

    let mut seen: HashSet<usize> = HashSet::new();

    while !stack.is_empty() {
        match stack.pop() {
            Some(node) => {
                if node == to {
                    return true;
                }

                if !seen.contains(&node) {
                    seen.insert(node);
                } else {
                    break;
                }

                for e in g.neighbours(node) {
                    stack.push(e);
                }
            }
            None => continue,
        }
    }
    false
}

fn find_path_bfs(from: usize, to: usize, graph: &Graph) -> bool {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut seen: HashSet<usize> = HashSet::new();

    queue.push_back(from);

    while !queue.is_empty() {
        match queue.pop_front() {
            Some(node) => {
                if node == to {
                    return true;
                }

                if !seen.contains(&node) {
                    seen.insert(node);
                } else {
                    continue;
                }

                for e in graph.neighbours(node) {
                    if !seen.contains(&e) {
                        queue.push_back(e);
                    }
                }
            }
            None => continue,
        }
    }

    false
}

pub enum SearchType {
    BFS,
    DFS,
}

pub fn find_path(from: usize, to: usize, g: &Graph, s_type: SearchType) -> bool {
    use SearchType::*;
    match s_type {
        BFS => find_path_bfs(from, to, g),
        DFS => find_path_dfs(from, to, g),
    }
}

pub fn topological_sort(g: &Graph) -> Vec<usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut stack: Vec<usize> = Vec::with_capacity(g.nodes());
    let vertices: Vec<usize> = (0..g.nodes()).collect();

    for v in vertices {
        if !visited.contains(&v) {
            visited.insert(v);
            for e in g.neighbours(v) {
                if !visited.contains(&e) {
                    visited.insert(e);
                    stack.push(e);
                }
            }
            stack.push(v);
        }
    }

    stack.reverse();
    stack[..].to_vec()
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Pair {
    i: usize,
    cost: usize,
}

/// This function implements Dijkstra's algorithm
pub fn find_distance(g: &Graph, from_node: usize, to_node: usize) -> usize {
    let mut dist = vec![usize::MAX; g.nodes()];

    let mut min_q = BinaryHeap::with_capacity(g.nodes());
    dist[from_node] = 0;
    min_q.push(Pair {
        i: from_node,
        cost: 0,
    });

    while !min_q.is_empty() {
        if let Some(Pair { i, cost }) = min_q.pop() {
            if i == to_node {
                return cost;
            }

            for e in g.edges_for(i) {
                let next = e.to();
                let next_cost = e.cost();
                if (cost + next_cost) < dist[next] {
                    dist[next] = cost + next_cost;
                }
                min_q.push(Pair {
                    i: next,
                    cost: dist[next],
                });
            }
        }
    }

    0
}

fn find_parent(parent: &Vec<usize>, node: usize) -> usize {
    if node == parent[node] {
        return node;
    }
    find_parent(parent, parent[node])
}

fn union(parent: &mut Vec<usize>, n_i: usize, n_j: usize) {
    let n_i_p = find_parent(parent, n_i);
    let n_j_p = find_parent(parent, n_j);

    parent[n_j_p] = n_i_p;
}

#[derive(Debug, Eq)]
struct Path {
    from: usize,
    to: usize,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to)
            || (self.from == other.to && self.to == other.from)
    }
}

impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.from > self.to {
            self.to.hash(state);
            self.from.hash(state);
        } else {
            self.from.hash(state);
            self.to.hash(state);
        }
    }
}

pub fn has_cycle(g: &Graph) -> bool {
    let mut parent = (0..g.nodes()).collect::<Vec<usize>>();
    let edges = g
        .all_edges()
        .iter()
        .map(|e| Path { from: e.0, to: e.1 })
        .collect::<HashSet<Path>>();

    println!("{:?}", edges);
    for e in edges {
        let i = e.from;
        let j = e.to;

        let n_i_p = find_parent(&parent, i);
        let n_j_p = find_parent(&parent, j);

        if n_i_p == n_j_p {
            return true;
        }

        union(&mut parent, n_i_p, n_j_p);
    }

    false
}
