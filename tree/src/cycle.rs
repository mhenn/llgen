


use crate::{
    constraints::get_nodes,
    init::ramped_half_half,
    nodes::Nodes,
    population::{Generation, Individual},
    settings::Settings,
};

pub fn evolution_cycle<T>(
    init: fn(usize, &Nodes<T>, &Settings) -> Vec<Individual<T>>,
    nodes: &Nodes<T>,
    pop_size: usize,
    evaluate: fn(&Generation<T>),
    //    crop: fn(f64, &Individual<T>) -> bool,
    //    mutation: fn(&Individual<T>) -> Individual<T>,
    //    combine: fn(&Vec<Individual<T>>, usize) -> Vec<Individual<T>>,
) where
    T: Copy + Clone + Default,
{
    //Todo: settings & get_nodes
    let settings = Settings::new().unwrap();
    let mut pop = Generation::new(pop_size);
    pop.populate(nodes, &settings, init);
    evaluate(&pop);
    //    pop.crossover(&settings, combine, selection)
    //    pop.mutate(mutation);
}

pub fn evaluate<T>(_gen: &Generation<T>) {}
pub fn crop<T>(pop_fitness: f64, ind: &Individual<T>) -> bool {
    ind.fitness > pop_fitness
}

fn ramped_hh() {
    let nodes = get_nodes();
    let size = 10;
    let config = Settings::new().unwrap();
    let _ret = ramped_half_half(size, &nodes, &config);
}

#[test]
fn evolve() {}
