use crate::grammar::Grammar;

#[derive(Default, Clone)]
pub struct Individual<'a> {
    pub chromosome: Vec<u8>,
    pub fitness: f64,
    pub word: Vec<&'a str>,
}

impl<'a> Individual<'a> {
    pub fn derive_word(
        &mut self,
        func: fn(Vec<u8>, &'a Grammar) -> Vec<&'a str>,
        grammar: &'a Grammar,
    ) {
        self.word = func(self.chromosome.clone(), grammar);
    }
}

#[derive(Default)]
pub struct Generation<'a> {
    pub individuals: Vec<Individual<'a>>,
    pub count: i32,
}

impl<'a> Generation<'a> {
    pub fn populate(&mut self, func: fn(i32, i32, i32) -> Generation<'a>) {
        let gen = func(1, 10, 100);
        self.individuals = gen.individuals;
    }

    pub fn derive_instances(
        &mut self,
        func: fn(Vec<u8>, &'a Grammar) -> Vec<&'a str>,
        grammar: &'a Grammar,
    ) {
        for x in self.individuals.iter_mut() {
            x.derive_word(func, grammar);
        }
    }

    pub fn select(&mut self, selection_function: fn(f64, &Individual<'a>) -> bool) {
        let sum = self.individuals.iter().fold(0.0, |acc, x| acc + x.fitness);
        let inds = self.individuals.clone();
        self.individuals = inds
            .into_iter()
            .filter(|x| selection_function(sum, x))
            .collect();
    }
}
