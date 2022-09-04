#![allow(dead_code, unused)]

use std::collections::VecDeque;
use rand::{distributions::Uniform, Rng};
use crate::grammar::*;

#[derive(Clone)]
pub struct GNode<T> {
    pub value: Vec<T>,
    //    pub  ids: Vec<u8>,
    pub edges: Vec<Edge<T>>,
}

#[derive(Clone)]
pub struct Edge<T> {
    pub value: T,
    pub end: GNode<T>,
}

impl<T> GNode<T> {
    fn add_edge_to(&mut self, end: GNode<T>, value: T) {
        self.edges.push(Edge { value, end });
    }
}

fn only_terminals(grammar: &Grammar, ls: &Vec<&str>) -> bool {
    let contains = ls
        .iter()
        .any(|x| grammar.non_terms.get(x).is_some());//matches!(grammar.check_membership(x), Ok(NodeType::NonTerminal)));
    !contains
}

pub fn grow_full<'a>(
    amount: usize,
    min: usize,
    max: usize,
    grammar: &'a Grammar,
) -> Vec<(Vec<usize>, Vec<&'a str>)> {
    let starting_symbol = <&str>::clone(&grammar.start);
    let mut words: Vec<(Vec<usize>, Vec<&str>)> = vec![];
    for _ in 0..amount {
        let min_val = rand::thread_rng().gen_range(min..=max);
        if let Some(val) = tree_growth(grammar, (vec![], vec![starting_symbol]), grow_cond, min_val) {
            words.push(tree_growth(grammar, val, full_cond, min).unwrap())
        }
    }
    words
}

pub fn grow_cond<'a>(word: &Vec<&str>, position: Option<usize>, min: usize) -> bool {
    word.len() > min
}

pub fn full_cond(word: &Vec<&str>, position: Option<usize>, min: usize) -> bool {
    position.is_none()
}

pub fn tree_growth<'a>(
    grammar: &'a Grammar,
    tup: (Vec<usize>, Vec<&'a str>),
    condition: fn(&Vec<&str>, Option<usize>, usize) -> bool,
    min: usize,
) -> Option<(Vec<usize>, Vec<&'a str>)> {
    let mut queue: VecDeque<(Vec<usize>, Vec<&str>)> = VecDeque::new();
    queue.push_front(tup);

    while let Some((decisions, current_word)) = queue.pop_back() {
        let pos = grammar.get_next_nt_in(&current_word);
        if condition(&current_word, pos, min) {
            return Some((decisions, current_word));
        }
        if pos.is_none(){
            continue;
        }
        let pos = pos.unwrap();
        for (i, rule) in grammar
            .rules
            .get(current_word[pos])
            .unwrap()
            .iter()
            .enumerate()
        {
            let mut new_word = current_word.clone();
            let mut new_decisions = decisions.clone();
            new_word.splice(pos..(pos + 1), rule[..].to_vec());
            new_decisions.push(i);
            queue.push_front((new_decisions, new_word));
        }
    }
    None
}

//#[test]
//fn generate_words() {
//
//    let word = vec!["<root>", "NL", "</root>"];
//    let grammar = get_bt_grammar();
//    let res = grow_full(1, 20, 30, &grammar);
//
//    println!("{:?}", res);
//    assert!(false);
//
//}

#[test]
fn word_with_just_terminals() {
    let word = vec!["<root>", "pickup", "</root>"];
    let grammar = get_bt_grammar();
    assert!(only_terminals(&grammar, &word));
}

#[test]
fn words_with_non_terminals() {
    let grammar = get_bt_grammar();
    let word = vec!["<root>", "NL", "</root>"];

    assert!(!only_terminals(&grammar, &word));
}

#[test]
fn full_from_start() {
    let word = vec!["<root>", "NL", "</root>"];
    let grammar = get_bt_grammar();
    let res = tree_growth(&grammar, (vec![], word), full_cond, 1);
   assert!(res.is_some());
}
