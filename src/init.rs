
use rand::Rng;
//use crate::grammar::*;
use crate::population::*;


fn generate_random_codon() -> u8{
    rand::thread_rng().gen_range(0..=255)
}

fn generate_chromosome<T>(size: usize, generator: fn() -> T) -> Vec<T>
    where T: Clone
{
    (0..size).into_iter().map(|_| generator()).collect()
}

fn generate_rnd_chromosomes(min: i32, max: i32, size: usize) -> Vec<Vec<u8>>{
    let mut rng =  rand::thread_rng();
    (0..size).into_iter()
        .map(|_| (min..((rng.gen::<i32>() % max) + min)).into_iter().map(|_| rng.gen_range(0..=255)).collect())
        .collect()
}


fn generate_individuals(chromosomes: Vec<Vec<u8>>) -> Vec<Individual>{
    chromosomes.into_iter()
        .map(|chromosome| Individual{ chromosome, fitness:0.0})
        .collect()
}

pub fn rnd(min: i32, max: i32, pop_size: usize) -> Generation {
    Generation{individuals: generate_individuals(generate_rnd_chromosomes(min, max, pop_size)), count:0}
}


#[test]
fn chromosomes_test(){
    let size: usize = 15;
    let max: i32 = 10;
    let chromosomes = generate_rnd_chromosomes(1, max, size);
    assert_eq!(chromosomes.len(), size );
    let over_count = chromosomes.iter().fold(0, |acc, x| {print!("{:?} ", x.len()); if (x.len() as i32) <= max {acc} else { acc+1}});
    assert_eq!(over_count, 0);
}

#[test]
fn chromosome_test(){
    let size: usize = 15;
    let mut rng =  rand::thread_rng();
    let chromosome = generate_chromosome(size,  generate_random_codon);
    println!("{:?}", chromosome);
    assert_eq!(1, 1);
}
