#![allow(dead_code, unused)]

use crate::{constraints::get_nodes, nodes::Nodes, settings::Settings};

pub struct Generation<T> {
    pub size: usize,
    pub individuals: Vec<Individual<T>>,
}

pub fn roulette_wheel<T>(individuals: Vec<Individual<T>>) {}

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

    pub fn crossover(&mut self, combine: fn(Individual<T>, Individual<T>) -> Vec<Individual<T>>) {
        self.individuals = combine();
    }
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
