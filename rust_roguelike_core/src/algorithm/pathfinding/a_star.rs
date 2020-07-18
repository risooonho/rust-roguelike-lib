use crate::algorithm::pathfinding::{CostCalculator, PathfindingAlgorithm, PathfindingResult};
use crate::math::graph::Graph;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

/// The A* search algorithm
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/A*_search_algorithm)
pub struct AStar {}

impl<N, E> PathfindingAlgorithm<N, E> for AStar {
    /// Finds the shortest available path from the start node to the goal node of the graph
    ///
    /// ```
    ///# use rust_roguelike_core::math::graph::occupancy::OccupancyMap;
    ///# use rust_roguelike_core::math::size2d::Size2d;
    ///# use rust_roguelike_core::algorithm::pathfinding::a_star::AStar;
    ///# use rust_roguelike_core::algorithm::pathfinding::{PathfindingAlgorithm, PathfindingResult};
    /// let mut map = OccupancyMap::new(Size2d::new(5, 4), false);
    /// map.add_border();
    /// map.set_node(7, true);
    /// let algorithm = AStar {};
    ///
    /// assert_eq!(algorithm.find(&map, 6, 8),
    ///            PathfindingResult::Path {
    ///              total_cost: 4,
    ///              indices: vec![11, 12, 13, 8],
    ///            });
    /// ```
    fn find<G>(&self, graph: &G, start: usize, goal: usize) -> PathfindingResult
    where
        G: Graph<N, E> + CostCalculator<E>,
        E: Debug,
    {
        println!("Find a path from {} to {}", start, goal);

        let mut open_nodes = BinaryHeap::new();
        open_nodes.push(OpenNode::start(start));

        let mut nodes: HashMap<usize, Node> = HashMap::new();
        nodes.insert(start, Node::new(0));

        while let Some(node) = open_nodes.pop() {
            if node.index == goal {
                return self.create_path(&nodes, goal);
            }

            for neighbor in graph.get_neighbors(node.index) {
                let neighbor_node = nodes
                    .entry(neighbor.index)
                    .or_insert_with(|| Node::new(u32::MAX));

                let cost_to_neighbor = graph.calculate_cost(node.index, &neighbor);
                let new_total_cost = node.total_cost + cost_to_neighbor + neighbor_node.heuristic;

                if new_total_cost < neighbor_node.total_cost {
                    neighbor_node.cost_from_previous = cost_to_neighbor;
                    neighbor_node.total_cost = new_total_cost;
                    neighbor_node.previous = Some(node.index);
                    open_nodes.push(OpenNode::new(neighbor.index, neighbor_node.total_cost));
                }
            }
        }

        PathfindingResult::NoPathFound
    }
}

impl AStar {
    /// Backtracks the path from the goal to the start node
    fn create_path(&self, nodes: &HashMap<usize, Node>, goal: usize) -> PathfindingResult {
        let mut current_node = nodes.get(&goal);
        let total_cost = current_node.unwrap().total_cost;
        let mut current_index = goal;
        let mut indices = Vec::new();

        while let Some(node) = current_node {
            indices.push(current_index);
            current_node = node.previous.and_then(|i| {
                current_index = i;
                nodes.get(&i)
            });
        }

        indices.pop();
        indices.reverse();

        PathfindingResult::Path {
            total_cost,
            indices,
        }
    }
}

#[derive(Copy, Clone)]
struct OpenNode {
    index: usize,
    total_cost: u32,
}

impl OpenNode {
    fn new(index: usize, total_cost: u32) -> Self {
        OpenNode { index, total_cost }
    }

    fn start(index: usize) -> Self {
        OpenNode {
            index,
            total_cost: 0,
        }
    }
}

impl PartialEq for OpenNode {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for OpenNode {}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost.cmp(&self.total_cost)
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Node {
    cost_from_previous: u32,
    heuristic: u32,
    total_cost: u32,
    previous: Option<usize>,
}

impl Node {
    fn new(total_cost: u32) -> Self {
        Node {
            cost_from_previous: 0,
            heuristic: 0,
            total_cost,
            previous: None,
        }
    }
}
