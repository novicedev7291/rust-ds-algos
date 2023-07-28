use std::collections::HashSet;
use std::collections::VecDeque;
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

                for e in g.edges_for_node(node) {
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

                for e in graph.edges_for_node(node) {
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
            for e in g.edges_for_node(v) {
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
