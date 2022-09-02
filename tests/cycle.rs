use rand::Rng;
use ruge::{cycle::*, grammar::*, init::*, population::*};

#[test]
fn combination() {
    let size: usize = 20;
    let grammar = get_bt_grammar();
    let mut inds = generate_individuals(generate_rnd_chromosomes(2, 10, size));
    inds = inds
        .into_iter()
        .map(|x| Individual {
            chromosome: x.chromosome,
            fitness: rand::thread_rng().gen(),
            word: vec![],
        })
        .collect();
    inds = combine(&inds, size, &grammar);
    // println!("{:?}", inds);
    //assert!(false);
}
