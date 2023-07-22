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

                match g.edges_for(node) {
                    Some(edges) => {
                        for e in edges {
                            stack.push(*e);
                        }
                    }
                    None => continue,
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

                match graph.edges_for(node) {
                    Some(edges) => {
                        for e in edges {
                            if !seen.contains(e) {
                                queue.push_back(*e);
                            }
                        }
                    }
                    None => continue,
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
