#![allow(dead_code, unused)]

pub struct Generation<T> {
    pub population: Vec<Individual<T>>,
}

pub struct Individual<T> {
    pub chromosome: T,
    pub fitness: f64,
    pub id: usize,
}
