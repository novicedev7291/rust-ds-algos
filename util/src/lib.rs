// A directed graph can be represented in two ways
// 1. Sparse array of array where index in first array
// denotes the nodes and the other array at each index denotes
// the nodes the index node can reach.
// [0] -> [2, 4, 9]
// [1] -> [3, 10, 19]
// [2] -> [2, 3, 4]
// or
// Using a matrix where row & column denotes the nodes and
// cell tells if node A having path to node B
//  0  1  2  3  4
//0[1, 0, 1, 0, 1 ]
//1[0, 1, 0, 1, 0 ]
// Matrix representation best suited for indirect graph

use std::fmt::Display;

#[derive(Debug)]
pub struct Graph {
    _g: Vec<Vec<usize>>,
    _type: GraphType,
}

#[derive(Eq, PartialEq, Debug)]
pub enum GraphType {
    DIRECTED,
    INDIRECTED,
}

#[derive(Debug, Clone)]
struct ArrParseError {
    msg: String,
}

impl Display for ArrParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

fn parse_array(arr_str: &str) -> Result<Vec<usize>, ArrParseError> {
    let mut result = Vec::new();
    let mut num = String::from("");
    let mut arr_start = false;
    let mut arr_end = false;
    for char in arr_str.chars() {
        match char {
            '[' | ' ' => {
                arr_start = true;
                continue;
            }
            ',' => {
                result.push(num.to_owned().parse::<usize>().unwrap());
                num.clear();
            }
            ']' => {
                result.push(num.to_owned().parse::<usize>().unwrap());
                arr_end = true;
                break;
            }
            _ => {
                if !arr_start {
                    return Err(ArrParseError {
                        msg: "Expecting [ at the start of array".to_string(),
                    });
                }
                num.push(char);
            }
        }
    }

    if !arr_end {
        return Err(ArrParseError {
            msg: "Expecting ] at the end of array".to_string(),
        });
    }

    Ok(result)
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
    pub fn new(nodes: usize, edges: &str, g_type: GraphType) -> Result<Self, InvalidGraphError> {
        if g_type == GraphType::INDIRECTED {
            unimplemented!();
        }

        let mut graph: Vec<Vec<usize>> = Vec::with_capacity(nodes);
        for _ in 0..nodes {
            graph.push(vec![]);
        }

        let mut arr_start = false;
        let mut inner_arr_i: usize = usize::MAX;

        for (i, char) in edges.chars().enumerate() {
            match char {
                '[' => {
                    if arr_start {
                        inner_arr_i = i;
                    }
                    arr_start = true;
                }
                ']' => {
                    if inner_arr_i != usize::MAX {
                        let edge = parse_array(&edges[inner_arr_i..i + 1]).unwrap();
                        if edge.len() != 2 {
                            eprint!("Inner edge array should have only two element");
                            return Err(InvalidGraphError {
                                msg: "Inner edge array should have only two element".to_owned(),
                            });
                        }

                        if let Some(node) = graph.get_mut(edge[0]) {
                            node.push(edge[1]);
                        } else {
                            return Err(InvalidGraphError {
                                msg: "No. of nodes & edges for nodes doesn't match".to_owned(),
                            });
                        }

                        inner_arr_i = usize::MAX;
                    } else {
                        break;
                    }
                }
                ',' | ' ' => continue,
                _ => {
                    if inner_arr_i != usize::MAX {
                        continue;
                    }
                    eprint!("Unknow character encountered during processing edges, exiting!!!");
                    return Err(InvalidGraphError {
                        msg: format!("Unknown character in edges string {}", char),
                    });
                }
            }
        }
        Ok(Self {
            _g: graph,
            _type: g_type,
        })
    }

    pub fn edges_for(&self, node: usize) -> Option<&Vec<usize>> {
        self._g.get(node)
    }

    pub fn nodes(&self) -> usize {
        self._g.len()
    }
}

#[cfg(test)]
mod tests {
    use super::GraphType::*;
    use super::*;

    #[test]
    fn should_create_graph() {
        let graph = Graph::new(3, "[[0, 1], [1, 2], [2, 0]]", DIRECTED).unwrap();

        assert_eq!(graph._g[0], vec![1]);
        assert_eq!(graph._g[1], vec![2]);
        assert_eq!(graph._g[2], vec![0]);
    }

    #[test]
    fn should_pass_this_test() {
        let graph = Graph::new(4, "[[1, 2], [0, 2], [2, 3], [1, 3]]", DIRECTED).unwrap();

        assert_eq!(graph._g[0], vec![2]);
        assert_eq!(graph._g[1], vec![2, 3]);
        assert_eq!(graph._g[2], vec![3]);
        assert_eq!(graph._g[3], vec![]);
    }

    #[test]
    fn should_error_when_invalid_char_in_edges() {
        assert!(Graph::new(2, "[[0,1] , 239, [1, 0]]", DIRECTED).is_err());
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
}