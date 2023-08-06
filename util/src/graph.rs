use parse::parse_array;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge {
    to_node: usize,
    weight: usize,
}

impl Edge {
    fn new(to_node: usize, val: usize) -> Self {
        Self {
            to_node,
            weight: val,
        }
    }

    pub fn to(&self) -> usize {
        self.to_node
    }

    pub fn cost(&self) -> usize {
        self.weight
    }
}

#[derive(Debug)]
pub struct Graph {
    _g: Vec<Vec<Edge>>,
    _type: GraphType,
}

#[derive(Eq, PartialEq, Debug)]
pub enum GraphType {
    DIRECTED,
    UNDIRECTED,
}

#[derive(Debug, Clone)]
pub struct InvalidGraphError {
    msg: String,
}

impl Display for InvalidGraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Graph {
    pub fn new_weighted(
        nodes: usize,
        edges: &str,
        edge_vals: &str,
        g_type: GraphType,
    ) -> Result<Self, InvalidGraphError> {
        parse_graph(nodes, edges, edge_vals, g_type)
    }

    pub fn new(nodes: usize, edges: &str, g_type: GraphType) -> Result<Self, InvalidGraphError> {
        // Capacity because unit weight 1 + same number of , + [ + ]
        let mut edge_vals = String::with_capacity(2 * nodes + 2);
        edge_vals.push('[');
        for i in 0..nodes {
            edge_vals.push('1');
            if i != nodes - 1 {
                edge_vals.push(',');
            }
        }
        edge_vals.push(']');
        parse_graph(nodes, edges, &edge_vals[..], g_type)
    }

    pub fn neighbours(&self, node: usize) -> Vec<usize> {
        self._g
            .get(node)
            .map_or_else(Vec::new, |edges| edges.iter().map(|n| n.to_node).collect())
    }

    pub fn edges_for(&self, node: usize) -> Vec<&Edge> {
        self._g[node].iter().collect()
    }

    pub fn nodes(&self) -> usize {
        self._g.len()
    }
}

fn parse_graph(
    nodes: usize,
    edges: &str,
    edge_vals: &str,
    g_type: GraphType,
) -> Result<Graph, InvalidGraphError> {
    let mut graph: Vec<Vec<Edge>> = Vec::with_capacity(nodes);
    for _ in 0..nodes {
        graph.push(vec![]);
    }

    let weights = parse_array(edge_vals).unwrap();
    if weights.len() != graph.len() {
        return Err(InvalidGraphError {
            msg: "All weights must be provided for edges, only few or more provided than edges!!"
                .to_owned(),
        });
    }

    // To track starting bracked in string slice
    let mut arr_start = false;
    // To track starting bracket in inner array, when closing encountered, it is reset
    let mut inner_sb_i: usize = usize::MAX;
    // To track the edge weight in give edge_vals slice
    let mut edge_i = 0;

    for (i, char) in edges.chars().enumerate() {
        match char {
            '[' => {
                if arr_start {
                    inner_sb_i = i;
                }
                arr_start = true;
            }
            ']' => {
                if inner_sb_i != usize::MAX {
                    let edge = parse_array(&edges[inner_sb_i..i + 1]).unwrap();
                    if edge.len() != 2 {
                        eprint!("Inner edge array should have only two element");
                        return Err(InvalidGraphError {
                            msg: "Inner edge array should have only two element".to_owned(),
                        });
                    }

                    if let Some(node) = graph.get_mut(edge[0]) {
                        node.push(Edge::new(edge[1], weights[edge_i]));
                        use self::GraphType::UNDIRECTED;
                        if g_type == UNDIRECTED {
                            if let Some(node) = graph.get_mut(edge[1]) {
                                node.push(Edge::new(edge[0], weights[edge_i]));
                            }
                        }
                    } else {
                        return Err(InvalidGraphError {
                            msg: "No. of nodes & edges for nodes doesn't match".to_owned(),
                        });
                    }

                    inner_sb_i = usize::MAX;
                    edge_i += 1;
                } else {
                    break;
                }
            }
            ',' | ' ' => continue,
            _ => {
                if inner_sb_i != usize::MAX {
                    continue;
                }
                eprint!("Unknow character encountered during processing edges, exiting!!!");
                return Err(InvalidGraphError {
                    msg: format!("Unknown character in edges string {}", char),
                });
            }
        }
    }
    Ok(Graph {
        _g: graph,
        _type: g_type,
    })
}

#[cfg(test)]
mod tests {
    use super::GraphType::*;
    use super::*;

    #[test]
    fn should_create_graph() {
        let graph = Graph::new(3, "[[0, 1], [1, 2], [2, 0]]", DIRECTED).unwrap();

        assert_eq!(graph._g[0], vec![Edge::new(1, 1)]);
        assert_eq!(graph._g[1], vec![Edge::new(2, 1)]);
        assert_eq!(graph._g[2], vec![Edge::new(0, 1)]);
    }

    #[test]
    fn should_pass_this_test() {
        let graph = Graph::new(4, "[[1, 2], [0, 2], [2, 3], [1, 3]]", DIRECTED).unwrap();

        assert_eq!(graph._g[0], vec![Edge::new(2, 1)]);
        assert_eq!(graph._g[1], vec![Edge::new(2, 1), Edge::new(3, 1)]);
        assert_eq!(graph._g[2], vec![Edge::new(3, 1)]);
        assert_eq!(graph._g[3], vec![]);
    }

    #[test]
    fn should_error_when_invalid_char_in_edges() {
        assert!(Graph::new(2, "[[0,1] , 239, [1, 0]]", DIRECTED).is_err());
    }

    #[test]
    fn should_create_weighted_graph() {
        let g = Graph::new_weighted(
            4,
            "[[1, 2], [0, 2], [2, 3], [1, 3]]",
            "[2, 7, 1, 4]",
            DIRECTED,
        )
        .unwrap();

        assert_eq!(g._g[0], vec![Edge::new(2, 7)]);
        assert_eq!(g._g[1], vec![Edge::new(2, 2), Edge::new(3, 4)]);
        assert_eq!(g._g[2], vec![Edge::new(3, 1)]);
        assert_eq!(g._g[3], vec![]);
    }

    #[test]
    fn should_parse_arrays() {
        let arr_str = "[1, 2, 3, 4]";
        assert_eq!(parse_array(arr_str).unwrap(), vec![1, 2, 3, 4]);

        let arr_str = "[1,2,3,4]";
        assert_eq!(parse_array(arr_str).unwrap(), vec![1, 2, 3, 4]);

        let arr_str = "[121,12, 3, 42192]";
        assert_eq!(parse_array(arr_str).unwrap(), vec![121, 12, 3, 42192]);
    }

    #[test]
    fn should_parse_arr_if_found_from_start() {
        let arr_str = "[1,2],3,2,ab";
        assert_eq!(parse_array(arr_str).unwrap(), vec![1, 2]);
    }

    #[test]
    fn should_error_when_not_starts_with_array() {
        let arr_str = "2,3,4[3,4]";
        assert!(parse_array(arr_str).is_err());
    }

    #[test]
    fn should_error_when_no_end_found() {
        let arr_str = "[1, 2, 3,4";
        assert!(parse_array(arr_str).is_err());
    }

    #[test]
    fn should_create_undirected_graph() -> Result<(), InvalidGraphError> {
        let mut g = Graph::new(3, "[[0,1], [1,2], [2, 0]]", UNDIRECTED)?;

        assert_eq!(
            graph_by_sorted_neighbors(&mut g),
            vec![
                vec![Edge::new(1, 1), Edge::new(2, 1)],
                vec![Edge::new(0, 1), Edge::new(2, 1)],
                vec![Edge::new(0, 1), Edge::new(1, 1)]
            ]
        );

        Ok(())
    }

    #[test]
    fn should_create_undirected_weighted_graph() -> Result<(), InvalidGraphError> {
        let mut g =
            Graph::new_weighted(4, "[[2,3], [0,1], [1,2], [3,1]]", "[2,3,8,1]", UNDIRECTED)?;

        assert_eq!(
            graph_by_sorted_neighbors(&mut g),
            vec![
                vec![Edge::new(1, 3)],
                vec![Edge::new(0, 3), Edge::new(2, 8), Edge::new(3, 1)],
                vec![Edge::new(1, 8), Edge::new(3, 2)],
                vec![Edge::new(1, 1), Edge::new(2, 2)]
            ]
        );
        Ok(())
    }

    fn graph_by_sorted_neighbors(g: &mut Graph) -> Vec<Vec<Edge>> {
        let temp = &mut g._g.to_vec();
        temp.iter_mut().for_each(|e| e.sort());
        temp.to_vec()
    }
}
