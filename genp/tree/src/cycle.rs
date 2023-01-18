use std::{fmt::Debug, time, thread};


use crate::{
    constraints::get_nodes,
    init::{ramped_half_half, get_xml_delims, write_to_file, write_bt_to_file},
    nodes::Nodes,
    population::{roulette_wheel, tree_crossover, Generation, Individual, IndividualTuple},
    settings::Settings, xml::node_to_xml_string, cmd::{docker_start, execute_BT, write_result, docker_kill_all, kill_BT, stop_refbox, docker_copy, docker_prune},
};

pub fn evolution_cycle<T>(
    cycles: usize,
    init: fn(usize, &Nodes<T>, &Settings) -> Vec<Individual<T>>,
    nodes: &Nodes<T>,
    pop_size: usize,
    elite_percentage: f64,
    evaluate: fn(&mut Vec<Individual<T>>, id: u32),
    //    crop: fn(f64, &Individual<T>) -> bool,
    //    mutation: fn(&Individual<T>) -> Individual<T>,
    combine: fn(Individual<T>, Individual<T>, usize) -> Vec<Individual<T>>,
    selection: fn(&Vec<Individual<T>>) -> IndividualTuple<T>,
) where
    T: Copy + Clone + Default + Debug,
{
    let mut count = 0;
    //Todo: settings & get_nodes
    let settings = Settings::new().unwrap();
    let mut pop = Generation::new(pop_size);
    loop{
        let start = Instant::now();
        if count == cycles{
            break;
        }


    pop.populate(nodes, &settings, init);
    evaluate(&mut pop.individuals, count as u32);
    pop.set_fitness_percentages();
    pop.handle_generation_update(2, combine, selection, elite_percentage);
    count += 1;
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}

pub fn evaluate<T>(inds: &mut Vec<Individual<T>>, id: u32)
    where
    T: Default + Copy,
    String: From<T>
{
    let mut ind_id: u32 = 0;
    let fiver  = time::Duration::from_secs(5);
    let thirty  = time::Duration::from_secs(30);
    for individual in inds.iter_mut() {
        docker_prune();
        ind_id += 1;
        //chromosome.fitness = rand::thread_rng().gen_range(1..100) as f64;
        let chrom = &individual.chromosome;
        let xml: String = node_to_xml_string(chrom, &get_xml_delims());
       // write_bt_to_file(xml, "../xml/generated.xml".to_string());
        docker_start();
        thread::sleep(fiver);
        let mut handle = execute_BT().unwrap();
        thread::sleep(thirty);
        handle.kill();
        kill_BT();
        docker_copy();
        stop_refbox();
        let cur_id : String = "gen_".to_owned() + &id.to_string() +"_ind_" + &ind_id.to_string();
        write_result("./output/".to_owned() + &cur_id);
//        docker_kill_all();

    }
}
pub fn crop<T>(pop_fitness: f64, ind: &Individual<T>) -> bool {
    ind.fitness > pop_fitness
}

fn ramped_hh() {
    let nodes = get_nodes();
    let size = 10;
    let config = Settings::new().unwrap();
    let _ret = ramped_half_half(size, &nodes, &config);
}

use std::time::{ Instant};

//#[test]
fn evolve() {
    let nodes = get_nodes();
        let start = Instant::now();
    evolution_cycle(
        10,
        ramped_half_half,
        &nodes,
        100,
        0.25,
        evaluate,
        tree_crossover,
        roulette_wheel,
    );
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    assert!(false);
}

#[test]
fn eval_test(){

    let nodes = get_nodes();
    let settings = Settings::new().unwrap();
    let mut pop = Generation::new(1);
    println!("REEEEEEEEEE" );
    pop.populate(&nodes, &settings, ramped_half_half);
    evaluate(&mut pop.individuals, 0);
    assert!(false)

}
