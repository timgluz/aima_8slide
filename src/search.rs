use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

use crate::actions::Action; // TODO: how to abstract it away?? and have something in search.rs instead
pub mod frontiers;
pub mod uniform;

pub trait SearchProblem {
    fn actions(&self) -> Vec<Action>;
    fn result(&self, action: &Action) -> Box<dyn SearchProblem>;
    fn test_goal(&self) -> bool;
    // returns cost of solution from previous solution applying an Action A;
    fn path_cost(&self) -> u32;
    fn value(&self) -> u32;
    fn as_string(&self) -> String;
    fn hash_code(&self) -> u64; // used for comparition
}

impl fmt::Debug for dyn SearchProblem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SearchProblem")
            .field("state", &self.as_string())
            .finish()
    }
}

#[derive(Clone)]
pub struct SearchNode {
    parent: Option<Rc<SearchNode>>,
    action: Option<Action>,
    item: Rc<Box<dyn SearchProblem>>,
    depth: usize,
    path_cost: u32,
}

impl SearchNode {
    pub fn root(problem: Box<dyn SearchProblem>) -> Self {
        SearchNode {
            parent: None,
            action: None,
            item: Rc::new(problem),
            depth: 0,
            path_cost: 0,
        }
    }

    pub fn child_node(parent: &Rc<SearchNode>, action: Action) -> SearchNode {
        let new_problem = parent.item().result(&action);
        let new_path_cost = parent.item().path_cost() + new_problem.path_cost();

        SearchNode {
            parent: Some(parent.clone()),
            action: Some(action),
            item: Rc::new(new_problem),
            depth: parent.depth() + 1,
            path_cost: new_path_cost,
        }
    }

    pub fn expand(&self) -> Vec<SearchNode> {
        let mut next_nodes = vec![];

        let possible_actions = self.item().actions();
        for action in possible_actions.into_iter() {
            let new_parent = Rc::new(self.clone());
            let next_node = SearchNode::child_node(&new_parent, action);
            next_nodes.push(next_node);
        }

        next_nodes
    }
    pub fn item(&self) -> &Rc<Box<dyn SearchProblem>> {
        &self.item
    }
    pub fn action(&self) -> &Option<Action> {
        &self.action
    }
    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn solution(&self) -> Vec<Action> {
        self.path()
            .iter()
            .rev()
            .map(|node| node.action().unwrap_or(Action::None))
            .collect()
    }

    pub fn path(&self) -> Vec<Rc<SearchNode>> {
        let mut path = vec![Rc::new(self.clone())];
        let mut cur_parent = self.parent.clone();

        loop {
            match cur_parent {
                None => break,
                Some(parent) => {
                    path.push(parent.clone());
                    cur_parent = parent.parent.clone();
                }
            }
        }
        path
    }
}

impl Eq for SearchNode {}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
            && self.path_cost == other.path_cost
            && self.item().hash_code() == other.item().hash_code()
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &SearchNode) -> Ordering {
        other.path_cost.cmp(&self.path_cost) // notice reverse ordering - it makes by default minHeap
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
