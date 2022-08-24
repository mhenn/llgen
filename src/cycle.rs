use crate::{grammar::Grammar, population::Generation};

pub fn evolution_cycle(init: fn(i32, i32, i32) -> Generation, grammar: Grammar) {
    let mut pop = Generation {
        ..Default::default()
    };
    pop.populate(init);
}
