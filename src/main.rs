mod cycle;
mod grammar;
mod init;
mod interpreter;
mod mapping;
mod population;

use std::collections::HashMap;

use crate::cycle::*;
use crate::init::*;
use crate::grammar::*;
use crate::interpreter::*;
use crate::population::*;



fn mutation<'a>(ind : &Individual<'a>) -> Individual<'a>{
    Individual { chromosome: vec![], fitness: 0.0, word: vec![] }
}

fn combine<'a>(inds: &Vec<Individual<'a>>,pop_size: i32) -> Vec<Individual<'a>>{
    vec![
    Individual { chromosome: vec![], fitness: 0.0, word: vec![] }]
}

fn evaluate<'a>(gen: &'a Generation){}

fn get_grammar<'a>()-> Grammar<'a>{
    let mut map = HashMap::new();
    map.insert("ROOT",  vec![vec!["<root>","","</root>"]]);
    map.insert("NL",    vec![vec!["LOCF", "NL"], vec!["LOCF"]]);
    map.insert("LOCF",  vec![vec!["L"], vec!["CF"]]);
    map.insert("L",     vec![vec!["ACT"]]);
    map.insert("CF",    vec![vec!["<seq>","NL","</seq>"],vec!["<fall>","NL","</fall>"],vec!["<par>","NL","</par>"],vec!["<pol>","LOCF","</pol>"]] );
    map.insert("ACT",   vec![vec!["pickup"],vec!["putdown"]]);

    Grammar{non_terminals: vec![""],
    terminals: vec![],
    rules: map,
    start: "ROOT"}
}


fn main() {




    evolution_cycle(rnd, 100,
interpret,
                    &get_grammar(), evaluate, crop, mutation, combine);
}
