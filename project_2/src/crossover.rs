use std::cmp::{max, min};
use rand::{Rng, RngCore};
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use crate::individual::chromosome::Chromosome;
use crate::parsing::TrainData;


pub trait Crossover {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome, t_data: &TrainData) -> Chromosome;
}


pub struct OrderOneCrossover;

impl OrderOneCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl Crossover for OrderOneCrossover {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome, t_data: &TrainData) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        let between = Uniform::from(0..parent_a.len());
        let nurse_count = parent_a.iter().filter(|&n| *n == 0).count();

        let t = between.sample(rng);
        let t2 = between.sample(rng);
        let mut chromo = vec![u16::MAX; parent_a.len()];
        let range = min(t, t2) as usize..max(t, t2) as usize;

        let mut parent_b_slice= vec![0; range.len()];
        parent_b_slice.clone_from_slice(&parent_b.genes[range.start..range.end]);

        chromo.splice(range.start..range.end, parent_b.genes[range.start..range.end].iter().cloned());

        let v = chromo.iter().filter(|&n| *n == 0).count(); // Checking how many zeroes. Should be n_nurses - 1.
        let mut remaining_nurses = nurse_count - v;
        let parent_a_iterator_start = parent_a.genes[0..range.end].iter();
        let parent_a_iterator_end = parent_a.genes[range.end..].iter();

        let parent_iterator = parent_a_iterator_end.chain(parent_a_iterator_start);
        let mut counter = range.end;
        //println!("{:?}", range.len());
        //println!("{:?}", range.start);
        //println!("{:?}", range.end);

        for v in parent_iterator {
            if *v == 0 && remaining_nurses > 0 {
                chromo[counter] = *v;
                remaining_nurses -= 1;
                counter = (counter + 1) % parent_a.len();
                continue;
            }
            if !parent_b_slice.contains(v) {
                chromo[counter] = *v;
                counter = (counter + 1) % parent_a.len();
            }

        }
        let t:Chromosome = chromo.into_iter().collect();

        //println!("{:?}", &parent_a.genes[range.end..]);
        //println!("{:?}", &t);
        //println!("{:?}", &parent_b.genes[range.start..range.end]);

        t
    }
}

pub struct OrderOneCrossoverNoDelim;

impl OrderOneCrossoverNoDelim {
    pub fn new() -> Self {
        Self
    }
}
//TODO
impl Crossover for OrderOneCrossoverNoDelim {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome, t_data:&TrainData) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        let between = Uniform::from(0..parent_a.len());
        let t = between.sample(rng);
        let t2 = between.sample(rng);
        let range = min(t, t2) as usize..max(t, t2) as usize;

        let mut chromo = vec![u16::MAX; parent_a.len()];
        let mut parent_b_slice= vec![0; range.len()];
        parent_b_slice.clone_from_slice(&parent_b.genes[range.start..range.end]);

        chromo.splice(range.start..range.end, parent_b.genes[range.start..range.end].iter().cloned());

        let parent_a_iterator_start = parent_a.genes[0..range.end].iter();
        let parent_a_iterator_end = parent_a.genes[range.end..].iter();

        let parent_iterator = parent_a_iterator_end.chain(parent_a_iterator_start);
        let mut counter = range.end;

        for v in parent_iterator {

            if !parent_b_slice.contains(v) {
                chromo[counter] = *v;
                counter = (counter + 1) % parent_a.len();
            }

        }
        chromo.into_iter().collect()
    }
}

pub struct DPXCrossover;

impl DPXCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl Crossover for DPXCrossover {
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome, t_data: &TrainData) -> Chromosome {
        todo!()
    }
}


///Heuristic and merge xover are intended to be used with a representation w/o delims
pub struct HeuristicCrossover;
impl HeuristicCrossover
{
    pub fn new() -> Self {
        Self
    }
}
impl Crossover for HeuristicCrossover{
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome, t_data: &TrainData) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        let mut child: Vec<u16> = Vec::new();
        let mut p1 = parent_a.clone();
        let mut p2 = parent_b.clone();
        let cut_point:usize  = rng.gen_range(0..parent_a.len());
        let (slice_1, slice_2) = if cut_point > (p1.len() / 2) as usize {(cut_point..p1.len() - 1, 0..cut_point)} else {(0..cut_point, cut_point..p1.len() - 1)};
        child.push(p1[slice_1.start]);

        let pos = p2.iter().position(|&a| a == p1[slice_1.start]).unwrap();
        p2.genes.swap(pos, slice_1.start);

        for i in slice_1.start..slice_1.end
        {
            let dist_1: f64 = t_data.travel_times[p1[i] as usize][p1[i+1] as usize];
            let dist_2 = t_data.travel_times[p2[i]  as usize][p2[i+1] as usize];


            if dist_1 > dist_2
            {
                child.push(p2[i+1]);
                let pos = p1.iter().position(|&a| a == p2[i+1]).unwrap();
                p1.genes.swap(pos, i+1);

            } else {
                child.push(p1[i+1]);
                let pos = p2.iter().position(|&a| a == p1[i+1]).unwrap();
                p2.genes.swap(pos, i+1);
            }
        }
        for j in slice_2.start..slice_2.end
        {
            let dist_1: f64 = t_data.travel_times[p1[j] as usize][p1[j+1] as usize];
            let dist_2 = t_data.travel_times[p2[j]  as usize][p2[j+1] as usize];

            if dist_1 > dist_2
            {
                child.push(p2[j+1]);
                let pos = p1.iter().position(|&a| a == p2[j+1]).unwrap();
                p1.genes.swap(pos, j+1);

            } else {
                child.push(p1[j+1]);
                let pos = p2.iter().position(|&a| a == p1[j+1]).unwrap();
                p2.genes.swap(pos, j+1);
            }
        }

        child.into_iter().collect()
    }
}

pub struct MergeCrossover;
impl MergeCrossover
{
    pub fn new() -> Self {Self}
}
impl Crossover for MergeCrossover{
    fn crossover(&self, rng: &mut dyn RngCore, parent_a: &Chromosome, parent_b: &Chromosome, t_data: &TrainData) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        let mut child: Vec<u16> = Vec::new();
        let mut p1 = parent_a.clone();
        let mut p2 = parent_b.clone();
        let cut_point:usize  = rng.gen_range(0..parent_a.len());
        let (slice_1, slice_2) = if cut_point > (p1.len() / 2) as usize {(cut_point..p1.len() - 1, 0..cut_point)} else {(0..cut_point, cut_point..p1.len() - 1)};
        child.push(p1[slice_1.start]);

        let pos = p2.iter().position(|&a| a == p1[slice_1.start]).unwrap();
        p2.genes.swap(pos, slice_1.start);

        for i in slice_1.start..slice_1.end
        {
            let dist_1: f64 = t_data.patients[&p1[i].to_string()].end_time - t_data.patients[&p1[i].to_string()].care_time;
            let dist_2: f64 = t_data.patients[&p2[i].to_string()].end_time - t_data.patients[&p2[i].to_string()].care_time;



            if dist_1 > dist_2
            {
                child.push(p2[i+1]);
                let pos = p1.iter().position(|&a| a == p2[i+1]).unwrap();
                p1.genes.swap(pos, i+1);

            } else {
                child.push(p1[i+1]);
                let pos = p2.iter().position(|&a| a == p1[i+1]).unwrap();
                p2.genes.swap(pos, i+1);
            }
        }
        for j in slice_2.start..slice_2.end
        {
            let dist_1: f64 = t_data.travel_times[p1[j] as usize][p1[j+1] as usize];
            let dist_2 = t_data.travel_times[p2[j]  as usize][p2[j+1] as usize];

            if dist_1 > dist_2
            {
                child.push(p2[j+1]);
                let pos = p1.iter().position(|&a| a == p2[j+1]).unwrap();
                p1.genes.swap(pos, j+1);

            } else {
                child.push(p1[j+1]);
                let pos = p2.iter().position(|&a| a == p1[j+1]).unwrap();
                p2.genes.swap(pos, j+1);
            }
        }


        child.into_iter().collect()
    }
}