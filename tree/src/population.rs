#![allow(dead_code, unused)]

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use crate::{
    constraints::get_nodes,
    nodes::{get_node_by_id, get_node_count, Node, Nodes},
    settings::Settings,
};

#[derive(Default)]
pub struct Generation<T> {
    pub size: usize,
    pub individuals: Vec<Individual<T>>,
}
pub fn node_crossover<T>(
    first: Individual<Node<T>>,
    second: Individual<Node<T>>,
) -> IndividualTuple<T> {
    let node_count_first = get_node_count(&first.chromosome);
    let node_count_second = get_node_count(&second.chromosome);
    let end = if node_count_first > node_count_second {
        node_count_first
    } else {
        node_count_second
    };
    let nr = thread_rng().gen_range(0..end);
    let node = get_node_by_id(&first.chromosome, nr);

    //1. set node for each individual
    //2. return tuple
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
        .unwrap();
    let i2 = individuals
        .choose_weighted(&mut thread_rng(), |item| item.fitness_percentage)
        .unwrap();
    Parents {
        first: *i1,
        second: *i2,
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

#[derive(Clone, Copy)]
pub struct Individual<T> {
    pub chromosome: T,
    pub fitness: f64,
    pub fitness_percentage: f64,
    pub id: usize,
}

impl<T> Individual<T> {
    pub fn mutate(&mut self, mutation: fn(&Individual<T>) -> Individual<T>) -> Self {
        mutation(self)
    }
}
