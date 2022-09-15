use crate::{constraints::*, nodes::*};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};



pub fn to_list<T>(node: &Node<T>, delimeter: &(T, T)) -> Vec<T>
    where T: Default + Debug + Copy
{
    let mut expr: Vec<T> = vec![];
    if !node.children.is_empty(){
        expr.push(delimeter.0);
    }
    expr.push(node.value);
    for sub_t in node.children.iter(){
       expr.append(&mut to_list(sub_t, delimeter));
    }

    if !node.children.is_empty(){
        expr.push(delimeter.1);
    }
    expr
}

pub fn to_xmly<T>(node: &Node<T>, delimeter: &(T,T,T)) -> Vec<T>
    where T: Default + Debug + Copy
{
    let mut expr: Vec<T> = vec![];
    expr.push(delimeter.0);
    expr.push(node.value);

    if node.children.is_empty(){
        expr.push(delimeter.1);
    }
    expr.push(delimeter.2);

    for sub_t in node.children.iter(){
       expr.append(&mut to_xmly(sub_t, delimeter));

    }

    if node.children.is_empty(){
        return expr
    }
    expr.push(delimeter.0);
    expr.push(delimeter.1);
    expr.push(node.value);
    expr.push(delimeter.2);


    expr
}



pub fn ramped_half_half<'a>(
    size: usize,
    nodes: &Nodes<&'a str>,
    config: &Settings,
) -> Vec<Individual<Node<&'a str>>> {
    let mut chroms: Vec<Individual<Node<&str>>> = vec![];
    for x in 0..size {
        let typ = x < size / 2;
        let depth = config.population.tree_depth;
        let width = config.population.tree_width;
        if let Some(tree) = gen_rnd_expr_tree(nodes, depth, width as u8, typ) {
            chroms.push(Individual {
                chromosome: tree,
                fitness: 0.0,
                id: x,
            });
        }
    }
    chroms
}

pub fn gen_rnd_expr_tree<'a>(
    nodes: &Nodes<&'a str>,
    depth: usize,
    width: u8,
    is_grow: bool,
) -> Option<Node<&'a str>> {
    let mut expr: Node<&str> = Node::new();
    let ind: usize = nodes.leafs.len() / (nodes.leafs.len() + nodes.intermediate.len());
    let mut rng = rand::thread_rng();
    if depth == 0 || is_grow && rng.gen_range(0..=100) < ind {
        if let Some(val) = nodes.leafs.choose(&mut rng) {
            expr.value = *val;
        }
    } else {
        let inter = nodes.intermediate.choose(&mut rng).unwrap();
        expr.value = inter.value;
        let mut arity = 1;
        if inter.random_arity {
            arity = rng.gen_range(1..=width);
        }
        for _ in 0..arity {
            if let Some(node) = gen_rnd_expr_tree(nodes, depth - 1, width, is_grow) {
                expr.children.push(node);
            }
        }
    }
    Some(expr)
}

pub fn gen_rnd_expr<T>(
    nodes: &Nodes<T>,
    delimeter: &(T, T),
    config: &Settings,
    depth: usize,
    is_grow: bool,
) -> Vec<T>
where
    T: Copy,
{
    let mut expr: Vec<T> = vec![];
    let ind: usize = nodes.leafs.len() / (nodes.leafs.len() + nodes.intermediate.len());
    let mut rng = rand::thread_rng();
    if depth == 0 || is_grow && rng.gen_range(0..=100) < ind {
        if let Some(val) = nodes.leafs.choose(&mut rng) {
            expr.push(*val);
        }
    } else {
        if rng.gen_range(0.0..=100.0) <= config.population.empty_branch_rate * 100.0 {
            return vec![];
        }

        let inter = nodes.intermediate.choose(&mut rng).unwrap();
        expr.push(delimeter.0);
        expr.push(inter.value);
        let mut arity = 1;
        if inter.random_arity {
            arity = rng.gen_range(1..2);
        }
        for _ in 0..arity {
            expr.append(&mut gen_rnd_expr(
                nodes,
                delimeter,
                config,
                depth - 1,
                is_grow,
            ))
        }
        expr.push(delimeter.1);
    }
    expr
}
use std::{time::{Duration, Instant}, fmt::Debug, fs};

use crate::{
    population::{to_xml, Individual},
    settings::{self, Population, Settings},
};
//
//#[test]
//fn gen_full_tree_list() {
//    let nodes = get_nodes();
//    let start = Instant::now();
//    let depth = 3;
//    let width = 3;
//    let expr = gen_rnd_expr_tree(&nodes, depth, width, false).unwrap();
//    to_list(expr);
//    //    println!("{:?}", expr);
//    let duration = start.elapsed();
//    println!("Time elapsed in expensive_function() is: {:?}", duration);
//}
//
//
//
//#[test]
//fn gen_full_tree() {
//    let nodes = get_nodes();
//    let start = Instant::now();
//    let depth = 3;
//    let width = 3;
//    let expr = gen_rnd_expr_tree(&nodes, depth, width, false).unwrap();
//    expr.bfs();
//    //    println!("{:?}", expr);
//    let duration = start.elapsed();
//    println!("Time elapsed in expensive_function() is: {:?}", duration);
//}
//
#[test]
fn gen_full_tree_dfs() {
    let nodes = get_nodes();
    let start = Instant::now();
    let depth = 3;
    let width = 3;
    let expr = gen_rnd_expr_tree(&nodes, depth, width, false).unwrap();
    //print!("{:?}", expr);
    println!();
    let mut xmly = to_xmly(&expr, &get_xml_delims());
    let xmly: String = xmly.into_iter().collect();
    println!("{:?}", xmly);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    assert!(false);
}
pub fn get_xml_delims<'a>() -> (&'a str, &'a str, &'a str) {
    ("<", "/", ">")
}

//#[test]
//fn chrom_generate() {
//    let size = 100;
//    let mut settings = Settings::new().unwrap();
//    settings.population.tree_depth = 3;
//    settings.population.tree_width = 3;
//    let nodes = get_nodes();
//    let start = Instant::now();
//    let inds = ramped_half_half(size, &nodes, &settings);
//    let duration = start.elapsed();
//    println!("Time elapsed in expensive_function() is: {:?}", duration);
//    print!("{:?}", inds[0].chromosome);
//}
