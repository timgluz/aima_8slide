use crate::search::frontiers::{Frontier, PriorityFrontier, QueueFrontier, StackFrontier};
use crate::search::{SearchNode, SearchProblem};

/// Uniform search strategies
/// This module includes a collection of algorithms that systematically tried to find a solution;
/// methods are originally implemented here:
/// https://github.com/aimacode/aima-python/blob/master/search.py

/// depth_first_tree_search
/// it follows first path until it hits wall, then backs up and check other branches;
/// TODO: fix infinite loop;
pub fn depth_first_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = StackFrontier::new();

    frontier.add(root_node);
    traverse_frontier(&mut frontier)
}

/// Search the shallowest nodes in the search tree first.
/// Search through the successors of a problem to find a goal.
pub fn breadth_first_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = QueueFrontier::new();

    frontier.add(root_node);
    traverse_frontier(&mut frontier)
}

/// search the node by expanding the node n with the lowest path cost g(n).
/// This is done by storing the frontier as a priority queue ordered by g.
///NB! implementation differs from reference implementation
/// we are not removing a node with worse path_cost
/// because better valued node will anyway be pushed out before
/// than old one; Therefore we are avoiding rebuilding Heap again
/// with cost of polluting it with additional nodes;
/// Although the main reason was that Rust BinaryHeap doesnt support deletion of
/// node; we had to convert heap to list, then remove the item and then
/// build a new node, which was bigger effort than just adding new element
pub fn uniform_cost_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = PriorityFrontier::new();

    frontier.add(root_node);
    traverse_frontier(&mut frontier)
}

/// Iterative deepening search is a general strategy often used in combination with DFS,
/// that finds the best depth limit. It does this by gradually increasing the limit
/// until the goal is found
pub fn iterative_deepening_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);

    for limit in 0..usize::MAX {
        if let Some(res) = recursive_dls(&root_node, limit) {
            return Some(res);
        }
    }

    None
}

/// This algorithm works around DFS issue of infinite-path problem
/// by cut-offing the search after reaching to the specified depth;
pub fn depth_limited_search(problem: Box<dyn SearchProblem>, limit: usize) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);

    recursive_dls(&root_node, limit)
}

fn recursive_dls(node: &SearchNode, limit: usize) -> Option<SearchNode> {
    if node.item().test_goal() == true {
        return Some(node.clone());
    }

    if limit == 0 {
        return None;
    }

    for child in node.expand().iter() {
        if let Some(res) = recursive_dls(&child, limit - 1) {
            return Some(res);
        }
    }

    None
}

// utils ----

fn traverse_frontier(frontier: &mut impl Frontier) -> Option<SearchNode> {
    let mut explored: Vec<SearchNode> = vec![];

    while let Some(current_node) = frontier.remove() {
        //debug_search_node(&current_node);

        if current_node.item().test_goal() {
            return Some(current_node.clone());
        }

        let child_nodes = current_node.expand();
        explored.push(current_node);

        for child_node in child_nodes.into_iter() {
            if !explored.contains(&child_node) {
                frontier.add(child_node)
            }
        }
    }

    None
}

fn debug_search_node(current_node: &SearchNode) {
    println!(
        "step.{:?} {:?} - {:?}",
        current_node.depth(),
        current_node.action(),
        current_node.item()
    );
}
