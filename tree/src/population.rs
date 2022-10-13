#![allow(dead_code, unused)]

use std::fmt::Debug;
use std::sync::Arc;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use crate::{
    constraints::get_nodes,
    init::{get_test_tree, get_test_tree_with},
    nodes::{get_node_by_id, get_node_count, set_single_node_by_id, Node, Nodes},
    settings::Settings,
};

#[derive(Default)]
pub struct Generation<T> {
    pub size: usize,
    pub individuals: Vec<Individual<T>>,
}

pub fn node_crossover<T>(
    first: Node<T>,
    second: Node<T>,
    constraints: &Nodes<T>,
) -> (Node<T>, Node<T>, usize)
where
    T: Debug + Clone + PartialEq + Default,
{
    let node_count_first = get_node_count(&first);
    let node_count_second = get_node_count(&second);
    let end = if node_count_first > node_count_second {
        node_count_second
    } else {
        node_count_first
    };

    let nr = thread_rng().gen_range(0..end);
    let boxed_node: Box<Node<T>> = Box::new(first.clone());
    if let Some(node) = get_node_by_id(&boxed_node, nr) {
        println!("{:?}", node);
        set_single_node_by_id(&second, &node, nr, constraints);
    }
    //   let boxed_node: Box<Node<T>> = Box::new(second.clone());
    //   if let Some(node) = get_node_by_id(boxed_node, nr) {
    //       set_single_node_by_id(&second, &node, nr, constraints);
    //   }

    //1. set node for each individual

    //2. return tuple

    (first, second, nr)
}
//
//pub fn subtree_crossover<T>(first: Individual<T>,second: Individual<T>) -> Individual<T>{
//
//}

pub fn tree_crossover<T>(
    first: Individual<T>,
    second: Individual<T>,
    offspring: usize,
) -> Vec<Individual<T>> {
    let mut ret: Vec<Individual<T>> = vec![];
    for _ in 0..offspring {
        //        ret.push(node_crossover(first, second));
    }
    // ret.push(subtree_crossover(first, second));
    ret
}

pub fn roulette_wheel<T>(individuals: Vec<Individual<T>>) -> IndividualTuple<T>
where
    T: Copy,
{
    let i1 = individuals
        .choose_weighted(&mut thread_rng(), |item| item.fitness_percentage)
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
    T: Copy + Clone + Default,
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

    pub fn crossover(
        &mut self,
        offspring: usize,
        combine: fn(Individual<T>, Individual<T>, usize) -> Vec<Individual<T>>,
        selection: fn(&Vec<Individual<T>>) -> IndividualTuple<T>,
    ) {
        let end = self.size / offspring;
        for _ in 0..end {
            let parents = selection(&self.individuals);
            self.individuals = combine(parents.first, parents.second, offspring);
        }
    }
}

pub struct IndividualTuple<T> {
    pub first: Individual<T>,
    pub second: Individual<T>,
}

#[derive(Clone, Default)]
pub struct Individual<T> {
    pub chromosome: Node<T>,
    pub fitness: f64,
    pub fitness_percentage: f64,
}

impl<T> Individual<T>
where
    T: Default,
{
    pub fn new(chromosome: Node<T>) -> Self {
        Self {
            chromosome,
            ..Default::default()
        }
    }

    pub fn mutate(&mut self, mutation: fn(&Individual<T>) -> Individual<T>) -> Self {
        mutation(self)
    }
}

#[test]
fn gen_tree_node_count() {
    let constraints = get_nodes();
    let expr1 = get_test_tree_with(1, 2);
    let expr2 = get_test_tree_with(1, 2);
    print!("{:?}", expr1);
    println!();
    println!("{:?}", expr2);
    let (expr1, expr2, nr) = node_crossover(expr1, expr2, &constraints);

    println!("{:?}", nr);
    println!("{:?}", expr1);
    println!();
    println!("{:?}", expr2);
    assert!(false)
}
