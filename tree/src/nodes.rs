use std::{collections::VecDeque, fmt::Debug};

pub struct Counter {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct IntermediateNode<T> {
    pub value: T,
    pub random_arity: bool,
    pub arity: usize,
}

pub enum NodeType<T> {
    Symbol(T),
    Intermediate((IntermediateNode<T>, usize)),
    Leaf(T),
}

impl Counter {
    pub fn new() -> Counter {
        Counter { id: 0 }
    }

    pub fn increment_id(&mut self) -> usize {
        let ret = self.id;
        self.id += 1;
        ret
    }
}

pub struct Nodes<T> {
    pub intermediate: Vec<IntermediateNode<T>>,
    pub leafs: Vec<T>,
}

#[derive(Default, Debug, Clone)]
pub struct Node<T> {
    pub id: usize,
    pub value: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: Debug + Default + Clone + PartialEq,
{
    pub fn new(id: usize) -> Self {
        Node {
            id,
            value: T::default(),
            children: vec![],
        }
    }

    pub fn bfs(&self) {
        let mut q: VecDeque<&Node<T>> = VecDeque::new();
        q.push_front(self);
        bfs_rec(&mut q);
    }

    pub fn set_node(&mut self, node: &Node<T>, constraints: &Nodes<T>) {
        // should be more generic but ... meh
        if self.children.is_empty() && node.children.is_empty() {
            self.value = node.value.clone();
        } else if !self.children.is_empty() && !node.children.is_empty() {
            if constraints
                .intermediate
                .iter()
                .any(|x| x.value == self.value && x.arity == 1)
                && self.children.len() > 1
            {
                return;
            }
        } else {
            return;
        }
        self.value = node.value.clone();
    }
}

pub fn set_single_node_by_id<T>(root: &Node<T>, node: &Node<T>, id: usize, constraints: &Nodes<T>)
where
    T: Debug + Default + PartialEq + Clone,
{
    if let Some(mut val) = get_node_by_id(root, id) {
        val.set_node(node, constraints);
    }
}

pub fn get_node_count<T>(node: &Node<T>) -> usize {
    let mut ret = 1;
    if node.children.is_empty() {
        return ret;
    }
    for x in node.children.iter() {
        ret += get_node_count(x);
    }
    ret
}

pub fn get_node_by_id<T>(root: &Node<T>, search_id: usize) -> Option<Node<T>> {
    if root.id == search_id {
        return Some(root.clone());
    }

    let mut que: VecDeque<&Node<T>> = VecDeque::new();
    que.push_front(root);

    while let Some(node) = que.pop_back() {
        for child in node.children.iter() {
            if child.id == search_id {
                return Some(*child);
            }
            que.push_front(&child);
        }
    }
    None
}

pub fn bfs_rec<T>(q: &mut VecDeque<&Node<T>>)
where
    T: Debug + Clone + Default,
{
    let v = q.pop_back().unwrap();
    println!("{:?}", v.value);
    for x in v.children.iter() {
        q.push_front(x);
    }
    bfs_rec(q);
}

pub fn dfs_rec<T>(node: Node<T>)
where
    T: Debug + Default + Copy,
{
    println!("Id:{:?} {:?}", node.id, node.value);
    for u in node.children.iter() {
        dfs_rec(u.clone());
    }
}
