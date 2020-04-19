use crate::search::{SearchNode, SearchProblem};
use crate::search::frontiers::{Frontier, StackFrontier};

pub fn depth_first_tree_search(problem: Box<dyn SearchProblem>) -> Option<SearchNode> {
    let root_node = SearchNode::root(problem);
    let mut frontier = StackFrontier::new();
    let mut explored: Vec<SearchNode> = vec![];

    frontier.add(root_node);
    loop {
        match frontier.remove() {
            None => break,
            Some(current_node) => {
                //debug_search_node(&current_node);

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
