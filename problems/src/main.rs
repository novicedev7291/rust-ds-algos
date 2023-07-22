use util::GraphType::*;
use util::{Graph, InvalidGraphError};

fn find_path_dfs(from: usize, to: usize, g: &Graph) -> bool {
    let mut seen = vec![0; g.nodes()];
    let mut stack = vec![];
    stack.push(from);

    loop {
        if let Some(start) = stack.pop() {
            seen[start] = 1;
            match g.edges_for(start) {
                Some(edges) => {
                    for n in edges {
                        if *n == to {
                            return true;
                        }
                        match seen.get(*n) {
                            Some(val) => {
                                if *val != 1 {
                                    stack.push(*val);
                                    break;
                                }
                            }
                            None => panic!("This must not happen!!!"),
                        }
                    }
                }
                None => continue,
            }

            if seen.iter().all(|x| *x == 1) {
                break;
            }
        }
    }
    false
}

fn find_path_bfs(from: usize, to: usize, graph: &Graph) -> bool {
    let mut stack = vec![from];
    stack.push(from);
    let mut seen = vec![0; graph.nodes()];

    while !stack.is_empty() {
        if let Some(node) = stack.pop() {
            match graph.edges_for(node) {
                Some(edges) => {
                    for e in edges {
                        if *e == to {
                            return true;
                        }

                        match seen.get(*e) {
                            Some(val) => {
                                if *val != 1 {
                                    stack.push(*e);
                                }
                            }
                            None => {
                                panic!("find_path panicked while checking seen array for neighbours, this must not happen!!!");
                            }
                        }
                    }
                    seen[node] = 1;
                }
                None => {
                    continue;
                }
            }
        }
    }
    false
}

fn main() -> Result<(), InvalidGraphError> {
    // Problem 1: Given a directed graph as below
    // [[1, 2], [0, 2], [2, 3], [1, 3]]
    // Find if there is a path from 1 -> 0
    // Find if there is a path from 0 -> 3

    let graph = Graph::new(4, "[[1, 2], [0, 2], [2, 3], [1, 3]]", DIRECTED)?;

    assert!(!find_path_bfs(1, 0, &graph));
    assert!(find_path_bfs(0, 3, &graph));

    let graph = Graph::new(6, "[[0,1],[0,2],[3,5],[5,4],[4,3]]", DIRECTED)?;
    assert!(!find_path_dfs(0, 5, &graph));

    let graph = Graph::new(3, "[[0,1],[1,2],[2,0]]", DIRECTED)?;
    assert!(find_path_dfs(0, 2, &graph));

    Ok(())
}
