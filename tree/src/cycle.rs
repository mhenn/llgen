
use rand::Rng;
use std::collections::HashMap;

pub fn evolution_cycle<'a>(
    init: fn(i32, i32, usize) -> Generation<'a>,
    pop_size: usize,
    derivation: fn(&Vec<u8>, &'a Grammar) -> Result<Vec<&'a str>, InterpreterError>,
    grammar: &'a Grammar<'a>,
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

fn standard_one_point_crossover<'a>(
    len: usize,
    i1: &Individual<'a>,
    i2: &Individual<'a>,
    grammar: &Grammar<'a>,
) -> Result<Individual<'a>, InterpreterError> {
    let mut chromosome: Vec<u8> = i1.chromosome.clone().into_iter().take(len).collect();
    let t: Vec<u8> = i2.chromosome.clone().into_iter().skip(len).collect();
    chromosome.extend(t);
    interpret(&chromosome, grammar)?;
    let ind = Individual {
        chromosome,
        fitness: 0.0,
        word: vec![],
    };
    Ok(ind)
}

pub fn combine<'a>(
    inds: &Vec<Individual<'a>>,
    pop_size: usize,
    grammar: &'a Grammar,
) -> Vec<Individual<'a>> {
    let elite = pop_size as f64 * 0.25;
    let mut new_inds: Vec<Individual> = inds.clone().into_iter().take(elite as usize).collect();
    while new_inds.len() < pop_size {
        let rand1 = rand::thread_rng().gen_range(0..inds.len());
        let rand2 = rand::thread_rng().gen_range(0..inds.len());
        let i1 = &inds[rand1];
        let i2 = &inds[rand2];
        let min = std::cmp::min(i1.chromosome.len(), i2.chromosome.len());
        //println!("{}", min);
        for _ in 0..1 {
            if let Ok(ind) = standard_one_point_crossover(min, &i1, &i2, grammar) {
                new_inds.push(ind);
            }
            if let Ok(ind) = standard_one_point_crossover(min, &i2, &i1, grammar) {
                new_inds.push(ind);
            }
            new_inds.push(Individual {
                chromosome: vec![],
                fitness: 0.0,
                word: vec![],
            });
        }
    }
    new_inds
}

pub fn evaluate<'a>(_gen: &'a Generation) {}
pub fn crop<'a>(pop_fitness: f64, ind: &Individual) -> bool {
    ind.fitness > pop_fitness
}
