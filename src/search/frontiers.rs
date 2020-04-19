use crate::search::SearchNode;
use std::collections::VecDeque;

pub trait Frontier {
    fn add(&mut self, node: SearchNode);
    fn remove(&mut self) -> Option<SearchNode>;
    fn is_empty(&self) -> bool;
    fn contains(&self, other: &SearchNode) -> bool;
    fn len(&self) -> usize;
}

pub struct StackFrontier {
    collection: Vec<SearchNode>,
}

impl StackFrontier {
    pub fn new() -> Self {
        StackFrontier { collection: vec![] }
    }
}

impl Frontier for StackFrontier {
    fn add(&mut self, node: SearchNode) {
        self.collection.push(node);
    }
    fn remove(&mut self) -> Option<SearchNode> {
        self.collection.pop()
    }
    fn is_empty(&self) -> bool {
        self.collection.is_empty()
    }
    fn contains(&self, other: &SearchNode) -> bool {
        self.collection.contains(other)
    }
    fn len(&self) -> usize {
        self.collection.len()
    }
}

/// it works like pipe - first thing that go in, will come out first
pub struct QueueFrontier {
    collection: VecDeque<SearchNode>
}

impl QueueFrontier {
    pub fn new() -> Self { QueueFrontier { collection: VecDeque::new() } }
}

impl Frontier for QueueFrontier {
    fn add(&mut self, node: SearchNode) { self.collection.push_back(node); }
    fn remove(&mut self) -> Option<SearchNode> { self.collection.pop_front() }
    fn is_empty(&self) -> bool { self.collection.is_empty() }
    fn contains(&self, other: &SearchNode) -> bool { self.collection.contains(other) }
    fn len(&self) -> usize { self.collection.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::Action;
    use crate::search::SearchProblem;

    struct TestSearchProblem {
        item: u32,
    }

    impl SearchProblem for TestSearchProblem {
        fn actions(&self) -> Vec<Action> {
            vec![]
        }

        fn result(&self, action: &Action) -> Box<dyn SearchProblem> {
            Box::new(TestSearchProblem { item: 0 })
        }

        fn test_goal(&self) -> bool {
            true
        }
        fn path_cost(&self) -> u32 {
            0
        }
        fn value(&self) -> u32 {
            self.item
        }
        fn as_string(&self) -> String {
            "test_problem".to_string()
        }
        fn hash_code(&self) -> u64 {
            0
        }
    }

    #[test]
    fn test_stack_frontier_creates_new_empty_frontier() {
        assert!(StackFrontier::new().is_empty())
    }

    #[test]
    fn test_stack_frontier_adds_new_item() {
        let mut frontier = StackFrontier::new();
        let test_problem = Box::new(TestSearchProblem { item: 0 });

        assert!(frontier.is_empty());
        frontier.add(SearchNode::root(test_problem));
        assert!(!frontier.is_empty());
    }

    #[test]
    fn test_stack_frontier_contains_returns_false_when_empty() {
        let frontier = StackFrontier::new();
        let test_problem = Box::new(TestSearchProblem { item: 1 });
        let marker_node = SearchNode::root(test_problem);

        assert_eq!(false, frontier.contains(&marker_node));
    }

    #[test]
    fn test_stack_frontier_contains_returns_true_if_element_exists() {
        let mut frontier = StackFrontier::new();
        let test_problem = Box::new(TestSearchProblem { item: 1 });
        let marker_node = SearchNode::root(test_problem);

        frontier.add(marker_node.clone());

        assert!(frontier.contains(&marker_node));
    }

    #[test]
    fn test_stack_frontier_remove_if_frontier_is_empty() {
        let mut frontier = StackFrontier::new();

        assert!(frontier.remove().is_none());
    }

    #[test]
    fn test_stack_frontier_remove_if_frontier_has_item() {
        let mut frontier = StackFrontier::new();
        let test_problem = Box::new(TestSearchProblem { item: 1 });
        let marker_node = SearchNode::root(test_problem);

        frontier.add(marker_node.clone());

        assert!(frontier.remove().is_some());
    }

    // QueueFrontier

    #[test]
    fn test_queue_frontier_creates_new_empty_frontier() { assert!(QueueFrontier::new().is_empty()) }

    #[test]
    fn test_queue_frontier_adds_new_item() {
        let mut frontier = QueueFrontier::new();
        let test_problem = Box::new(TestSearchProblem { item: 0});

        assert!(frontier.is_empty());
        frontier.add(SearchNode::root(test_problem));
        assert!(!frontier.is_empty());
    }

    #[test]
    fn test_queue_frontier_contains_false_when_empty() {
        let frontier = QueueFrontier::new();
        let test_problem = Box::new(TestSearchProblem { item: 2} );
        let marker_node = SearchNode::root(test_problem);

        assert_eq!(false, frontier.contains(&marker_node));
    }

    #[test]
    fn test_queue_frontier_contains_true_if_element_exists() {
        let mut frontier = QueueFrontier::new();
        let test_problem = Box::new(TestSearchProblem { item: 3});
        let marker_node = SearchNode::root(test_problem);

        frontier.add(marker_node.clone());
        assert!(frontier.contains(&marker_node));
    }

    #[test]
    fn test_queue_frontier_remove_if_frontier_is_empty() {
        let mut frontier = QueueFrontier::new();

        assert!(frontier.remove().is_none());
    }

    #[test]
    fn test_queue_frontier_remove_if_frontier_has_item() {
        let mut frontier = QueueFrontier::new();
        let test_problem = Box::new(TestSearchProblem {item: 4});
        let marker_node = SearchNode::root(test_problem);

        frontier.add(marker_node.clone());

        assert!(frontier.remove().is_some());
    }

    fn make_test_node(item_val: u32) -> SearchNode {
        let test_problem = Box::new(TestSearchProblem {item: item_val});
        SearchNode::root(test_problem)
    }

    #[test]
    fn test_queue_frontier_remove_works_like_fifo() {
        let mut frontier = QueueFrontier::new();
        let first_node = make_test_node(5);
        let second_node = make_test_node(6);

        frontier.add(first_node);
        frontier.add(second_node);

        assert_eq!(5, frontier.remove().unwrap().item().value());
        assert_eq!(6, frontier.remove().unwrap().item().value());
    }
}
