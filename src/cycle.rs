use crate::{
    grammar::Grammar,
    population::{Generation, Individual},
};

pub fn evolution_cycle<'a>(
    init: fn(i32, i32, i32) -> Generation<'a>,
    derivation: fn(Vec<u8>, &Grammar) -> Vec<&'a str>,
    grammar: &'a Grammar<'a>,
    evaluate: fn(&Generation),
    crop: fn(f64, &Individual<'a>) -> bool,
) {
    let mut pop = Generation {
        ..Default::default()
    };
    pop.populate(init);
    pop.derive_instances(derivation, &grammar);
    evaluate(&pop);
    pop.select(crop);
    //pop.mutate();
    //pop.repopulate();
}
