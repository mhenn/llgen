#![allow(dead_code, unused)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Range,
};

use rand::Rng;

pub struct Generation<T> {
    pub population: Vec<Individual<T>>,
}

pub struct Individual<T> {
    pub chromosome: T,
    pub fitness: f64,
}


//pub fn generate_grammar_tree<'a>(tree: HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {

//}

// Tree generation
pub fn get_pruefer_seq(len: usize) -> Vec<usize> {
    (0..len)
        .into_iter()
        .map(|_| rand::thread_rng().gen_range(1..len))
        .collect()
}

pub fn generate_tree(seq: &mut Vec<usize>) -> HashMap<usize, Vec<usize>> {
    let n = seq.len();
    let s = seq.clone();
    //let mut l: HashSet<usize> = (1..=n + 2).into_iter().collect();
    let mut l: Vec<usize> = (1..=n + 2).into_iter().collect();

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for x in s.iter(){
        let s1: HashSet<usize> = l.iter().cloned().collect();
        let s2: HashSet<usize> = seq.iter().cloned().collect();
        let v = (&s1 - &s2);
        let v = v.into_iter().min().unwrap();

        l.remove(l.iter().position(|&x| x == v).unwrap());
        seq.remove(0);
        map.entry(*x)
            .and_modify(|e| e.push(v))
            .or_insert_with(|| vec![v]);
    }
    let l: Vec<usize> = l.into_iter().collect();
    map.entry(l[0])
        .and_modify(|e| e.push(l[1]))
        .or_insert_with(|| vec![l[1]]);
    map
}

use std::time::{Duration, Instant};
#[test]
fn gen_tree_by_long_pruefer() {
    let len = 100;
    let mut  seq = get_pruefer_seq(len);
    let map = generate_tree(&mut seq);
    println!("{:?}", map);
    assert!(map.len() <= len);
}
#[test]
fn gen_tree() {
    let mut tm = HashMap::new();
    tm.insert(4, vec![1, 2, 3]);
    tm.insert(5, vec![ 4, 6]);
    let mut seq = vec![4, 4, 4, 5];
    let map = generate_tree(&mut seq);
    println!("{:?}", map);
    assert!(map == tm);
}
