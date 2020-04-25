use crate::search::frontiers::{Frontier, PriorityFrontier, QueueFrontier, StackFrontier};
use crate::search::{SearchNode, SearchProblem};

/// Uniform search strategies
/// This module includes a collection of algorithms that systematically tried to find a solution;
/// methods are originally implemented here:
/// https://github.com/aimacode/aima-python/blob/master/search.py

/// depth_first_tree_search
/// it follows first path until it hits wall, then backs up and check other branches;
pub fn depth_first_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = StackFrontier::new();
    let mut explored: Vec<SearchNode> = vec![];

    frontier.add(root_node);
    while let Some(current_node) = frontier.remove() {
        debug_search_node(&current_node);

        if current_node.item().test_goal() {
            return Some(current_node.clone());
        }

        let child_nodes = current_node.expand();
        explored.push(current_node);

        for child_node in child_nodes.into_iter() {
            if !explored.contains(&child_node) {
                frontier.add(child_node);
            }
        }
    }

    None
}

/// Search the shallowest nodes in the search tree first.
/// Search through the successors of a problem to find a goal.
pub fn breadth_first_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = QueueFrontier::new();

    frontier.add(root_node);
    traverse_from_root(&mut frontier)
}

/// search the node by expanding the node n with the lowest path cost g(n).
/// This is done by storing the frontier as a priority queue ordered by g.
pub fn uniform_cost_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = PriorityFrontier::new();
    let mut explored: Vec<SearchNode> = vec![];

    frontier.add(root_node);
    while let Some(current_node) = frontier.remove() {
        debug_search_node(&current_node);

        if current_node.item().test_goal() {
            return Some(current_node.clone());
        }

        let child_nodes = current_node.expand();
        explored.push(current_node);

        for child_node in child_nodes.into_iter() {
            //NB! implementation differs from reference implementation
            // we are not removing a node with worse path_cost
            // because better valued node will anyway be pushed out before
            // than old one; Therefore we are avoiding rebuilding Heap again
            // with cost of polluting it with additional nodes;
            // Although the main reason was that Rust BinaryHeap doesnt support deletion of
            // node; we had to convert heap to list, then remove the item and then
            // build a new node, which was bigger effort than just adding new element
            if !explored.contains(&child_node) {
                frontier.add(child_node);
            }
        }
    }

    None
}

fn traverse_from_root(frontier: &mut impl Frontier) -> Option<SearchNode> {
    let mut explored: Vec<SearchNode> = vec![];

    while let Some(current_node) = frontier.remove() {
        debug_search_node(&current_node);

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
