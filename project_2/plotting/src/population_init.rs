use rand_chacha::rand_core::RngCore;
use crate::individual::individual::Route;
use crate::parsing::TrainData;
use crate::individual::chromosome::Chromosome;
use rand::prelude::SliceRandom;
use crate::population_init::individual_init::random_chromo;
use crate::individual::individual::{calculate_fitness, Individual};
//use kmeans::*;

pub(crate) mod individual_init {
    use super::*;

    pub fn random_route(rng: &mut dyn RngCore, t_data: &TrainData) -> Route {

        //TODO: thread_rng? or keep using seeded chacha8?
        //let mut rng = ChaCha8Rng::from_seed([42; 32]);
        let mut vec: Vec<u16> = (1..101).collect();
        //add nurse delimiters
        vec.append(&mut vec![0u16; 24]);
        vec.shuffle(rng);
        let chromo: Chromosome = vec.into_iter().collect();
        let (fitness, feasible)= calculate_fitness(&chromo, t_data);
        let r = Route::create(chromo, fitness, !(feasible > 0.0));
        r

    }

    pub fn nurse_distributed_route(rng: &mut dyn RngCore, t_data: &TrainData) -> Route {
        //TODO: thread_rng? or keep using seeded chacha8?
        //let mut rng = ChaCha8Rng::from_seed([42; 32]);

        let num_nurses: usize = t_data.nbr_nurses as usize;
        let num_patients: usize = t_data.patients.len();
        let step = (num_patients as f32 / num_nurses as f32).round() as usize;
        let mut vec: Vec<u16> = (1..(num_patients+1) as u16).collect();
        //add nurse delimiters
        //vec.append(&mut vec![0u16; 24]);
        vec.shuffle(rng);
        for i in 1..num_nurses {
            let index = (i * (step + 1)) - 1;
            if index > vec.len() {
                vec.insert(vec.len(), 0);
            } else { vec.insert(index, 0) }
        }
        let chromo: Chromosome = vec.into_iter().collect();
        let (fitness, feasible)= calculate_fitness(&chromo, t_data);
        let r = Route::create(chromo, fitness, !(feasible > 0.0));
        r
    }

    pub fn random_chromo(rng: &mut dyn RngCore) -> Chromosome {
        let mut vec: Vec<u16> = (1..101).collect();
        //add nurse delimiters
        vec.append(&mut vec![0u16; 24]);
        vec.shuffle(rng);
        let chromo: Chromosome = vec.into_iter().collect();
        chromo
    }

    pub fn random_chromo_no_delimit(rng: &mut dyn RngCore, num_patients: u16) -> Chromosome {
        let mut vec: Vec<u16> = (1..num_patients + 1).collect();
        vec.shuffle(rng);
        let chromo: Chromosome = vec.into_iter().collect();
        chromo
    }


    pub fn nurse_distributed_chromo(rng: &mut dyn RngCore, t_data: &TrainData) -> Chromosome {
        //TODO: thread_rng? or keep using seeded chacha8?
        //let mut rng = ChaCha8Rng::from_seed([42; 32]);

        let num_nurses: usize = t_data.nbr_nurses as usize;
        let num_patients: usize = t_data.patients.len();
        let step = (num_patients as f32 / num_nurses as f32).round() as usize;

        let mut vec: Vec<u16> = (1..(num_patients+1) as u16).collect();
        //add nurse delimiters
        //vec.append(&mut vec![0u16; 24]);
        vec.shuffle(rng);
        for i in 1..num_nurses {
            let index = (i * (step + 1)) - 1;
            if index > vec.len() {
                vec.insert(vec.len(), 0);
            } else { vec.insert(index, 0) }
        }
        let chromo: Chromosome = vec.into_iter().collect();
        chromo
    }
}


pub(crate) mod pop_init {
    use crate::population_init::individual_init::{nurse_distributed_chromo, random_chromo_no_delimit};
    use super::*;


    pub trait PopulationGenerator
    {
        fn generate_population<I>(&self, data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I>
        where I: Individual;
    }

    pub struct RandomPopulation;

    impl RandomPopulation{
        pub fn new() -> Self {Self }
    }

    impl PopulationGenerator for RandomPopulation
    {
        fn generate_population<I>(&self, data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I>
        where I: Individual
        {
            let mut population: Vec<I> = Vec::new();
            for _ in 0..pop_size {
                let chromo = random_chromo(rng);
                let (fitness, feasible) = calculate_fitness(&chromo, &data);
                population.push(I::create(chromo, fitness, !(feasible > 0.0)));
            }
            population
        }
    }


    pub struct RandomPopulationNoDelim;

    impl RandomPopulationNoDelim{
        pub fn new() -> Self {Self }
    }

    impl PopulationGenerator for RandomPopulationNoDelim
    {
        fn generate_population<I>(&self, data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I>
            where I: Individual
        {
            let mut population: Vec<I> = Vec::new();
            for _ in 0..pop_size {
                let chromo = random_chromo_no_delimit(rng, data.patients.len() as u16);
                let (fitness, feasible) = calculate_fitness(&chromo, &data);
                population.push(I::create(chromo, fitness, !(feasible > 0.0)));
            }
            population
        }
    }


    pub struct BalancedRoutes;
    impl BalancedRoutes
    {
        pub fn new() -> Self {Self }
    }
    impl PopulationGenerator for BalancedRoutes {
        fn generate_population<I>(&self, data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I> where I: Individual {
            let mut population: Vec<I> = Vec::new();
            for _ in 0..pop_size {
                let chromo = nurse_distributed_chromo(rng, &data);
                let (fitness, feasible) = calculate_fitness(&chromo, &data);
                population.push(I::create(chromo, fitness, !(feasible > 0.0)))
            }
            population
        }
    }

    pub fn init_pop_random<I>(data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I>
        where I: Individual,
    {
        let mut population: Vec<I> = Vec::new();
        for _ in 0..pop_size {
            let chromo = random_chromo(rng);
            let (fitness, feasible) = calculate_fitness(&chromo, &data);
            population.push(I::create(chromo, fitness, !(feasible > 0.0)))
        }
        population
    }

    pub fn init_pop_random_no_delim<I>(data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I>
        where I: Individual,
    {
        let mut population: Vec<I> = Vec::new();
        for _ in 0..pop_size {
            let chromo = random_chromo_no_delimit(rng, data.patients.len() as u16);
            let (fitness, feasible) = calculate_fitness(&chromo, &data);
            population.push(I::create(chromo, fitness, !(feasible > 0.0)))
        }
        population
    }

    pub fn init_pop_even_chromosomes<I>(data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I>
        where I: Individual,
    {
        let mut population: Vec<I> = Vec::new();
        for _ in 0..pop_size {
            let chromo = nurse_distributed_chromo(rng, &data);
            let (fitness, feasible) = calculate_fitness(&chromo, &data);
            population.push(I::create(chromo, fitness, !(feasible > 0.0)))
        }
        population
    }

    pub fn init_pop_kmeans<I>(data: &TrainData, pop_size: usize, rng: &mut dyn RngCore) -> Vec<I>
        where I: Individual
    {
        let mut coords = vec![0; data.patients.len() * 2];

        for (_, p) in data.patients.iter() {
            coords.push(p.x_coord);
            coords.push(p.y_coord);
        }
        //let kmean = KMeans::new(coords, 200, 2);
        //let result = kmean.kmeans_lloyd(10, 100, KMeans::init_kmeanplusplus, &KMeansConfig::default());
        /*
        println!("Centroids: {:?}", result.centroids);
        println!("Cluster-Assignments: {:?}", result.assignments);
        println!("Error: {}", result.distsum);
        */
        todo!()

    }
}
