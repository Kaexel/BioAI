
use ordered_float::OrderedFloat;
use std::string::String;
use crate::individual::chromosome::Chromosome;
use crate::individual::individual::Individual;
use crate::parsing::TrainData;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


pub fn pop_std_dev<I>(population: &[I]) -> f64
where I: Individual
{
    let avg = avg_fitness(population);
    let mut sum: f64 = 0.0f64;
    for i in population.iter()
    {
        sum += (i.fitness() - avg).powf(2.0f64);
    }

    (sum / (population.len() - 1) as f64).sqrt()
}

pub fn split_into_nurses(chromosome: &Chromosome) -> Vec<Vec<u16>> {

    if chromosome.genes.contains(&0u16)
    {
        let mut split:Vec<Vec<u16>> = Vec::new();
        split.push(Vec::new());
        let mut nurse_counter:usize = 0;
        for gene in chromosome.iter() {
            if *gene == 0
            {
                nurse_counter += 1;
                split.push(Vec::new())
            } else {
                split[nurse_counter].push(*gene);
            }
        }
        split
    }
    else {
        //push_forward_insertion()
        todo!()
    }

}


pub fn combine_into_chromo(split: &Vec<Vec<u16>>) -> Chromosome {
    let mut t: Vec<u16> = Vec::new();
    for nurse in split.iter()
    {
        for patient in nurse.iter()
        {
            t.push(*patient);
        }
        t.push(0)
    }
    t.pop();
    let chromo: Chromosome = t.into_iter().collect();
    chromo
}

pub fn best_fitness<I>(population: &[I]) -> &I
where I: Individual
{
    population.iter().min_by_key(|a| OrderedFloat(a.fitness())).unwrap()
}

pub fn best_k_individuals<I>(population: &[I]) -> &I
    where I: Individual
{
    todo!()
}

pub fn keep_best_n<I>(population: &mut Vec<I>, n: usize)
where I: Individual
{
    population.sort_by(|a, b| OrderedFloat(a.fitness()).cmp(&OrderedFloat(b.fitness())));
    population.truncate(n)
}

pub fn avg_fitness<I>(population: &[I]) -> f64
    where I: Individual
{
    let avg = population.iter().map(|a| a.fitness()).sum::<f64>() / population.len() as f64;
    avg
}

pub fn hamming_distance(h1: &Chromosome, h2: &Chromosome) -> u16
{
    assert_eq!(h1.len(), h2.len());
    let t = h1.iter().zip(h2.iter());
    let mut h_dist = 0u16;
    for (one, two) in t {
        if one != two {
            h_dist += 1;
        }
    }
    h_dist
}

pub fn gen_solution_string(solution: &Chromosome, t_data: &TrainData) -> String
{
    let split = split_into_nurses(solution);

    let mut total_travel_time = 0.0f64;
    let t_matrix = &t_data.travel_times;
    let mut string_builder: String = String::new();
    for (nurse, route) in split.iter().enumerate() {

        let mut time:f64 = 0.0;
        let mut prev_patient: u16 = 0;
        let mut current_nurse_demand = 0;
        let mut path_string: String = String::from("D(0)");
        for gene in route.iter()
        {
            let current_patient = &t_data.patients[&gene.to_string()];
            let arrival_time = time + t_matrix[prev_patient as usize][*gene as usize];
            let departure_time = arrival_time + current_patient.care_time;

            time = departure_time;
            total_travel_time += t_matrix[prev_patient as usize][*gene as usize];
            current_nurse_demand +=  current_patient.demand;

            path_string.push_str(&String::from(format!("->{}({:.2}-{:.2})[{}-{}]", &gene, arrival_time, departure_time, &current_patient.start_time, current_patient.end_time)));
            prev_patient = *gene;
        }
            let route_time =  if !route.is_empty() {time + t_matrix[route[route.len() - 1] as usize][0]} else {0.0};
        if !route.is_empty()
        {
            total_travel_time += t_matrix[*route.last().unwrap() as usize][0];
            path_string.push_str(&String::from(format!("->D({})", route_time)));
        }
        path_string.push_str("\n");

        path_string.insert_str(0, &String::from(format!("{}\t", current_nurse_demand)));
        path_string.insert_str(0, &String::from(format!("{:.2}\t", route_time)));
        path_string.insert_str(0, &String::from(format!("Nurse {:0>2}\t", nurse)));

        string_builder.push_str(&path_string);

    }
    string_builder.push_str(&String::from(format!("\n------------------------------------------------------------------\n")));
    string_builder.push_str(&String::from(format!("Objective value (total travel time): {}", total_travel_time)));
    string_builder.insert_str(0, &String::from(format!("\n-------------------------------------------\n")));
    string_builder.insert_str(0, &String::from(format!("\nDepot return time: {}", t_data.depot.return_time)));
    string_builder.insert_str(0, &String::from(format!("Nurse capacity: {}",  t_data.capacity_nurse)));



    string_builder

}

pub fn push_forward_insertion(chromo: &Chromosome, t_data: &TrainData) -> Vec<Vec<u16>>
{
    let mut routes: Vec<Vec<u16>> = vec![vec![0u16; 2]; t_data.nbr_nurses as usize];

    let mut route_counter:usize = 0;

    let travel_matrix = &t_data.travel_times;
    //TODO: fix demand
    let mut demand = 0;
    let mut chromo_iter = chromo.iter();
    let mut gene = chromo_iter.next().unwrap();
    loop
    {
        let mut lowest_cost: f64 = f64::MAX;
        let mut best_index: usize = usize::MAX;
        let mut route_demand: i32 = 0;

        let current_patient = &t_data.patients[&gene.to_string()];
        route_demand += current_patient.demand;

        let t = routes.len();
        if route_counter >= t - 1
        {
            routes[t - 1].push(*gene);
            gene = chromo_iter.next().unwrap_or(&u16::MAX);
            if *gene == u16::MAX
            {
                break
            }
            continue;
        }
        if routes[route_counter].len() == 2
        {
            // Add edge from depot to first patient
            routes[route_counter].insert(1, *gene);
            gene = chromo_iter.next().unwrap_or(&u16::MAX);
            if *gene == u16::MAX
            {
                break
            }
            continue;
        }

        let mut route_iter = routes[route_counter].iter();
        let mut origin = route_iter.next().unwrap();

        for (i, destination) in route_iter.enumerate()
        {
            let origin_end_time = if *origin == 0 {0.0f64} else {t_data.patients[&origin.to_string()].end_time};
            let destination_start_time = if *destination == 0 {t_data.depot.return_time} else {t_data.patients[&destination.to_string()].start_time};
            let patient_end = t_data.patients[&gene.to_string()].end_time;
            let patient_start =t_data.patients[&gene.to_string()].start_time;
            let travel_origin_patient = travel_matrix[*origin as usize][*gene as usize];
            let travel_patient_destination = travel_matrix[*gene as usize][*destination as usize];

            if !(origin_end_time + travel_origin_patient < patient_start && patient_end + travel_patient_destination < destination_start_time)
            {
                origin = destination;
                continue;
            }

            let t_time: f64 = travel_matrix[*gene as usize][*destination as usize];

            if OrderedFloat(t_time) <= OrderedFloat(lowest_cost)
            {
                lowest_cost = t_time;
                best_index = i;
            }

            origin = destination;
        }

        // If gene fits into route, insert it,
        // else current route full, go to next
        if best_index < usize::MAX
        {
            routes[route_counter].insert(best_index + 1, *gene);
            gene = chromo_iter.next().unwrap_or(&u16::MAX);
            if *gene == u16::MAX
            {
                break
            }
        } else {
            route_counter += 1;
        }
    }

    let t = routes.len();
    for (i, route) in routes.iter_mut().enumerate()
    {
        if !(i == t - 1)
        {
            route.remove(0);
            route.pop();
        } else {
            route.retain(|&x| x != 0);
        }
    }
    routes
}