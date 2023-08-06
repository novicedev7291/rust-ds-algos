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
mod graph;
mod parse;

pub use graph::*;
