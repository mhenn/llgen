#![allow(dead_code, unused)]
use crate::{grammar::Grammar, interpreter::InterpreterError};

#[derive(Default, Debug, Clone)]
pub struct Individual<'a> {
    pub chromosome: Vec<u8>,
    pub fitness: f64,
    pub word: Vec<&'a str>,
}

impl<'a> Individual<'a> {
    pub fn derive_word(
        &mut self,
        func: fn(&Vec<u8>, &'a Grammar<'a>) -> Result<Vec<&'a str>, InterpreterError>,
        grammar: &'a Grammar,
    ) {
        if let Ok(word) = func(&self.chromosome, grammar) {
            self.word = word;
        }
    }
}

#[derive(Default)]
pub struct Generation<'a> {
    pub individuals: Vec<Individual<'a>>,
    pub count: i32,
    pub pop_size: usize,
}

impl<'a> Generation<'a> {
    pub fn populate(&mut self, func: fn(i32, i32, usize) -> Generation<'a>, size: usize) {
        let gen = func(1, 10, size);
        self.pop_size = size;
        self.individuals = gen.individuals;
    }

    pub fn derive_instances(
        &mut self,
        func: fn(&Vec<u8>, &'a Grammar<'a>) -> Result<Vec<&'a str>, InterpreterError>,
        grammar: &'a Grammar,
    ) {
        for x in self.individuals.iter_mut() {
            x.derive_word(func, grammar);
        }
    }

    pub fn select(&mut self, selection_function: fn(f64, &Individual<'a>) -> bool) {
        let avg =
            self.individuals.iter().fold(0.0, |acc, x| acc + x.fitness) / (self.pop_size as f64);
        let inds = self.individuals.clone();
        self.individuals = inds
            .into_iter()
            .filter(|x| selection_function(avg, x))
            .collect();
    }

    pub fn mutate(&mut self, mutation: fn(&Individual<'a>) -> Individual<'a>) {
        let inds = self.individuals.clone();
        self.individuals = inds.into_iter().map(|x| mutation(&x)).collect();
    }

    pub fn repopulate(
        &mut self,
        combine: fn(&Vec<Individual<'a>>, usize, &Grammar) -> Vec<Individual<'a>>,
        grammar: &Grammar,
    ) {
        self.individuals
            .sort_by(|a, b| b.fitness.total_cmp(&a.fitness));
        self.individuals = combine(&self.individuals, self.pop_size, grammar);
    }
}
