use std::{fmt::Debug, time, thread};

use crate::{
    constraints::get_nodes,
    init::{ramped_half_half, get_xml_delims, write_to_file, write_bt_to_file},
    nodes::{Nodes, get_node_count},
    population::{roulette_wheel, tree_crossover, Generation, Individual, IndividualTuple},
    settings::Settings, xml::node_to_xml_string, cmd::{docker_start, execute_BT, write_result, docker_kill_all, kill_BT, stop_refbox, docker_copy, docker_prune, exec_wait_bt}, parse::parse_points,
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
    T: Copy + Clone + Default + Debug + PartialEq,
{
    let mut count = 0;
    //Todo: settings & get_nodes
    let settings = Settings::new().unwrap();
    let mut pop = Generation::new(pop_size);
    pop.populate(nodes, &settings, init);
    loop{
        if count == cycles{
            break;
        }

    evaluate(&mut pop.individuals, count as u32);
    pop.set_fitness_percentages();
    pop.handle_generation_update(2, combine, selection, elite_percentage);
    pop.mutate(&settings, nodes);
    count += 1;
    }
}

pub fn evaluate_ref<T>(inds: &mut Vec<Individual<T>>, id: u32)
    where
    T: Default + Copy,
    String: From<T>
{
    let mut ind_id: u32 = 0;
    let fiver  = time::Duration::from_secs(5);
    let thirty  = time::Duration::from_secs(30);
    for individual in inds.iter_mut() {
        docker_prune();
        let cur_id : String = "gen_".to_owned() + &id.to_string() +"_ind_" + &ind_id.to_string();
        ind_id += 1;
        let chrom = &individual.chromosome;
        let xml: String = node_to_xml_string(chrom, &get_xml_delims());
        write_bt_to_file(&xml, "../xml/generated.xml".to_string());
        write_bt_to_file(&xml, "./log/".to_owned()+ &cur_id  );
        docker_start();
        thread::sleep(fiver);
        let mut handle = execute_BT();
        thread::sleep(thirty);
        kill_BT();
        docker_copy();
        stop_refbox();
        write_result("./output/".to_owned() + &cur_id);
        let points = parse_points("./output/".to_owned() + &cur_id) ;
        individual.fitness = points as f64;
        docker_kill_all();
    }
}

pub fn evaluate<T>(inds: &mut Vec<Individual<T>>, id: u32)
    where
    T: Default + Copy+ Debug,
    String: From<T>
{
    let mut ind_id: u32 = 0;
    for individual in inds.iter_mut() {
        let cur_id : String = "gen_".to_owned() + &id.to_string() +"_ind_" + &ind_id.to_string();
        ind_id += 1;
        let chrom = &individual.chromosome;
//        let count = get_node_count(chrom);
        let xml: String = node_to_xml_string(chrom, &get_xml_delims());
        write_bt_to_file(&xml, "../xml/generated.xml".to_string());
        write_bt_to_file(&xml, "./log/".to_owned()+ &cur_id  );
        let mut out = exec_wait_bt().unwrap();
        let out = String::from_utf8(out.stdout).unwrap();
        let points = handle_points(out);

        if points > 30 {
            println!("{:?}", cur_id);
        }

        individual.fitness = points as f64;
        write_to_file(points.to_string() ,"./output/".to_owned() + &cur_id)
    }
}

pub fn handle_points(out: String) -> i32{
    let mut out: Vec<&str> = out.split('\n').into_iter().collect();
    out.retain(|a| a != &"");
    let mut sum: Vec<i32> = out.into_iter().map(|a| a.parse::<i32>().unwrap()).collect();
    sum = sum.into_iter().map(|a| if a < 0 { 0} else {a}).collect();
    sum.into_iter().reduce(|a,b| a+b).unwrap()
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

#[test]
fn evolve() {
    let nodes = get_nodes();
    evolution_cycle(
        100,
        ramped_half_half,
        &nodes,
        100,
        0.10,
        evaluate,
        tree_crossover,
        roulette_wheel,
    );
    assert!(false);
}

//#[test]
fn eval_test(){
    let nodes = get_nodes();
    let settings = Settings::new().unwrap();
    let mut pop = Generation::new(4);
    pop.populate(&nodes, &settings, ramped_half_half);
    evaluate(&mut pop.individuals, 0);
    assert!(false)
}
