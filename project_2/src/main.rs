extern crate core;

use std::{env, thread};
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use rand::{Rng, thread_rng};

mod mutation;
mod gen_alg;
mod parsing;
mod crossover;
mod individual;
mod plotting;
#[cfg(test)]
mod tests;
mod selection;
mod fitness;
mod population_init;
mod helper;
mod kmeans;

use std::time::{Instant};

use rand::seq::IteratorRandom;
use rand_chacha::rand_core::RngCore;
use crate::gen_alg::GenAlg;
use crate::individual::individual::{Individual, Route};
use crate::mutation::{MutationHolder};
use crate::population_init::pop_init::PopulationGenerator;
use crate::selection::{ParentSelection, SurvivorSelection};

//const PYTHON_SCRIPT_PATH: &str = r"C:\Users\Axel\PycharmProjects\axel_tools\bio_ai\bio_ai_plotter.py";
const PYTHON_SCRIPT_PATH: &str = r"plotting\bio_ai_plotter.py";
const MIN_POP_DEV:f64 = 4.0;
const P_MUT_MIN: f64 = 0.07;
const XOVER_PROB: f64 = 0.97;

pub fn island<I, S, P, G>(mut population: Vec<I>, mut gen_alg: GenAlg<S, P>, pop_gen: G, pop_size: usize, rng: &mut dyn RngCore, sender: Sender<Vec<I>>, receiver: Receiver<Vec<I>>) -> I
    where P: ParentSelection, S: SurvivorSelection, I: Individual, G: PopulationGenerator,
{

    const SEND_INTERVAL:usize = 150;
    const NUM_SEND_I:usize = 5;

    let mut prev: f64 = 0.0;
    let mut stagnation_counter: u8 = 0;
    for i in 0..1 00 {
        //println!("{:?}", helper::avg_fitness(&population));
        population = gen_alg.evolve(rng, &population);
        //let best_solution= helper::best_fitness(&population);
        //println!("{:?}", calculate_fitness(&best_solution.chromosome(), &data));


        if i % 10 == 0
        {
            let current_avg = helper::avg_fitness(&population);
            if prev == current_avg
            {
                helper::keep_best_n::<I>(&mut population, 2);
                population.extend(pop_gen.generate_population::<I>(gen_alg.t_data(), pop_size - 2, rng));
                stagnation_counter = 0;
            }
            prev = current_avg;
        }

        if i % 100 == 0
        {
            println!("Gen {}", i);
        }

        if i % SEND_INTERVAL == 0
        {
            let sample: Vec<&I> = population.iter().choose_multiple(rng, NUM_SEND_I);
            let mut vec: Vec<I> = Vec::new();
            for i in sample.iter()
            {
                vec.push(I::create(i.chromosome().clone(), i.fitness(), i.feasible()))
            }

            //Fix send errors
            sender.send(vec).unwrap_or(());
            //thread::sleep(Duration::from_millis(100));
            //let msg = receiver.recv().unwrap();

        }

        //population.extend(msg);
        for x in receiver.try_iter()
        {
            population.extend(x);
        }
    }

    println!("Goodbye.");
    let best_solution= helper::best_fitness(&population);
    I::create(best_solution.chromosome().clone(), best_solution.fitness(), best_solution.feasible())

}

pub fn setup_tx_rx<I>(n_threads: usize) -> (Vec<Sender<Vec<I>>>, Vec<Receiver<Vec<I>>>)
where I: Individual {
    let (senders, receivers): (Vec<Sender<Vec<I>>>, Vec<Receiver<Vec<I>>>) = (0..n_threads).into_iter().map(|_| mpsc::channel()).unzip();
    (senders, receivers)
}

pub fn mutation_vec(rng: &mut dyn RngCore) -> MutationHolder
{
    let mut holder = MutationHolder::new();
    //holder.register(mutation::InversionMutation::new(0.5));
    //holder.register(mutation::ScrambleMutation::new(0.5));
    holder.register(mutation::cross_route::CrossRouteInsertMutation::new(rng.gen_range(0.06..0.15)));
    holder.register(mutation::cross_route::CrossRouteSwapMutation::new(rng.gen_range(0.06..0.15)));
    holder.register(mutation::in_route::InRouteSwapMutation::new(rng.gen_range(0.06..0.15)));
    holder.register(mutation::in_route::InRouteInsertMutation::new(rng.gen_range(0.06..0.15)));
    holder.register(mutation::in_route::InRouteInversionMutation::new(rng.gen_range(0.06..0.15)));
    holder.register(mutation::in_route::InRouteScrambleMutation::new(rng.gen_range(0.06..0.15)));
    holder
}

fn main() {
    const NTHREADS: usize = 8;

    let (mut senders, receivers) = setup_tx_rx(NTHREADS);
    senders.rotate_left(1);
    //let island_configs = setup_island_configs();

    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1);
    let filepath = &args[1];
    let data = parsing::parse_json(&filepath.clone());


    let now = Instant::now();

    let children : Vec<_> = senders
        .into_iter()
        .zip(receivers.into_iter())
        .enumerate()
        .map(|(_, (tx, rx))| {
            let d = data.clone();
            thread::spawn(move || {
                let mut rng = thread_rng();

                let population = population_init::pop_init::init_pop_random::<Route>(&d, 300, &mut rng);
                let algo = gen_alg::GenAlg::new(
                    selection::ElitismSurvivorSelection::new(),
                    selection::TournamentParentSelection::new(2),
                    crossover::OrderOneCrossover::new(),
                    &d,     mutation_vec(&mut rng),
                );

                let t = island(population, algo, population_init::pop_init::RandomPopulation::new(), 300, &mut rng, tx, rx);
                t
                //println!("thread {} sent: {}", i, i);
                //println!("thread {} recv: {:?}", i, rx.recv().unwrap());
            })
        }).collect();

    let mut best_solutions: Vec<Route> = Vec::new();
    for child in children {
        best_solutions.push(child.join().unwrap());
    }

    let mut b: &Route = &best_solutions[0];
    let mut sol_string = String::from(&data.instance_name) + "\n";
    for solution in best_solutions.iter()
    {
        if solution.feasible()
        {
            if solution.fitness() < b.fitness()
            {
                b = solution;
            }

        }
        sol_string.push_str( &String::from(solution.chromosome().format_chromosome() + "\n" + &solution.fitness().to_string() + " " +  &solution.feasible().to_string() + "\n"));
    }

    //let unique_best = best_solutions.into_iter().unique().collect();
   // println!("{:?}", unique_best);

    parsing::write_solution_to_file( &sol_string);
    parsing::pretty_print_solution_to_file(&helper::gen_solution_string(&b.chromosome(), &data));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);


    //Note that this only works on Windows. Plotting solution and instance using Python
    let output = Command::new("cmd").args(["/K", "python", PYTHON_SCRIPT_PATH]).output().expect("Failed!!");
    //println!("{:?}", String::from_utf8(output.stdout))
}
