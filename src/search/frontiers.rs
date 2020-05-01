use crate::search::SearchNode;
use std::collections::BinaryHeap;
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
    collection: VecDeque<SearchNode>,
}

impl QueueFrontier {
    pub fn new() -> Self {
        QueueFrontier {
            collection: VecDeque::new(),
        }
    }
}

impl Frontier for QueueFrontier {
    fn add(&mut self, node: SearchNode) {
        self.collection.push_back(node);
    }
    fn remove(&mut self) -> Option<SearchNode> {
        self.collection.pop_front()
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

/// PriorityFrontier takes best node first
pub struct PriorityFrontier {
    collection: BinaryHeap<SearchNode>, // relies default impl of Ord
}

impl PriorityFrontier {
    pub fn new() -> Self {
        PriorityFrontier {
            collection: BinaryHeap::new(),
        }
    }
}

impl Frontier for PriorityFrontier {
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
        self.collection.iter().find(|&n| n.eq(other)).is_some()
    }
    fn len(&self) -> usize {
        self.collection.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::{Action, SearchProblem};

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
            "test_node".to_string()
        }
        fn hash_code(&self) -> u64 {
            0
        }
    }

    fn make_test_node(item_val: u32) -> SearchNode {
        let test_node = Box::new(TestSearchProblem { item: item_val });
        SearchNode::root(test_node)
    }

    #[test]
    fn test_stack_frontier_creates_new_empty_frontier() {
        assert!(StackFrontier::new().is_empty())
    }

    #[test]
    fn test_stack_frontier_adds_new_item() {
        let mut frontier = StackFrontier::new();
        let test_node = make_test_node(0);

        assert!(frontier.is_empty());
        frontier.add(test_node);
        assert!(!frontier.is_empty());
    }

    #[test]
    fn test_stack_frontier_contains_returns_false_when_empty() {
        let frontier = StackFrontier::new();
        let test_node = make_test_node(1);

        assert_eq!(false, frontier.contains(&test_node));
    }

    #[test]
    fn test_stack_frontier_contains_returns_true_if_element_exists() {
        let mut frontier = StackFrontier::new();
        let test_node = make_test_node(2);

        frontier.add(test_node.clone());

        assert!(frontier.contains(&test_node));
    }

    #[test]
    fn test_stack_frontier_remove_if_frontier_is_empty() {
        let mut frontier = StackFrontier::new();

        assert!(frontier.remove().is_none());
    }

    #[test]
    fn test_stack_frontier_remove_if_frontier_has_item() {
        let mut frontier = StackFrontier::new();
        let test_node = make_test_node(1);

        frontier.add(test_node.clone());

        assert!(frontier.remove().is_some());
    }

    // QueueFrontier

    #[test]
    fn test_queue_frontier_creates_new_empty_frontier() {
        assert!(QueueFrontier::new().is_empty())
    }

    #[test]
    fn test_queue_frontier_adds_new_item() {
        let mut frontier = QueueFrontier::new();
        let test_node = make_test_node(0);

        assert!(frontier.is_empty());
        frontier.add(test_node);
        assert!(!frontier.is_empty());
    }

    #[test]
    fn test_queue_frontier_contains_false_when_empty() {
        let frontier = QueueFrontier::new();
        let test_node = make_test_node(2);

        assert_eq!(false, frontier.contains(&test_node));
    }

    #[test]
    fn test_queue_frontier_contains_true_if_element_exists() {
        let mut frontier = QueueFrontier::new();
        let test_node = make_test_node(3);

        frontier.add(test_node.clone());
        assert!(frontier.contains(&test_node));
    }

    #[test]
    fn test_queue_frontier_remove_if_frontier_is_empty() {
        let mut frontier = QueueFrontier::new();

        assert!(frontier.remove().is_none());
    }

    #[test]
    fn test_queue_frontier_remove_if_frontier_has_item() {
        let mut frontier = QueueFrontier::new();
        let test_node = make_test_node(4);

        frontier.add(test_node.clone());

        assert!(frontier.remove().is_some());
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

    //TODO  tests for priotiryFrontier

    #[test]
    fn test_priority_queue_create_new_empty_frontier() {
        assert!(PriorityFrontier::new().is_empty());
    }

    #[test]
    fn test_priority_queue_add_new_item() {
        let mut frontier = PriorityFrontier::new();
        let test_node = make_test_node(7);

        assert!(frontier.is_empty());
        frontier.add(test_node);
        assert!(!frontier.is_empty());
    }

    #[test]
    fn test_priority_queue_contains_false_when_empty() {
        let frontier = PriorityFrontier::new();
        let test_node = make_test_node(8);

        assert_eq!(false, frontier.contains(&test_node));
    }

    #[test]
    fn test_priority_contains_true_if_element_exists() {
        let mut frontier = PriorityFrontier::new();
        let test_node = make_test_node(9);

        frontier.add(test_node.clone());
        assert!(frontier.contains(&test_node));
    }

    #[test]
    fn test_priority_remove_if_frontier_is_empty() {
        let mut frontier = PriorityFrontier::new();

        assert!(frontier.remove().is_none());
    }

    #[test]
    fn test_priority_remove_if_frontier_has_item() {
        let mut frontier = PriorityFrontier::new();
        let test_node = make_test_node(10);

        frontier.add(test_node.clone());
        assert!(frontier.remove().is_some());
    }
}
