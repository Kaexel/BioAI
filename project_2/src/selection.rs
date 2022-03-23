
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::{Rng, RngCore};
use ordered_float::OrderedFloat;
use crate::individual::individual::Individual;

pub fn general_crowding<'a, I>(b1: &'a I, b2: &'a I, phi: f64, rng: &mut dyn RngCore) -> usize
where I: Individual
{
    let p1 : f64;
    if OrderedFloat(b1.fitness()) > OrderedFloat(b2.fitness()) {
        p1 = b1.fitness() / (b1.fitness() + phi * b2.fitness())
    } else if OrderedFloat(b1.fitness()) < OrderedFloat(b2.fitness()) {
        p1 = (phi*b1.fitness()) / (phi*b1.fitness() + b2.fitness())
    } else {
        p1 = 0.5
    }
    //println!("{:?}", b1.fitness());
   // println!("{:?}", b2.fitness());

    if rng.gen_bool(p1 as f64) {0} else {1}
}

pub fn boltzmann_operator<I>(current: &I, neighbor: &I, fmax: f64, favg: f64) -> f64
where I: Individual
{
    let k:f64 = 0.3;
    let delta_f = neighbor.fitness() - current.fitness();
    let prob =  if OrderedFloat(delta_f) > OrderedFloat(0.0) {1f64} else {f64::powf(std::f64::consts::E, (k * delta_f) / (fmax - favg))};
    prob
}

pub trait SurvivorSelection {
    fn select<I>(&self, rng: &mut dyn RngCore, population: &mut Vec<I>, pop_size: usize)
        where I: Individual;
}

pub trait ParentSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) ->  [&'a I; 2]
        where I: Individual;
}

pub struct TournamentParentSelection {
    tournament_size: usize,
}
impl TournamentParentSelection {
    pub fn new(tournament_size: usize) -> Self {
        Self { tournament_size }
    }
}
impl ParentSelection for TournamentParentSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> [&'a I; 2] where I: Individual {
        assert!(population.len() >= (2 * self.tournament_size));


        let sample = population.iter().choose_multiple(rng, 2 * self.tournament_size);
        let winner1: &I = sample[..self.tournament_size].iter().min_by_key(|a| OrderedFloat(a.fitness())).unwrap();
        let winner2: &I = sample[self.tournament_size..].iter().min_by_key(|a| OrderedFloat(a.fitness())).unwrap();

        [winner1, winner2]
    }
}


pub struct RouletteParentSelection;

impl RouletteParentSelection {
    pub fn new() -> Self {
        Self { }
    }
}
impl ParentSelection for RouletteParentSelection {

    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> [&'a I; 2] where I: Individual {

        let parent1 = population.choose_weighted(rng, |individual| individual.fitness()).expect("Empty population");
        let parent2 = population.choose_weighted(rng, |individual| individual.fitness()).expect("Empty population");


        [parent1, parent2]
    }
}

pub struct CrowdingSurvivorSelection;

impl CrowdingSurvivorSelection {
    pub fn new() -> Self {Self}
}

impl SurvivorSelection for CrowdingSurvivorSelection {
    fn select<I>(&self, rng: &mut dyn RngCore, population: &mut Vec<I>, pop_size: usize) where I: Individual {
        todo!()
    }
}

pub struct ElitismSurvivorSelection;

impl ElitismSurvivorSelection {
    pub fn new() -> Self  {
        Self
    }
}

impl SurvivorSelection for ElitismSurvivorSelection {
    fn select<I>(&self, rng: &mut dyn RngCore, population: &mut Vec<I>, pop_size: usize) where I: Individual {
        population.sort_by(|a, b| OrderedFloat(a.fitness()).cmp(&OrderedFloat(b.fitness())));
        population.truncate(pop_size)

    }
}

pub struct ElitismSurvivorSelectionKeepFeasible;

impl ElitismSurvivorSelectionKeepFeasible {
    pub fn new() -> Self  {
        Self
    }
}

impl SurvivorSelection for ElitismSurvivorSelectionKeepFeasible {
    fn select<I>(&self, rng: &mut dyn RngCore, population: &mut Vec<I>, pop_size: usize) where I: Individual {
        population.sort_by(|a, b| OrderedFloat(a.fitness()).cmp(&OrderedFloat(b.fitness())));
        let mut t: Vec<I> = population.drain(..pop_size).into_iter().collect();
        t.retain(|f| f.feasible());
        t.sort_by(|a, b| OrderedFloat(a.fitness()).cmp(&OrderedFloat(b.fitness())));
        population.append(&mut t);
        let _ = population.drain(population.len() - pop_size..);

    }
}
