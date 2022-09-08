#![allow(dead_code, unused)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::Range,
};

use rand::{seq::SliceRandom, Rng};

pub struct Generation<T> {
    pub population: Vec<Individual<T>>,
}

pub struct Individual<T> {
    pub chromosome: T,
    pub fitness: f64,
    pub id: usize,
}

pub struct IntermediateNode<'a> {
    pub value: &'a str,
    pub random_arity: bool,
    pub arity: usize,
}



pub fn gen_rnd_expr<'a>(
    intermediate: &'a Vec<IntermediateNode>,
    leafs: &'a Vec<&str>,
    depth: usize,
    is_grow: bool,
) -> Vec<&'a str> {
    let mut expr: Vec<&str> = vec![];
    let ind: usize = leafs.len() / (leafs.len() + intermediate.len());
    let mut rng = rand::thread_rng();
    if depth == 0 || is_grow && rng.gen_range(0..=100) < ind {
        if let Some(val) = leafs.choose(&mut rng) {
            expr.push(val);
        }
    } else {
        let inter = intermediate.choose(&mut rng).unwrap();
        expr.push("(");
        expr.push(inter.value);
        let mut arity = 1;
        if inter.random_arity {
            arity = rng.gen_range(1..10);
        }
        for _ in (0..arity) {
            expr.append(&mut gen_rnd_expr(intermediate, leafs, depth - 1, is_grow))
        }
        expr.push(")");
    }
    expr
}

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
    for x in s.iter() {
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

pub fn get_leafs<'a>() -> Vec<&'a str>{
    vec!["pickup","place","move", "ready"]
}

pub fn get_intermediate<'a>() -> Vec<IntermediateNode<'a>>{

    vec![IntermediateNode{value:"seq", arity: 2, random_arity: true},
        IntermediateNode{value:"fall", arity:2 , random_arity: true},
        IntermediateNode{value:"par" , arity:2 , random_arity: true},
        IntermediateNode{value:"pol" , arity:1 , random_arity: false}]
}


#[test]
fn gen_full(){
    let depth = 3;
    let inter = get_intermediate();
    let leafs = get_leafs();
     let start = Instant::now();
    let expr = gen_rnd_expr(&inter, &leafs, depth, false);
    println!("{:?}", expr);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[test]
fn get_grow(){
    let depth = 3;
    let inter = get_intermediate();
    let leafs = get_leafs();
     let start = Instant::now();
    let expr = gen_rnd_expr(&inter, &leafs, depth, true);
    println!("{:?}", expr);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[test]
fn gen_tree_by_long_pruefer() {
    let len = 100;
    let mut seq = get_pruefer_seq(len);
    let map = generate_tree(&mut seq);
    println!("{:?}", map);
    assert!(map.len() <= len);
}
#[test]
fn gen_tree() {
    let mut tm = HashMap::new();
    tm.insert(4, vec![1, 2, 3]);
    tm.insert(5, vec![4, 6]);
    let mut seq = vec![4, 4, 4, 5];
    let map = generate_tree(&mut seq);
    println!("{:?}", map);
    assert!(map == tm);
}
