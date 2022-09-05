use std::time::{Instant};
use tree::population::*;
use std::{
    collections::HashSet,
};



fn vect_difference(v1: &Vec<usize>, v2: &Vec<usize>) -> Vec<usize> {
    let s1: HashSet<usize> = v1.iter().cloned().collect();
    let s2: HashSet<usize> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}

#[test]
pub fn run_hash_plus(){

    let seq = get_pruefer_seq(500);
    let n = seq.len();
    let  l: HashSet<usize> = (1..=n + 2).into_iter().collect();
    let start = Instant::now();
    let has: HashSet<usize> = seq.into_iter().collect();
    (&has - &l).iter().min();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[test]
pub fn run_hash(){

    let seq = get_pruefer_seq(500);
    let n = seq.len();
    let has: HashSet<usize> = seq.into_iter().collect();
    let  l: HashSet<usize> = (1..=n + 2).into_iter().collect();
    let start = Instant::now();
    (&has - &l).iter().min();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[test]
pub fn run_vec_to_hash(){

    let seq = get_pruefer_seq(500);
    let n = seq.len();
    let  l: Vec<usize> = (1..=n + 2).into_iter().collect();
    let start = Instant::now();

    vect_difference(&seq, &l).iter().min();

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}


#[test]
pub fn run_iter(){

    let seq = get_pruefer_seq(500);
    let n = seq.len();
    let l: HashSet<usize> = (1..=n + 2).into_iter().collect();
    let start = Instant::now();
    let v = l.iter().filter(|x| !seq.contains(x)).min();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}


