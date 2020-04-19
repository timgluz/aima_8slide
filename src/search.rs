use std::fmt;
use std::rc::{Rc, Weak};
use std::borrow::Borrow;

use crate::actions::Action; // TODO: how to abstract it away?? and have something in search.rs instead
pub mod frontiers;
pub mod uniform;

pub trait SearchProblem {
    fn actions(&self) -> Vec<Action>;
    fn result(&self, action: &Action) -> Box<dyn SearchProblem>;
    fn test_goal(&self) -> bool;
    fn path_cost(&self) -> u32;
    fn value(&self) -> u32;
    fn as_string(&self) -> String;
    fn hash_code(&self) -> u64; // used for comparition
}

impl fmt::Debug for SearchProblem {
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

        SearchNode {
            parent: Some(parent.clone()),
            action: Some(action),
            item: Rc::new(new_problem),
            depth: parent.depth() + 1,
            path_cost: 1u32,
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
        self.path().iter()
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

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        let self_item_hash = self.item().hash_code();
        let other_item_hash = other.item().hash_code();

        self_item_hash == other_item_hash
    }
}
