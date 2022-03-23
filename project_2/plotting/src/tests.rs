use crate::individual::chromosome::Chromosome;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use ordered_float::OrderedFloat;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use crate::crossover::Crossover;
use crate::gen_alg::{RouletteSelection, Selection};
use crate::helper::hamming_distance;
use crate::individual::individual::{Route};
use crate::mutation::Mutation;
use crate::population_init::individual_init::{random_chromo, random_chromo_no_delimit, random_route};
use crate::selection::{general_crowding, ParentSelection};
use super::*;

pub fn valid_solution() {
    todo!();
}

pub fn valid_chromosome(chromo: &Chromosome) {
    let v = chromo.iter().filter(|&n| *n == 0).count();
    assert_eq!(v, 24);

    for i in 1..101
    {
        let t = chromo.iter().filter(|&n| *n == i).count();
        assert_eq!(t, 1);
    }
}

pub fn valid_chromosome_nurseless(chromo: &Chromosome)
{
    for i in 1..101
    {
        let t = chromo.iter().filter(|&n| *n == i).count();
        assert_eq!(t, 1);
    }
}

#[test]
pub fn point_mean()
{
    let mut points: Vec<(u16, u16)> = vec![(2, 2), (8, 4), (5, 9)];
    let t = kmeans::point_mean(&points);
    assert_eq!(t, (5.0, 5.0));
}


#[test]
pub fn nurse_distributed_route()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");

    let t = population_init::individual_init::nurse_distributed_route(&mut rng, &data);

    valid_chromosome(&t.chromosome());
}

#[test]
pub fn deterministic_crowding()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");

    // Tests that it picks best fitness five times
    //or _ in 0..100 {
    //

    //   let best_fitness = std::cmp::max(OrderedFloat(parent_one.fitness()), OrderedFloat(parent_two.fitness()));

    //   let best_winner = general_crowding::<Route>(&parent_one, &parent_two, 0.0f64, &mut rng);

    //   //assert_eq!(best_fitness, best_winner.fitness())
    //}
}

#[test]
pub fn probabilistic_crowding()
{
    todo!()
}

#[test]
pub fn hamming()
{
    let mut rng = ChaCha8Rng::from_seed([43; 32]);
    let data = parsing::parse_json("train/train_0.json");
    let parent1 = random_chromo(&mut rng);
    let parent2 = random_chromo(&mut rng);

    let crossover = crossover::OrderOneCrossover::new();

    let mut child = crossover
        .crossover(&mut rng, &parent1, &parent2, &data);

    let mut child_2 = crossover
        .crossover(&mut rng, &parent2, &parent1, &data);

    println!("{:?}", hamming_distance(&parent1, &child));
    println!("{:?}", hamming_distance(&parent1, &child_2));
    println!("{:?}", hamming_distance(&parent2, &child));
    println!("{:?}", hamming_distance(&parent2, &child_2));

}

#[test]
pub fn tournament_selection() {
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");
    let mut population: Vec<Route> = Vec::new();
    for i in 0..100 {
        population.push(Route::new(random_chromo(&mut rng), &data));
    }

    println!("{:?}", &population);
    let parent_s = selection::TournamentParentSelection::new(2);

    let parents = parent_s.select(&mut rng, &population);

    println!("{:?}", parents);
}

#[test]
pub fn order_one_crossover()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");

    let mut parent_one = random_chromo(&mut rng);
    let mut parent_two = random_chromo(&mut rng);

    assert_ne!(&parent_one.genes, &parent_two.genes);

    let crossover = crossover::OrderOneCrossover::new();

    let chromie = crossover.crossover(& mut rng, &parent_one, &parent_two, &data );
    valid_chromosome(&chromie);
}

#[test]
pub fn heuristic_crossover()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");
    let mut parent_one = random_chromo_no_delimit(&mut rng, 100);
    let mut parent_two = random_chromo_no_delimit(&mut rng, 100);

    assert_ne!(&parent_one.genes, &parent_two.genes);

    let crossover = crossover::HeuristicCrossover::new();

    let chromie = crossover.crossover(& mut rng, &parent_one, &parent_two, &data);
    valid_chromosome_nurseless(&chromie);
}

#[test]
pub fn merge_crossover()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");
    let mut parent_one = random_chromo_no_delimit(&mut rng, 100);
    let mut parent_two = random_chromo_no_delimit(&mut rng, 100);

    assert_ne!(&parent_one.genes, &parent_two.genes);

    let crossover = crossover::MergeCrossover::new();

    let chromie = crossover.crossover(& mut rng, &parent_one, &parent_two, &data);
    valid_chromosome_nurseless(&chromie);
}


#[test]
pub fn valid_random_chromosome()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");

    let roote = random_route(&mut rng, &data);
    valid_chromosome(&roote.chromosome());

}

#[test]
pub fn chromosome_inversion()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::InversionMutation::new(1f64);
    let chromo_copy = chromo.clone();
    mutation.mutate(&mut rng, &mut chromo);

    println!("{:?}", chromo);
    println!("{:?}", chromo_copy);

    //TODO: fiks dette Axel, nå sjekker du bare at muteringen gjør noe
    assert_ne!(chromo.genes, chromo_copy.genes);
}
#[test]
pub fn in_route_inversion()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::in_route::InRouteInversionMutation::new(1f64);
    let chromo_copy = chromo.clone();
    mutation.mutate(&mut rng, &mut chromo);

    println!("{:?}", chromo);
    println!("{:?}", chromo_copy);

    //TODO: fiks dette Axel, nå sjekker du bare at muteringen gjør noe
    assert_ne!(chromo.genes, chromo_copy.genes);
}

#[test]
pub fn in_route_swap()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::in_route::InRouteSwapMutation::new(1f64);
    let chromo_copy = chromo.clone();
    mutation.mutate(&mut rng, &mut chromo);

    println!("{:?}", chromo);
    println!("{:?}", chromo_copy);

    //TODO: fiks dette Axel, nå sjekker du bare at muteringen gjør noe
    assert_ne!(chromo.genes, chromo_copy.genes);
    assert_eq!(hamming_distance(&chromo, &chromo_copy), 2);
}

#[test]
pub fn in_route_insert()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::in_route::InRouteInsertMutation::new(1f64);
    let chromo_copy = chromo.clone();
    mutation.mutate(&mut rng, &mut chromo);

    println!("{:?}", chromo);
    println!("{:?}", chromo_copy);

    //TODO: fiks dette Axel, nå sjekker du bare at muteringen gjør noe
    assert_ne!(chromo.genes, chromo_copy.genes);
    //assert_eq!(hamming_distance(&chromo, &chromo_copy), 2);
}

#[test]
pub fn cross_route_insert()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::cross_route::CrossRouteInsertMutation::new(1f64);
    let chromo_copy = chromo.clone();
    mutation.mutate(&mut rng, &mut chromo);

    println!("{:?}", chromo);
    println!("{:?}", chromo_copy);

    //TODO: fiks dette Axel, nå sjekker du bare at muteringen gjør noe
    assert_ne!(chromo.genes, chromo_copy.genes);
    //assert_eq!(hamming_distance(&chromo, &chromo_copy), 2);
}

#[test]
pub fn cross_route_swap()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::cross_route::CrossRouteSwapMutation::new(1f64);
    let chromo_copy = chromo.clone();
    mutation.mutate(&mut rng, &mut chromo);

    println!("{:?}", chromo);
    println!("{:?}", chromo_copy);

    //TODO: fiks dette Axel, nå sjekker du bare at muteringen gjør noe
    assert_ne!(chromo.genes, chromo_copy.genes);
    //assert_eq!(hamming_distance(&chromo, &chromo_copy), 2);
}


#[test]
pub fn split_into_nurses()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::in_route::InRouteInversionMutation::new(1f64);

    let mongo = helper::split_into_nurses(&chromo);
    println!("{:?}", mongo);
    println!("{:?}", chromo);

}

#[test]
pub fn push_forward_insertion()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo_no_delimit(&mut rng, 100);
    let data = parsing::parse_json("train/train_0.json");

    let test_t = helper::push_forward_insertion(&chromo, &data);

    //let mongo = helper::split_into_nurses(&chromo);

    valid_chromosome_nurseless(&test_t.into_iter().flatten().into_iter().collect());

}

#[test]
pub fn fitness_nurseless()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");

    let mut chromo = random_chromo_no_delimit(&mut rng, 100);

    let mut chromo_no_nurse = chromo.clone();
    chromo_no_nurse.genes.retain(|&x| x != 0);

    let fitness_nurse = calculate_fitness(&chromo, &data);
    let fitness_no_nurse = calculate_fitness(&chromo_no_nurse, &data);
    println!("{:?}", fitness_no_nurse);
    println!("{:?}", fitness_nurse);
    assert_eq!(fitness_nurse, fitness_no_nurse);
    //valid_chromosome_nurseless(&test.into_iter().flatten().into_iter().collect());

}

#[test]
pub fn combine_into_chromo()
{

    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mongo = helper::split_into_nurses(&chromo);

    let t = helper::combine_into_chromo(&mongo);
    println!("{:?}", mongo);
    println!("{:?}", chromo);
    println!("{:?}", t);
    assert_eq!(chromo.genes, t.genes);

}
#[test]
pub fn chromosome_scramble()
{
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut chromo = random_chromo(&mut rng);

    let mutation = mutation::ScrambleMutation::new(1f64);
    let chromo_copy = chromo.clone();
    mutation.mutate(&mut rng, &mut chromo);
    println!("{:?}", chromo);
    println!("{:?}", chromo_copy);
    //TODO: fiks dette Axel, nå sjekker du bare at muteringen gjør noe
    assert_ne!(chromo.genes, chromo_copy.genes);
}


#[test]
pub fn kmeans()
{

    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let data = parsing::parse_json("train/train_0.json");
    //population_init::init_pop_kmeans::<Route>(&data, 100, &mut rng);
    todo!()
}



