pub mod graph;

#[cfg(test)]
mod tests {
    use super::graph::SearchType::*;
    use super::*;
    use util::{Graph, InvalidGraphError, WeightedGraph};
    use util::{GraphType::*, UnitWeightedGraph};

    #[test]
    fn should_find_path_using_bfs() -> Result<(), InvalidGraphError> {
        // Problem 1: Given a directed graph as below
        // [[1, 2], [0, 2], [2, 3], [1, 3]]
        // Find if there is a path from 1 -> 0
        // Find if there is a path from 0 -> 3

        let g = <Graph as UnitWeightedGraph>::new(4, "[[1, 2], [0, 2], [2, 3], [1, 3]]", DIRECTED)?;

        assert!(!graph::find_path(1, 0, &g, BFS));
        assert!(graph::find_path(0, 3, &g, BFS));

        Ok(())
    }

    #[test]
    fn should_find_path_using_dfs() -> Result<(), InvalidGraphError> {
        let g = <Graph as UnitWeightedGraph>::new(4, "[[1, 2], [0, 2], [2, 3], [1, 3]]", DIRECTED)?;
        assert!(graph::find_path(0, 3, &g, DFS));

        let g = <Graph as UnitWeightedGraph>::new(6, "[[0,1],[0,2],[3,5],[5,4],[4,3]]", DIRECTED)?;
        assert!(!graph::find_path(0, 5, &g, DFS));

        let g = <Graph as UnitWeightedGraph>::new(3, "[[0,1],[1,2],[2,0]]", DIRECTED)?;
        assert!(graph::find_path(0, 2, &g, DFS));

        let g = <Graph as WeightedGraph>::new(
            6,
            "[[0,1], [0,3], [1, 2], [3, 2], [2, 4], [4, 5]]",
            "[2, 3, 6, 3, 4, 2 ]",
            DIRECTED,
        )?;
        assert!(graph::find_path(0, 5, &g, DFS));

        Ok(())
    }

    #[test]
    fn should_match_topo_logical_sort_given_graph() -> Result<(), InvalidGraphError> {
        let g = <Graph as UnitWeightedGraph>::new(
            6,
            "[[5,2], [5,0], [4, 0], [4,1], [2, 3], [3, 1]]",
            DIRECTED,
        )?;
        assert_eq!(graph::topological_sort(&g), vec![5, 4, 2, 3, 1, 0]);

        let g = <Graph as UnitWeightedGraph>::new(4, "[[1, 0], [2, 0], [3, 0]]", DIRECTED)?;
        assert_eq!(graph::topological_sort(&g), vec![3, 2, 1, 0]);

        Ok(())
    }

    #[test]
    fn should_find_shortest_distance_to_given_node_from_start() -> Result<(), InvalidGraphError> {
        let g = <Graph as WeightedGraph>::new(
            6,
            "[[0,1], [0,3], [1, 2], [3, 2], [2, 4], [4, 5]]",
            "[2, 3, 6, 3, 4, 2 ]",
            DIRECTED,
        )?;

        assert_eq!(graph::find_distance(&g, 0, 5), 12);
        assert_eq!(graph::find_distance(&g, 1, 5), 12);
        assert_eq!(graph::find_distance(&g, 3, 4), 7);

        Ok(())
    }
}
