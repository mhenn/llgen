use rand::Rng;
use std::collections::HashMap;





pub fn evolution_cycle<'a>(
    init: fn(usize, &Nodes<T>, &Settings) -> Vec<Individual<T>>,
    pop_size: usize,
    evaluate: fn(&Generation),
    crop: fn(f64, &Individual<'a>) -> bool,
    mutation: fn(&Individual<'a>) -> Individual<'a>,
    combine: fn(&Vec<Individual<'a>>, usize, &Grammar) -> Vec<Individual<'a>>,
) {
    let mut pop = Generation {
        ..Default::default()
    };

    pop.populate(init, pop_size);
    pop.derive_instances(derivation, &grammar);
    evaluate(&pop);
    pop.select(crop);
    pop.mutate(mutation);
    pop.repopulate(combine, grammar);
}

pub fn mutation<'a>(ind: &Individual<'a>) -> Individual<'a> {
    ind.clone()
}

pub fn evaluate<'a>(_gen: &'a Generation) {}
pub fn crop<'a>(pop_fitness: f64, ind: &Individual) -> bool {
    ind.fitness > pop_fitness
}
