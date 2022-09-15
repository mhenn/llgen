use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};


static COUNTER: AtomicUsize = AtomicUsize::new(1);

pub struct Nodes<T> {
    pub root: T,
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
    T: Debug + Default + Clone,
{
    pub fn new() -> Self {
        Node {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            value: T::default(),
            children: vec![],
        }
    }
    pub fn bfs(&self) {
        let mut q: VecDeque<&Node<T>> = VecDeque::new();
        q.push_front(self);
        bfs_rec(&mut q);
    }
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


