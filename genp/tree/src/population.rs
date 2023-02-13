#![allow(dead_code, unused)]

use std::fmt::Debug;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

use crate::{
    constraints::get_nodes,
    init::{get_test_tree, get_test_tree_with},
    nodes::{
        get_node_by_id, get_node_count, set_node_by_id, set_single_node_by_id,
        set_subtree_by_node_id, Node, Nodes, set_node_by_id_force,
    },
    settings::Settings,
};

#[derive(Default)]
pub struct Generation<T> {
    pub size: usize,
    pub individuals: Vec<Individual<T>>,
}

pub fn get_shortest_tree_len<T>(first: &Node<T>, second: &Node<T>) -> usize
where
    T: Debug + Clone + PartialEq + Default,
{
    let node_count_first = get_node_count(first);
    let node_count_second = get_node_count(second);
    if node_count_first > node_count_second {
        node_count_second
    } else {
        node_count_first
    }
}

pub fn get_nodes_from_trees<T>(
    first: &Node<T>,
    second: &Node<T>,
    id: usize,
) -> (Option<Node<T>>, Option<Node<T>>)
where
    T: Debug + Clone + PartialEq + Default,
{
    let first_boxed: Box<Node<T>> = Box::new(first.clone());
    let second_boxed: Box<Node<T>> = Box::new(second.clone());

    let res1 = get_node_by_id(&first_boxed, id);
    let res2 = get_node_by_id(&second_boxed, id);
    (res1, res2)
}

pub fn node_crossover<T>(
    mut first: Node<T>,
    mut second: Node<T>,
    constraints: &Nodes<T>,
) -> (Node<T>, Node<T>)
where
    T: Debug + Clone + PartialEq + Default,
{
    let end = get_shortest_tree_len(&first, &second);
    let id = thread_rng().gen_range(0..end);
    let (res_node1, res_node2) = get_nodes_from_trees(&first, &second, id);
    if let (Some(n1), Some(n2)) = (res_node1, res_node2) {
        set_node_by_id(&mut second, &n1, id, constraints);
        set_node_by_id(&mut first, &n2, id, constraints);
    }
    (first, second)
}

pub fn subtree_crossover<T>(mut first: Node<T>, mut second: Node<T>) -> (Node<T>, Node<T>)
where
    T: Debug + Clone + PartialEq + Default,
{
    let end = get_shortest_tree_len(&first, &second);
    let id = thread_rng().gen_range(0..end);

    if let (Some(res_node1), Some(res_node2)) = get_nodes_from_trees(&first, &second, id) {
        let cut_subtree = set_subtree_by_node_id(&mut first, &res_node2, id);
        set_subtree_by_node_id(&mut second, &res_node1, id);
    }

    (first, second)
}

pub fn tree_crossover<T>(
    first: Individual<T>,
    second: Individual<T>,
    offspring: usize,
) -> Vec<Individual<T>>
where
    T: Default + Clone + PartialEq + Debug,
{
    let mut ret: Vec<Individual<T>> = vec![];
    //for _ in (0..offspring).step_by(2) {
    let (first, second) = subtree_crossover(first.chromosome, second.chromosome);
    ret.push(Individual::new(first));
    ret.push(Individual::new(second));
    //}
    ret
}

pub fn roulette_wheel<T>(individuals: &Vec<Individual<T>>) -> IndividualTuple<T>
where
    T: Copy,
{
    let i1 = individuals
        .choose_weighted(&mut thread_rng(), |item| {
            item.fitness_percentage
        })
        .unwrap()
        .clone();
    let i2 = individuals
        .choose_weighted(&mut thread_rng(), |item| item.fitness_percentage)
        .unwrap()
        .clone();
    IndividualTuple {
        first: i1,
        second: i2,
    }
}

impl<T> Generation<T>
where
    T: Copy + Clone + Default + Debug + PartialEq ,
{
    pub fn new(size: usize) -> Generation<T> {
        Generation {
            size,
            ..Default::default()
        }
    }

    pub fn populate(
        &mut self,
        nodes: &Nodes<T>,
        settings: &Settings,
        func: fn(usize, &Nodes<T>, &Settings) -> Vec<Individual<T>>,
    ) {
        let gen = func(self.size, nodes, &settings);
        self.individuals = gen;
    }

    pub fn crop(&mut self, selection_function: fn(f64, &Individual<T>) -> bool) {
        let avg = self.individuals.iter().fold(0.0, |acc, x| acc + x.fitness) / (self.size as f64);
        self.individuals = self
            .individuals
            .clone()
            .into_iter()
            .filter(|x| selection_function(avg, x))
            .collect();
    }

    pub fn sort_by_fitness(&mut self) {
        self.individuals
            .sort_by(|a, b| b.fitness.total_cmp(&a.fitness));
    }

    pub fn select_elites(&mut self, percentage: f64) -> Vec<Individual<T>> {
        let inds = self.individuals.clone();
        inds.into_iter()
            .take((self.size as f64 * percentage) as usize)
            .map(|mut x| {x.elite = true; x.chromosome = self.deep_cp(&x); x})
            .collect()
    }

    pub fn deep_cp(&mut self, unit: &Individual<T>) -> Node<T>{
        let mut node = Node::new(unit.chromosome.id);
        node.value = unit.chromosome.value;
        for i in 0..unit.chromosome.children.len(){
            let ind = unit.chromosome.children[i].clone();
            let mut child  = Node::new(ind.id);
            child.value = ind.value;
            node.children.push(child);
        }
        node
    }

    pub fn mutate(&mut self, setting: &Settings, nodes: &Nodes<T>, mut_rate: f64){
        let mut val: i32;
        let mut s_val: i32;
//        let m_rate : i32  = (setting.population.mutation_rate * 100.0) as i32;
        let m_rate : i32  = (setting.population.mutation_rate * 100.0) as i32;
        for i in 0..self.individuals.len(){
            val = thread_rng().gen_range(0..100) ;
            s_val = thread_rng().gen_range(0..100) ;

            if val < m_rate && !self.individuals[i].elite{
                let end = get_node_count(&self.individuals[i].chromosome);
                let id = thread_rng().gen_range(0..end);
                if s_val > 50{
                    self.individuals[i].chromosome = self.individuals[i].mutate(id);
                }else {
                    self.individuals[i].chromosome.children.shuffle(&mut thread_rng());



               // } else if s_val >= 25{
               //     let children = self.individuals[i].chromosome.children.clone();
               //     self.individuals[i].chromosome.children = children.into_iter().filter(|x| x.id != id).collect();
               // } else if s_val < 25 && self.individuals[i].chromosome.children.len() < setting.population.tree_width {
               //     let mut rng = rand::thread_rng();
               //     if let Some(add_val) = nodes.leafs.choose(&mut rng) {
               //         let mut new_node: Node<T> = Node::new(end + 1);
               //         new_node.value = *add_val;
               //         self.individuals[i].chromosome.children.push(new_node);
               //     }

               }
            }
        }
    }

    pub fn set_fitness_percentages(&mut self) {
        let max: f64 = self.individuals.iter().fold(0.0, |acc, f| acc + f.fitness)
            / self.individuals.len() as f64;
        let inds = self.individuals.clone();
        self.individuals = inds
            .into_iter()
            .map(|mut f| {
                f.fitness_percentage = (f.fitness / max);
                f
            })
            .collect();
    }

    pub fn crossover(
        &mut self,
        offspring: usize,
        combine: fn(Individual<T>, Individual<T>, usize) -> Vec<Individual<T>>,
        selection: fn(&Vec<Individual<T>>) -> IndividualTuple<T>,
        inds: Vec<Individual<T>>
    ) -> Vec<Individual<T>> {
        let end = self.size / offspring;
        let mut new_inds: Vec<Individual<T>> =  vec![];
        for _ in 0..end {
            let parents = selection(&inds);
            new_inds.append(&mut combine(parents.first, parents.second, offspring));
        }
        new_inds
    }

    pub fn handle_generation_update(
        &mut self,
        offspring: usize,
        combine: fn(Individual<T>, Individual<T>, usize) -> Vec<Individual<T>>,
        selection: fn(&Vec<Individual<T>>) -> IndividualTuple<T>,
        elite_percentage: f64,
    ) {
        self.sort_by_fitness();

        let split_at = (self.size - (self.size as f64 * elite_percentage) as usize);

        let mut elites = self.select_elites(elite_percentage);
        let mut inds = self.individuals.clone();
        inds.split_off(inds.len()/2);
        let mut sec = inds.clone();
        inds.append(&mut sec);
        let mut next_gen = self.crossover(offspring, combine, selection, inds);
        next_gen.split_off(split_at);
        next_gen.append(&mut elites);


        // maybe a new generation should be returned instead
        self.individuals = next_gen;
    }
}

pub struct IndividualTuple<T> {
    pub first: Individual<T>,
    pub second: Individual<T>,
}

#[derive(Clone, Default, Debug)]
pub struct Individual<T> {
    pub chromosome: Node<T>,
    pub fitness: f64,
    pub fitness_percentage: f64,
    pub elite : bool
}

impl<T> Individual<T>
where
    T: Default + Clone  ,
{
    pub fn new(chromosome: Node<T>) -> Self {
        Self {
            chromosome,
            ..Default::default()
        }
    }

    pub fn mutate(&mut self, id: usize) -> Node<T> {
        let constraints = get_nodes();
        let boxed: Box<Node<T>> = Box::new(self.chromosome.clone());
        let res1 = get_node_by_id(&boxed, id);
        set_node_by_id_force(&mut self.chromosome, &res1.unwrap(), id);
        self.chromosome.clone()
    }
}

#[test]
fn gen_tree_tree_crossover() {
    let expr1 = get_test_tree_with(2, 3);
    let expr2 = get_test_tree_with(2, 3);
    let (expr1, expr2) = subtree_crossover(expr1, expr2);

}

#[test]
fn gen_tree_node_crossover() {
    let constraints = get_nodes();
    let expr1 = get_test_tree_with(1, 2);
    let expr2 = get_test_tree_with(1, 2);
    let (expr1, expr2) = node_crossover(expr1, expr2, &constraints);

}
