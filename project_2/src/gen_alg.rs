use rand::{Rng, RngCore};
use rand::seq::SliceRandom;
use crate::parsing::TrainData;
use crate::crossover::Crossover;
use crate::{helper, MIN_POP_DEV, P_MUT_MIN, XOVER_PROB};
use crate::helper::hamming_distance;
use crate::individual::individual::{calculate_fitness, Individual};
use crate::mutation::{MutationHolder};
use crate::selection::{general_crowding, ParentSelection, SurvivorSelection};



pub struct GenAlg<'a, S, P> {
    survivor_selection_method: S,
    parent_selection_method: P,
    crossover_method: Box<dyn Crossover>,

    t_data: &'a TrainData,
    pop_size_multiplier: usize,
    m_method_vec: MutationHolder
}
impl<'a, S, P> GenAlg<'a, S, P>
where S: SurvivorSelection, P: ParentSelection
{
    pub fn new(survivor_selection_method: S,
               parent_selection_method: P,
               crossover_method: impl Crossover + 'static,
               t_data: &'a TrainData,
               m_method_vec: MutationHolder


    ) -> Self {
        Self {
            survivor_selection_method,
            parent_selection_method,
            crossover_method: Box::new(crossover_method),
            t_data,
            pop_size_multiplier: 5,
            m_method_vec
        }
    }

    pub fn evolve<I>(&mut self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where I: Individual,
    {
        assert!(!population.is_empty());
        let mut std = helper::pop_std_dev(&population);
        if std >= MIN_POP_DEV
        {
            std = MIN_POP_DEV;
        }
        let p = P_MUT_MIN + 0.1 * (MIN_POP_DEV  - std);
        self.m_method_vec.adjust_chances(p);
        let mut new_pop: Vec<I> = (0..(population.len() * self.pop_size_multiplier))
            .map(|_| {  // iterator from function
                let parents = self.parent_selection_method.select(rng, population);

                // Create offspring proportional with XOVER probabilty
                if rng.gen_bool(XOVER_PROB)
                {
                    // create child from crossover of parents
                    let mut child = self
                        .crossover_method
                        .crossover(rng, parents[0].chromosome(), parents[1].chromosome(), &self.t_data);

                    // Run anywhere from 0 to num_mutator mutations
                    for _ in 0..rng.gen_range(0..self.m_method_vec.len())
                    {
                        self.m_method_vec[rng.gen_range(0..self.m_method_vec.len())].mutate(rng, &mut child);
                    }

                    let (fitness, feasible) = calculate_fitness(&child, &self.t_data);
                    //println!("{:?}", &fitness);
                    // create child individual from chromosome
                    // (parents, I::create(child, fitness))

                    // Crowding with CF = 1
                    let child_individual = I::create(child, fitness, !(feasible > 0.0) );
                    let competitor = if hamming_distance(parents[0].chromosome(), child_individual.chromosome()) < hamming_distance(parents[1].chromosome(), child_individual.chromosome()) { parents[0] } else { parents[1] };

                    let m = general_crowding::<I>(competitor, &child_individual, 0.0, rng).clone();
                    if m == 0 { child_individual } else { I::create(competitor.chromosome().clone(), competitor.fitness(), !(feasible > 0.0)) }
                }
                // If not crossover, equal chance of choosing either parent
                else {
                    if rng.gen_bool(0.5) {I::create(parents[0].chromosome().clone(), parents[0].fitness(), parents[0].feasible())} else {  I::create(parents[1].chromosome().clone(), parents[1].fitness(),parents[1].feasible()) }
                }


            })
            .collect(); // Collect into collection of individuals


        //new_pop.extend(population.iter().map(|a| I::create(a.chromosome().clone(), a.fitness())));

        //Cull population using survivor selection method
        self.survivor_selection_method.select(rng, &mut new_pop, population.len());
        new_pop
    }

    pub fn t_data(&self) -> &TrainData
    {
        self.t_data
    }

}




pub trait Selection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where I: Individual;
}


pub struct RouletteSelection;

impl RouletteSelection {
    pub fn new() -> Self  {
        Self
    }
}

impl Selection for RouletteSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I where I: Individual {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Empty population")
    }
}

