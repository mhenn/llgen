use ruge::{grammar::*, init::*};

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
