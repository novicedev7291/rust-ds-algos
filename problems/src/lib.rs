pub mod graph;

#[cfg(test)]
mod tests {
    use super::graph::SearchType::*;
    use super::*;
    use util::GraphType::*;
    use util::{Graph, InvalidGraphError};

    #[test]
    fn should_find_path_using_bfs() -> Result<(), InvalidGraphError> {
        // Problem 1: Given a directed graph as below
        // [[1, 2], [0, 2], [2, 3], [1, 3]]
        // Find if there is a path from 1 -> 0
        // Find if there is a path from 0 -> 3

        let g = Graph::new(4, "[[1, 2], [0, 2], [2, 3], [1, 3]]", DIRECTED)?;

        assert!(!graph::find_path(1, 0, &g, BFS));
        assert!(graph::find_path(0, 3, &g, BFS));

        Ok(())
    }

    #[test]
    fn should_find_path_using_dfs() -> Result<(), InvalidGraphError> {
        let g = Graph::new(4, "[[1, 2], [0, 2], [2, 3], [1, 3]]", DIRECTED)?;
        assert!(graph::find_path(0, 3, &g, DFS));

        let g = Graph::new(6, "[[0,1],[0,2],[3,5],[5,4],[4,3]]", DIRECTED)?;
        assert!(!graph::find_path(0, 5, &g, DFS));

        let g = Graph::new(3, "[[0,1],[1,2],[2,0]]", DIRECTED)?;
        assert!(graph::find_path(0, 2, &g, DFS));

        Ok(())
    }
}
