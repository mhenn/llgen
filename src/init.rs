use crate::population::*;
use rand::{distributions::Uniform, Rng};

pub fn generate_random_codon() -> u8 {
    rand::thread_rng().gen_range(0..=255)
}

pub fn generate_chromosome<T>(size: i32, generator: fn() -> T) -> Vec<T> {
    (0..size).into_iter().map(|_| generator()).collect()
}

pub fn generate_rnd_chromosomes(min: i32, max: i32, size: usize) -> Vec<Vec<u8>> {
    let dist = Uniform::new_inclusive(min, max);

    (0..size)
        .into_iter()
        .map(|_| generate_chromosome(rand::thread_rng().sample(dist), generate_random_codon))
        .collect()
}

pub fn generate_individuals<'a>(chromosomes: Vec<Vec<u8>>) -> Vec<Individual<'a>> {
    chromosomes
        .into_iter()
        .map(|chromosome| Individual {
            chromosome,
            fitness: 0.0,
            word: vec![],
        })
        .collect()
}

//min: chromosome size min
//max: same just max
pub fn rnd<'a>(min: i32, max: i32, pop_size: usize) -> Generation<'a> {
    Generation {
        individuals: generate_individuals(generate_rnd_chromosomes(min, max, pop_size)),
        pop_size,
        count: 0,
    }
}
/////////////////////////////
///

pub fn ptc2(tree_size: usize, tree_depth: usize, pop_size: usize) {}

#[test]
fn chromosomes_rnd() {
    let size: usize = 15;
    let max: i32 = 10;
    let chromosomes = generate_rnd_chromosomes(1, max, size);
    assert_eq!(chromosomes.len(), size);
    let over_count = chromosomes.iter().fold(0, |acc, x| {
        if (x.len() as i32) <= max {
            acc
        } else {
            acc + 1
        }
    });
    assert_eq!(over_count, 0);
}

#[test]
fn chromosome_rnd() {
    let size: i32 = 15;
    let chromosome = generate_chromosome(size, generate_random_codon);
    assert_eq!(chromosome.len() as i32, size);
}
