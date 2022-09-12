use std::collections::{HashMap, HashSet, VecDeque};

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub struct Nodes<T> {
    pub root: T,
    pub intermediate: Vec<IntermediateNode<T>>,
    pub leafs: Vec<T>,
}

#[derive(Copy, Clone)]
pub struct IntermediateNode<T> {
    pub value: T,
    pub random_arity: bool,
    pub arity: usize,
}

pub enum NodeType<T> {
    Symbol(T),
    Intermediate((IntermediateNode<T>, usize)),
    Leaf(T),
}

pub fn get_nodes<'a>() -> Nodes<&'a str> {
    let intermediate = intermediates();
    let leafs = leafs();
    Nodes {
        root: "root",
        intermediate,
        leafs,
    }
}

pub fn intermediates<'a>() -> Vec<IntermediateNode<&'a str>> {
    vec![
        IntermediateNode {
            value: "seq",
            arity: 2,
            random_arity: true,
        },
        IntermediateNode {
            value: "fall",
            arity: 2,
            random_arity: true,
        },
        IntermediateNode {
            value: "par",
            arity: 2,
            random_arity: true,
        },
        IntermediateNode {
            value: "pol",
            arity: 1,
            random_arity: false,
        },
    ]
}

pub fn leafs<'a>() -> Vec<&'a str> {
    vec!["pickup", "place", "move", "ready"]
}
pub fn get_delim<'a>() -> (&'a str, &'a str) {
    ("(", ")")
}

pub fn ramped_half_half<'a>(
    size: usize,
    nodes: &Nodes<&'a str>,
    config: &Settings,
) -> Vec<Individual<Vec<&'a str>>> {
    let mut chroms: Vec<Individual<Vec<&str>>> = vec![];
    for x in 0..size {
        let typ = x < size / 2;
        let tree = gen_rnd_expr_iter::<&str>(nodes, &get_delim(), config, typ);
        chroms.push(Individual {
            chromosome: tree,
            fitness: 0.0,
            id: x,
        });
    }
    chroms
}

pub fn gen_rnd_expr_iter<T>(
    nodes: &Nodes<T>,
    delimeter: &(T, T),
    config: &Settings,
    is_grow: bool,
) -> Vec<T>
where
    T: Copy,
{
    let mut expr: Vec<T> = vec![];
    let mut rng = rand::thread_rng();
    let ind: usize = nodes.leafs.len() / (nodes.leafs.len() + nodes.intermediate.len());
    let max_depth = config.population.tree_depth;
    let width = config.population.tree_width;
    let mut stack: VecDeque<NodeType<T>> = VecDeque::new();
    stack.push_front(NodeType::Intermediate((
        *nodes.intermediate.choose(&mut rng).unwrap(),
        0,
    )));

    while let Some(node) = stack.pop_front() {
        let res = match node {
            NodeType::Symbol(value) => value,
            NodeType::Intermediate((node, depth)) => {
                stack.push_front(NodeType::Symbol(delimeter.1));
                let mut arity = 1;
                if node.random_arity {
                    arity = rng.gen_range(1..width);
                }
                for _ in 0..arity {
                    if depth >= max_depth || is_grow && rng.gen_range(0..=100) < ind {
                        stack.push_front(NodeType::Leaf(*nodes.leafs.choose(&mut rng).unwrap()));
                    } else {
                    if rng.gen_range(0.0..=100.0) <= config.population.empty_branch_rate * 100.0 {
                        continue;
                    }
                        stack.push_front(NodeType::Intermediate((
                            *nodes.intermediate.choose(&mut rng).unwrap(),
                            depth + 1,
                        )));
                    }
                }
                expr.push(delimeter.0);
                node.value
            }
            NodeType::Leaf(value) => value,
        };
        expr.push(res);
    }
    expr
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
            expr.append(&mut gen_rnd_expr(nodes, delimeter, config, depth, is_grow))
        }
        expr.push(delimeter.1);
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

pub fn chrom_from_tree<'a>(
    tree: &HashMap<usize, Vec<usize>>,
    root: &usize,
    leafs: &'a [&str],
    intermediate: &'a [IntermediateNode<&str>],
) -> Vec<&'a str> {
    let mut chromosome: Vec<&str> = vec!["("];
    let mut rng = rand::thread_rng();

    if let Some(values) = tree.get(root) {
        let mut inter;
        'outer: loop {
            inter = intermediate.choose(&mut rng).unwrap();
            if values.len() == 1 && !inter.random_arity || values.len() > 1 && inter.random_arity {
                break 'outer;
            }
        }
        chromosome.push(inter.value);
        for _ in values {
            chromosome.append(&mut chrom_from_tree(tree, root, leafs, intermediate));
        }
    } else {
        let leaf = leafs.choose(&mut rng).unwrap();
        chromosome.push(leaf);
    }

    chromosome.push(")");
    chromosome
}

pub fn generate_tree(seq: &mut Vec<usize>) -> (HashMap<usize, Vec<usize>>, usize) {
    let n = seq.len();
    let s = seq.clone();
    //let mut l: HashSet<usize> = (1..=n + 2).into_iter().collect();
    let mut l: Vec<usize> = (1..=n + 2).into_iter().collect();

    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for x in s.iter() {
        let s1: HashSet<usize> = l.iter().cloned().collect();
        let s2: HashSet<usize> = seq.iter().cloned().collect();
        let v = &s1 - &s2;
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
    (map, l[0])
}

use std::time::{Duration, Instant};

use crate::{
    population::Individual,
    settings::{self, Population, Settings},
};

//#[test]
//fn pruefer_chromosome() {
//    let len = 2;
//    let inter = get_intermediate();
//    let leafs = get_leafs();
//    let start = Instant::now();
//    let mut seq = get_pruefer_seq(len);
//    let (map, root) = generate_tree(&mut seq);
//    let expr = chrom_from_tree(&map, &root,&leafs, &inter);
//    println!("{:?}", expr);
//    let duration = start.elapsed();
//    println!("Time elapsed in expensive_function() is: {:?}", duration);
//    assert!(false)
//}
#[test]
fn gen_full_iter() {
    let nodes = get_nodes();
    let settings = Settings::new().unwrap();
    let start = Instant::now();
    let expr = gen_rnd_expr_iter(&nodes, &get_delim(), &settings,  false);
    println!("{:?}", expr);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[test]
fn get_grow_iter() {
    let nodes = get_nodes();
    let mut settings = Settings::new().unwrap();
    settings.population.tree_depth = 3;
    settings.population.tree_width = 3;
    let start = Instant::now();
    let expr = gen_rnd_expr_iter(&nodes, &get_delim(), &settings,  false);
    println!("{:?}", expr);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

#[test]
fn chrom_generate(){
    let size = 100;
    let mut settings = Settings::new().unwrap();
    settings.population.tree_depth = 3;
    settings.population.tree_width = 3;
    let nodes = get_nodes();
    let start = Instant::now();
    let inds = ramped_half_half(size, &nodes, &settings);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    print!("{:?}", inds[0].chromosome);
}

#[test]
fn gen_tree_by_long_pruefer() {
    let len = 100;
    let mut seq = get_pruefer_seq(len);
    let (map, root) = generate_tree(&mut seq);
    println!("{:?}", map);
    assert!(map.len() <= len);
}
#[test]
fn gen_tree() {
    let mut tm = HashMap::new();
    tm.insert(4, vec![1, 2, 3]);
    tm.insert(5, vec![4, 6]);
    let mut seq = vec![4, 4, 4, 5];
    let (map, root) = generate_tree(&mut seq);
    println!("{:?}", map);
    assert!(map == tm);
}
