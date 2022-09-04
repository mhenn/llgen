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

pub fn get_pruefer_seq(len: usize) -> Vec<usize> {
    (0..len)
        .into_iter()
        .map(|_| rand::thread_rng().gen_range(1..len))
        .collect()
}

pub fn generate_tree(seq: &mut Vec<usize>) -> HashMap<usize, Vec<usize>> {
    let n = seq.len();
    let mut l: HashSet<usize> = (1..=n + 2).into_iter().collect();
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    while let Some(x) =  seq.pop() {
        let s2: HashSet<usize> = seq.iter().cloned().collect();
        let v = (&l - &s2);
        let v = v.into_iter().min().unwrap();
        l.remove(&v);
        map.entry(x)
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
    let mut  seq = get_pruefer_seq(300);

    let map = generate_tree(&mut seq);
      println!("{:?}", map);
    assert!(false);
}

#[test]
fn gen_tree() {
    let mut tm = HashMap::new();
    tm.insert(4, vec![1, 2, 3]);
    tm.insert(5, vec![6, 4]);
    let mut seq = vec![4, 4, 4, 5];
    let map = generate_tree(&mut seq);
    println!("{:?}", map);
    assert!(map == tm);
}
