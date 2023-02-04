
use crate::{constraints::*, nodes::*, population::*, settings::*};
use rand::{seq::SliceRandom, Rng};
use std::fs::File;
use std::io::prelude::*;

pub fn to_list<T>(node: &Node<T>, delimeter: &(T, T)) -> Vec<T>
where
    T: Default + Debug + Copy,
{
    let mut expr: Vec<T> = vec![];
    if !node.children.is_empty() {
        expr.push(delimeter.0);
    }
    expr.push(node.value);
    for sub_t in node.children.iter() {
        expr.append(&mut to_list(sub_t, delimeter));
    }

    if !node.children.is_empty() {
        expr.push(delimeter.1);
    }
    expr
}

pub fn to_xml<T>(node: &Node<T>, delimeter: &(T, T, T)) -> Vec<T>
where
    T: Default + Debug + Copy,
{
    let mut expr: Vec<T> = vec![];
    expr.push(delimeter.0);
    expr.push(node.value);

    if node.children.is_empty() {
        expr.push(delimeter.1);
    }
    expr.push(delimeter.2);

    for sub_t in node.children.iter() {
        expr.append(&mut to_xml(sub_t, delimeter));
    }

    if node.children.is_empty() {
        return expr;
    }
    expr.push(delimeter.0);
    expr.push(delimeter.1);
    expr.push(node.value);
    expr.push(delimeter.2);

    expr
}

pub fn write_to_file(text:String, path:String)
{
    let mut file = File::create(path).unwrap();
    file.write_all(text.as_bytes());
}

pub fn write_bt_to_file(text:&String, path:String)
{
    let mut out :String = "<root> <BehaviorTree ID='GP'>".to_owned();
    out = out + &text;
    out.push_str("</BehaviorTree></root>");
    write_to_file(out, path)
}

pub fn ramped_half_half<T>(size: usize, nodes: &Nodes<T>, config: &Settings) -> Vec<Individual<T>>
where
    T: Debug + Copy + Default + PartialEq,
{
    let mut chroms: Vec<Individual<T>> = vec![];
    for x in 0..size {
        let typ = x < size / 2;
        let depth = config.population.tree_depth;
        let width = config.population.tree_width;
        if let Some(tree) = gen_rnd_expr_tree(nodes, depth, width as u8, typ, &mut Counter::new()) {
            chroms.push(Individual {
                chromosome: tree,
                fitness: 0.0,
                fitness_percentage: 0.0,
                elite: false,
                //       id: x,
            });
        }
    }
    chroms
}

pub fn gen_rnd_expr_tree<T>(
    nodes: &Nodes<T>,
    depth: usize,
    width: u8,
    is_grow: bool,
    counter: &mut Counter,
) -> Option<Node<T>>
where
    T: Debug + Copy + Default + PartialEq,
{
    let id = counter.increment_id();
    let mut expr: Node<T> = Node::new(id);
    let ind: usize = nodes.leafs.len() / (nodes.leafs.len() + nodes.intermediate.len());
    let mut rng = rand::thread_rng();
    if depth == 0 || is_grow && rng.gen_range(0..=100) < ind {
        if let Some(val) = nodes.leafs.choose(&mut rng) {
            expr.value = *val;
        }
    } else {
        let inter = nodes.intermediate.choose(&mut rng).unwrap();
        expr.value = inter.value;
        let mut arity = 1;
        if inter.random_arity {
            arity = rng.gen_range(1..=width);
        }
        for _ in 0..arity {
            if let Some(node) = gen_rnd_expr_tree(nodes, depth - 1, width, is_grow, counter) {
                expr.children.push(node);
            }
        }
    }
    Some(expr)
}

pub fn gen_rnd_expr<T>(
    nodes: &Nodes<T>,
    delimeter: &(T, T),
    config: &Settings,
    depth: usize,
    is_grow: bool,
) -> Vec<T>
where
    T: Copy + PartialEq,
{
    let mut expr: Vec<T> = vec![];
    let ind: usize = nodes.leafs.len() / (nodes.leafs.len() + nodes.intermediate.len());
    let mut rng = rand::thread_rng();
    if depth == 0 || is_grow && rng.gen_range(0..=100) < ind {
        if let Some(val) = nodes.leafs.choose(&mut rng) {
            expr.push(*val);
        }
    } else {
        if rng.gen_range(0.0..=100.0) <= config.population.empty_branch_rate * 100.0 {
            return vec![];
        }

        let inter = nodes.intermediate.choose(&mut rng).unwrap();
        expr.push(delimeter.0);
        expr.push(inter.value);
        let mut arity = 1;
        if inter.random_arity {
            arity = rng.gen_range(1..2);
        }
        for _ in 0..arity {
            expr.append(&mut gen_rnd_expr(
                nodes,
                delimeter,
                config,
                depth - 1,
                is_grow,
            ))
        }
        expr.push(delimeter.1);
    }
    expr
}

use std::{fmt::Debug, time::Instant};

pub fn get_test_tree<'a>() -> Node<&'a str> {
    let nodes = get_nodes();
    let depth = 1;
    let width = 6;
    gen_rnd_expr_tree(&nodes, depth, width, false, &mut Counter::new()).unwrap()
}

pub fn get_test_tree_with<'a>(depth: usize, width: u8) -> Node<&'a str> {
    let nodes = get_nodes();
    gen_rnd_expr_tree(&nodes, depth, width, false, &mut Counter::new()).unwrap()
}

#[test]
fn ramped_hh() {
    let nodes = get_nodes();
    let size = 100;
    let config = Settings::new().unwrap();
    let ret = ramped_half_half(size, &nodes, &config);
    assert!(ret.len() == 100);
}

#[test]
fn gen_tree_node_count() {
    let start = Instant::now();
    let expr = get_test_tree();
    print!("{:?}", expr);
    let nodes = get_node_count(&expr);
    print!("{:?}", nodes);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn aids() {
    let expr = get_test_tree();
    let xml = to_xml(&expr, &get_xml_delims());
    let xml: String = xml.into_iter().collect();
    //write_to_file(xml, "../../behavior/xml/generated.xml".to_string())
    write_to_file(xml, "../behavior/xml/generated.xml".to_string())
}

#[test]
fn gen_tree_to_xml() {
    let start = Instant::now();
    let expr = get_test_tree();
    println!("{:?}", expr);
    let xml = to_xml(&expr, &get_xml_delims());
    let xml: String = xml.into_iter().collect();
    println!("{:?}", xml);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[test]
fn gen_full_tree() {
    let start = Instant::now();
    let expr = get_test_tree();
    print!("{:?}", expr);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

pub fn get_xml_delims<'a>() -> (&'a str, &'a str, &'a str) {
    ("<", "/", ">")
}

use std::fmt::Debug;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use crate::{
    constraints::get_nodes,
    init::{get_test_tree, get_test_tree_with},
    nodes::{
        get_node_by_id, get_node_count, set_node_by_id, set_single_node_by_id,
        set_subtree_by_node_id, Node, Nodes, set_node_by_id_force,
    },
    settings::Settings,
};

#[derive(Default)]
pub struct Generation<T> {
    pub size: usize,
    pub individuals: Vec<Individual<T>>,
}

pub fn get_shortest_tree_len<T>(first: &Node<T>, second: &Node<T>) -> usize
where
    T: Debug + Clone + PartialEq + Default,
{
    let node_count_first = get_node_count(first);
    let node_count_second = get_node_count(second);
    if node_count_first > node_count_second {
        node_count_second
    } else {
        node_count_first
    }
}

pub fn get_nodes_from_trees<T>(
    first: &Node<T>,
    second: &Node<T>,
    id: usize,
) -> (Option<Node<T>>, Option<Node<T>>)
where
    T: Debug + Clone + PartialEq + Default,
{
    let first_boxed: Box<Node<T>> = Box::new(first.clone());
    let second_boxed: Box<Node<T>> = Box::new(second.clone());

    let res1 = get_node_by_id(&first_boxed, id);
    let res2 = get_node_by_id(&second_boxed, id);
    (res1, res2)
}

pub fn node_crossover<T>(
    mut first: Node<T>,
    mut second: Node<T>,
    constraints: &Nodes<T>,
) -> (Node<T>, Node<T>)
where
    T: Debug + Clone + PartialEq + Default,
{
    let end = get_shortest_tree_len(&first, &second);
    let id = thread_rng().gen_range(0..end);
    let (res_node1, res_node2) = get_nodes_from_trees(&first, &second, id);
    if let (Some(n1), Some(n2)) = (res_node1, res_node2) {
        set_node_by_id(&mut second, &n1, id, constraints);
        set_node_by_id(&mut first, &n2, id, constraints);
    }
    (first, second)
}

pub fn subtree_crossover<T>(mut first: Node<T>, mut second: Node<T>) -> (Node<T>, Node<T>)
where
    T: Debug + Clone + PartialEq + Default,
{
    let end = get_shortest_tree_len(&first, &second);
    let id = thread_rng().gen_range(0..end);

    if let (Some(res_node1), Some(res_node2)) = get_nodes_from_trees(&first, &second, id) {
        let cut_subtree = set_subtree_by_node_id(&mut first, &res_node2, id);
        set_subtree_by_node_id(&mut second, &res_node1, id);
    }

    (first, second)
}

pub fn tree_crossover<T>(
    first: Individual<T>,
    second: Individual<T>,
    offspring: usize,
) -> Vec<Individual<T>>
where
    T: Default + Clone + PartialEq + Debug,
{
    let mut ret: Vec<Individual<T>> = vec![];
    //for _ in (0..offspring).step_by(2) {
    let (first, second) = subtree_crossover(first.chromosome, second.chromosome);
    ret.push(Individual::new(first));
    ret.push(Individual::new(second));
    //}
    ret
}

pub fn roulette_wheel<T>(individuals: &Vec<Individual<T>>) -> IndividualTuple<T>
where
    T: Copy,
{
    let i1 = individuals
        .choose_weighted(&mut thread_rng(), |item| {
            item.fitness_percentage
        })
        .unwrap()
        .clone();
    let i2 = individuals
        .choose_weighted(&mut thread_rng(), |item| item.fitness_percentage)
        .unwrap()
        .clone();
    IndividualTuple {
        first: i1,
        second: i2,
    }
}

impl<T> Generation<T>
where
    T: Copy + Clone + Default + Debug + PartialEq ,
{
    pub fn new(size: usize) -> Generation<T> {
        Generation {
            size,
            ..Default::default()
        }
    }

    pub fn populate(
        &mut self,
        nodes: &Nodes<T>,
        settings: &Settings,
        func: fn(usize, &Nodes<T>, &Settings) -> Vec<Individual<T>>,
    ) {
        let gen = func(self.size, nodes, &settings);
        self.individuals = gen;
    }

    pub fn crop(&mut self, selection_function: fn(f64, &Individual<T>) -> bool) {
        let avg = self.individuals.iter().fold(0.0, |acc, x| acc + x.fitness) / (self.size as f64);
        self.individuals = self
            .individuals
            .clone()
            .into_iter()
            .filter(|x| selection_function(avg, x))
            .collect();
    }

    pub fn sort_by_fitness(&mut self) {
        self.individuals
            .sort_by(|a, b| b.fitness.total_cmp(&a.fitness));
    }

    pub fn select_elites(&mut self, percentage: f64) -> Vec<Individual<T>> {
        let inds = self.individuals.clone();
        inds.into_iter()
            .take((self.size as f64 * percentage) as usize)
            .map(|mut x| {x.elite = true; x.chromosome = self.deep_cp(&x); x})
            .collect()
    }

    pub fn deep_cp(&mut self, unit: &Individual<T>) -> Node<T>{
        let mut node = Node::new(unit.chromosome.id);
        node.value = unit.chromosome.value;
        for i in 0..unit.chromosome.children.len(){
            let ind = unit.chromosome.children[i].clone();
            let mut child  = Node::new(ind.id);
            child.value = ind.value;
            node.children.push(child);
        }
        node
    }

    pub fn mutate(&mut self, setting: &Settings, nodes: &Nodes<T>){
        let mut val: i32;
        let mut s_val: i32;
        let m_rate : i32  = (setting.population.mutation_rate * 100.0) as i32;
        for i in 0..self.individuals.len(){
            val = thread_rng().gen_range(0..100) ;
            s_val = thread_rng().gen_range(0..100) ;

            if val < m_rate && !self.individuals[i].elite{
                let end = get_node_count(&self.individuals[i].chromosome);
                let id = thread_rng().gen_range(0..end);
                if s_val > 50{
                    self.individuals[i].chromosome = self.individuals[i].mutate(id);
                }else {
                    self.individuals[i].chromosome.children.shuffle(&mut thread_rng());



               // } else if s_val >= 25{
               //     let children = self.individuals[i].chromosome.children.clone();
               //     self.individuals[i].chromosome.children = children.into_iter().filter(|x| x.id != id).collect();
               // } else if s_val < 25 && self.individuals[i].chromosome.children.len() < setting.population.tree_width {
               //     let mut rng = rand::thread_rng();
               //     if let Some(add_val) = nodes.leafs.choose(&mut rng) {
               //         let mut new_node: Node<T> = Node::new(end + 1);
               //         new_node.value = *add_val;
               //         self.individuals[i].chromosome.children.push(new_node);
               //     }

               }
            }
        }
    }

    pub fn set_fitness_percentages(&mut self) {
        let max: f64 = self.individuals.iter().fold(0.0, |acc, f| acc + f.fitness)
            / self.individuals.len() as f64;
        let inds = self.individuals.clone();
        self.individuals = inds
            .into_iter()
            .map(|mut f| {
                f.fitness_percentage = (f.fitness / max);
                f
            })
            .collect();
    }

    pub fn crossover(
        &mut self,
        offspring: usize,
        combine: fn(Individual<T>, Individual<T>, usize) -> Vec<Individual<T>>,
        selection: fn(&Vec<Individual<T>>) -> IndividualTuple<T>,
        inds: Vec<Individual<T>>
    ) -> Vec<Individual<T>> {
        let end = self.size / offspring;
        let mut new_inds: Vec<Individual<T>> =  vec![];
        for _ in 0..end {
            let parents = selection(&inds);
            new_inds.append(&mut combine(parents.first, parents.second, offspring));
        }
        new_inds
    }

    pub fn handle_generation_update(
        &mut self,
        offspring: usize,
        combine: fn(Individual<T>, Individual<T>, usize) -> Vec<Individual<T>>,
        selection: fn(&Vec<Individual<T>>) -> IndividualTuple<T>,
        elite_percentage: f64,
    ) {
        self.sort_by_fitness();

        let split_at = (self.size - (self.size as f64 * elite_percentage) as usize);

        let mut elites = self.select_elites(elite_percentage);
        let mut inds = self.individuals.clone();
        inds.split_off(inds.len()/2);
        let mut sec = inds.clone();
        inds.append(&mut sec);
        let mut next_gen = self.crossover(offspring, combine, selection, inds);
        next_gen.split_off(split_at);
        next_gen.append(&mut elites);


        // maybe a new generation should be returned instead
        self.individuals = next_gen;
    }
}

pub struct IndividualTuple<T> {
    pub first: Individual<T>,
    pub second: Individual<T>,
}

#[derive(Clone, Default, Debug)]
pub struct Individual<T> {
    pub chromosome: Node<T>,
    pub fitness: f64,
    pub fitness_percentage: f64,
    pub elite : bool
}

impl<T> Individual<T>
where
    T: Default + Clone  ,
{
    pub fn new(chromosome: Node<T>) -> Self {
        Self {
            chromosome,
            ..Default::default()
        }
    }

    pub fn mutate(&mut self, id: usize) -> Node<T> {
        let constraints = get_nodes();
        let boxed: Box<Node<T>> = Box::new(self.chromosome.clone());
        let res1 = get_node_by_id(&boxed, id);
        set_node_by_id_force(&mut self.chromosome, &res1.unwrap(), id);
        self.chromosome.clone()
    }
}

#[test]
fn gen_tree_tree_crossover() {
    let expr1 = get_test_tree_with(2, 3);
    let expr2 = get_test_tree_with(2, 3);
    let (expr1, expr2) = subtree_crossover(expr1, expr2);

}

#[test]
fn gen_tree_node_crossover() {
    let constraints = get_nodes();
    let expr1 = get_test_tree_with(1, 2);
    let expr2 = get_test_tree_with(1, 2);
    let (expr1, expr2) = node_crossover(expr1, expr2, &constraints);

}


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
    pub fn default() -> Self {
        Self::new()
    }

    pub fn new() -> Counter {
        Counter {
            id: Default::default(),
        }
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


    pub fn delete_by_id(&mut self, id: usize ){
        let  children = self.children.clone();
        self.children = children.into_iter().filter(|x| x.id != id).collect();
        let  children = self.children.clone();
        self.children = children.into_iter().map(|mut x| {x.delete_by_id(id); x}).collect();
    }

    pub fn bfs(&self) {
        let mut q: VecDeque<&Node<T>> = VecDeque::new();
        q.push_front(self);
        bfs_rec(&mut q);
    }

    pub fn set_subtree(&mut self, node: &Node<T>) {
        self.value = node.value.clone();
        self.children = node.children.clone();
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

pub fn set_single_node_by_id<T>(
    root: &Node<T>,
    node: &Node<T>,
    id: usize,
    constraints: &Nodes<T>,
) -> Node<T>
where
    T: Debug + Default + PartialEq + Clone,
{
    let root: &Node<T> = &root.clone();
    if let Some(mut val) = get_node_by_id(root, id) {
        val.set_node(node, constraints);
    }
    root.clone()
}

pub fn set_subtree_by_node_id<T>(
    root: &mut Node<T>,
    node_to_set: &Node<T>,
    id: usize,
) -> Option<Node<T>>
where
    T: Debug + Clone + Default + PartialEq,
{
    if root.id == id {
        let ret = root.clone();
        root.set_subtree(node_to_set);
        return Some(ret);
    }

    let mut que: VecDeque<&mut Node<T>> = VecDeque::new();
    que.push_front(root);

    while let Some(node) = que.pop_back() {
        for child in node.children.iter_mut() {
            if child.id == id {
                let ret = child.clone();
                child.set_subtree(node_to_set);
                return Some(ret);
            }
            que.push_front(child);
        }
    }
    None
}

pub fn set_node_by_id_force<T>(
    root: &mut Node<T>,
    node_to_set: &Node<T>,
    id: usize,
) where
    T:  Clone + Default  ,
{
    if root.id == id {
        return
    }

    let mut que: VecDeque<&mut Node<T>> = VecDeque::new();
    que.push_front(root);

    while let Some(node) = que.pop_back() {
        for child in node.children.iter_mut() {
            if child.id == id {
                child.value = node_to_set.value.clone();
                return;
            }
            que.push_front(child);
        }
    }
}


pub fn set_node_by_id<T>(
    root: &mut Node<T>,
    node_to_set: &Node<T>,
    id: usize,
    constraints: &Nodes<T>,
) where
    T:  Debug + Clone + Default + PartialEq,
{
    if root.id == id {
        root.set_node(node_to_set, constraints);
    }

    let mut que: VecDeque<&mut Node<T>> = VecDeque::new();
    que.push_front(root);

    while let Some(node) = que.pop_back() {
        for child in node.children.iter_mut() {
            if child.id == id {
                child.set_node(node_to_set, constraints);
                return;
            }
            que.push_front(child);
        }
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

pub fn get_node_by_id<T>(root: &Node<T>, search_id: usize) -> Option<Node<T>>
where
    T: Clone,
{
    if root.id == search_id {
        return Some(root.clone());
    }

    let mut que: VecDeque<Node<T>> = VecDeque::new();
    que.push_front(root.clone());

    while let Some(node) = que.pop_back() {
        for child in node.children.iter().clone() {
            if child.id == search_id {
                return Some(child.clone());
            }
            que.push_front(child.clone());
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


