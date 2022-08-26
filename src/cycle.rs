use crate::{
    grammar::Grammar,
    population::{Generation, Individual}, interpreter::InterpreterError,
};

pub fn evolution_cycle<'a>(
    init: fn(i32, i32, i32) -> Generation<'a>,
    pop_size: i32,
    derivation: fn(Vec<u8>, &'a Grammar) -> Result<Vec<&'a str>, InterpreterError>,
    grammar: &'a Grammar<'a>,
    evaluate: fn(&Generation),
    crop: fn(f64, &Individual<'a>) -> bool,
    mutation: fn(&Individual<'a>) -> Individual<'a>,
    combine: fn(&Vec<Individual<'a>>, i32) -> Vec<Individual<'a>>,
) {
    let mut pop = Generation {
        ..Default::default()
    };
    pop.populate(init, pop_size);
    pop.derive_instances(
derivation, &grammar);
    evaluate(&pop);
    pop.select(crop);
    pop.mutate(mutation);
    pop.repopulate(
        combine);
}
