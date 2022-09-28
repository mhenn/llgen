#![allow(dead_code, unused)]

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

use crate::{constraints::get_nodes, nodes::Nodes, settings::Settings};

pub struct Generation<T> {
    pub size: usize,
    pub individuals: Vec<Individual<T>>,
}

//pub fn node_crossover<T>(first: Individual<T>,second: Individual<T>)-> Individual<T>{
//
//}
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

pub fn roulette_wheel<T>(individuals: Vec<Individual<T>>) -> Parents<T>
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
    T: Copy + Clone,
{
    pub fn populate(
        &mut self,
        nodes: &Nodes<T>,
        func: fn(usize, &Nodes<T>, &Settings) -> Vec<Individual<T>>,
        size: usize,
    ) {
        let mut settings = Settings::new().unwrap();
        let gen = func(size, nodes, &settings);
        self.size = size;
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
        selection: fn(&Vec<Individual<T>>) -> Parents<T>,
    ) {
        let end = self.size / offspring;
        for _ in 0..end {
            let parents = selection(&self.individuals);
            self.individuals = combine(parents.first, parents.second, offspring);
        }
    }
}

pub struct Parents<T> {
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
