use std::cmp::{max, min};
use std::ops::Index;
use rand::{Rng, RngCore};
use rand::distributions::Uniform;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use crate::helper;

use crate::individual::chromosome::Chromosome;

pub trait Mutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
    fn adjust_chance(&mut self, chance: f64);
}

mod local_search {
    use super::*;

    pub fn in_invert(rng: &mut dyn RngCore, child: &mut Chromosome)
    {
        let mut split = helper::split_into_nurses(&child);
        let nurse = Uniform::from(0..split.len()).sample(rng);
        let _ = &split[nurse].reverse();
        for (gene, cool_gene) in child.iter_mut().zip(helper::combine_into_chromo(&split).iter())
        {
            *gene = *cool_gene;
        }
    }

    pub fn in_scramble(rng: &mut dyn RngCore, child: &mut Chromosome)
    {
        let mut split = helper::split_into_nurses(&child);
        let nurse = Uniform::from(0..split.len()).sample(rng);
        let _ = &split[nurse].shuffle(rng);
        for (gene, cool_gene) in child.iter_mut().zip(helper::combine_into_chromo(&split).iter())
        {
            *gene = *cool_gene;
        }
    }

    pub(crate) fn in_swap(rng: &mut dyn RngCore, child: &mut Chromosome)
    {
        let mut split = helper::split_into_nurses(&child);
        let mut nurse = Uniform::from(0..split.len()).sample(rng);

        while split[nurse].is_empty()
        {
            nurse = (nurse+1) % split.len();
        }
        let gene_1 = rng.gen_range(0..split[nurse].len());
        let gene_2 = rng.gen_range(0..split[nurse].len());
        let _ = &split[nurse].swap(gene_1, gene_2);
        for (gene, cool_gene) in child.iter_mut().zip(helper::combine_into_chromo(&split).iter())
        {
            *gene = *cool_gene;
        }
    }

    pub fn in_insert(rng: &mut dyn RngCore, child: &mut Chromosome)
    {
        let mut split = helper::split_into_nurses(&child);
        let mut nurse = Uniform::from(0..split.len()).sample(rng);
        // TODO: fiks de dumme greiene her Axel
        while split[nurse].is_empty()
        {
            nurse = (nurse+1) % split.len();
        }
        let index = rng.gen_range(0..split[nurse].len());
        let gene = split[nurse].remove(index);

        let new_index = if split[nurse].is_empty() {0} else {rng.gen_range(0..split[nurse].len())};
        let _ = &split[nurse].insert(new_index, gene);
        for (gene, cool_gene) in child.iter_mut().zip(helper::combine_into_chromo(&split).iter())
        {
            *gene = *cool_gene;
        }
    }

    ///Need two routes that contain patients
    pub fn cross_swap(rng: &mut dyn RngCore, child: &mut Chromosome)
    {
        let mut split = helper::split_into_nurses(&child);
        let between =  Uniform::from(0..split.len());
        let mut nurse_1 = between.sample(rng);
        let mut nurse_2 = between.sample(rng);

        // TODO: fiks de dumme greiene her Axel
        while split[nurse_1].is_empty()
        {
            nurse_1 = (nurse_1+1) % split.len();
        }

        while split[nurse_2].is_empty() || (nurse_1 == nurse_2)
        {
            nurse_2 = (nurse_2+1) % split.len();
        }

        let nurse_1_index =  rng.gen_range(0..split[nurse_1].len());
        let nurse_2_index =  rng.gen_range(0..split[nurse_2].len());

        let nurse_1_gene = split[nurse_1].remove(nurse_1_index);
        let nurse_2_gene = split[nurse_2].remove(nurse_2_index);

        let _ = &split[nurse_2].insert(nurse_2_index, nurse_1_gene);
        let _ = &split[nurse_1].insert(nurse_1_index, nurse_2_gene);

        for (gene, cool_gene) in child.iter_mut().zip(helper::combine_into_chromo(&split).iter())
        {
            *gene = *cool_gene;
        }
    }

    //TODO
    pub fn cross_swap_subset(rng: &mut dyn RngCore, child: &mut Chromosome)
    {
        let mut split = helper::split_into_nurses(&child);
        let between =  Uniform::from(0..split.len());
        let mut nurse_1 = between.sample(rng);
        let mut nurse_2 = between.sample(rng);

        // TODO: fiks de dumme greiene her Axel
        while split[nurse_1].is_empty()
        {
            nurse_1 = (nurse_1+1) % split.len();
        }

        while split[nurse_2].is_empty() || (nurse_1 == nurse_2)
        {
            nurse_2 = (nurse_2+1) % split.len();
        }

        let nurse_1_index =  rng.gen_range(0..split[nurse_1].len());
        let nurse_2_index =  rng.gen_range(0..split[nurse_2].len());

        let nurse_1_gene = split[nurse_1].remove(nurse_1_index);
        //min(t, t2)..max(t, t2)
       // let t = split[nurse_1].drain()
        let nurse_2_gene = split[nurse_2].remove(nurse_2_index);

        let _ = &split[nurse_2].insert(nurse_2_index, nurse_1_gene);
        let _ = &split[nurse_1].insert(nurse_1_index, nurse_2_gene);

        for (gene, cool_gene) in child.iter_mut().zip(helper::combine_into_chromo(&split).iter())
        {
            *gene = *cool_gene;
        }
    }
    ///Need at least one route that contain patients
    pub fn cross_insert(rng: &mut dyn RngCore, child: &mut Chromosome)
    {
        let mut split = helper::split_into_nurses(&child);
        let mut gene: u16 = 0;

        {
            let mut t = rng.gen_range(0..split.len());

            //TODO: sikkert en MYE bedre måte å gjøre dette på
            while split[t].is_empty()
            {
                t = (t+1) % split.len();
            }
            let route_1 = &mut split[t];

            let old_index = rng.gen_range(0..route_1.len());
            gene = route_1.remove(old_index);
        }

        let route_2 = split.choose_mut(rng).unwrap();
        let new_index = if route_2.is_empty() {0} else {rng.gen_range(0..route_2.len())};
        let _ = route_2.insert(new_index, gene);

        for (gene, cool_gene) in child.iter_mut().zip(helper::combine_into_chromo(&split).iter())
        {
            *gene = *cool_gene;
        }
    }

}

pub(crate) mod in_route {
    use super::*;
    pub struct InRouteInversionMutation {
        chance: f64,
    }
    impl InRouteInversionMutation {
        pub fn new(chance: f64) -> Self {
            assert!(0.0 <= chance  && chance <= 1.0);
            Self {chance}
        }
    }
    impl Mutation for InRouteInversionMutation {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
            if rng.gen_bool(self.chance as _) {
                local_search::in_invert(rng, child);
            }
        }

        fn adjust_chance(&mut self, chance: f64) {
            self.chance = chance
        }
    }

    pub struct InRouteScrambleMutation {
        chance: f64,
    }
    impl InRouteScrambleMutation {
        pub fn new(chance: f64) -> Self {
            assert!(0.0 <= chance  && chance <= 1.0);
            Self {chance}
        }
    }
    impl Mutation for InRouteScrambleMutation {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
            if rng.gen_bool(self.chance as _) {
                local_search::in_scramble(rng, child);
            }
        }

        fn adjust_chance(&mut self, chance: f64) {
            self.chance = chance;
        }
    }

    pub struct InRouteSwapMutation {
        chance: f64,
    }
    impl InRouteSwapMutation {
        pub fn new(chance: f64) -> Self {
            assert!(0.0 <= chance  && chance <= 1.0);
            Self {chance}
        }
    }
    impl Mutation for InRouteSwapMutation {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
            if rng.gen_bool(self.chance as _) {
                local_search::in_swap(rng, child);
            }
        }

        fn adjust_chance(&mut self, chance: f64) {
            self.chance = chance;
        }
    }

    pub struct InRouteInsertMutation {
        chance: f64,
    }
    impl InRouteInsertMutation {
        pub fn new(chance: f64) -> Self {
            assert!(0.0 <= chance  && chance <= 1.0);
            Self {chance}
        }
    }
    impl Mutation for InRouteInsertMutation {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
            if rng.gen_bool(self.chance as _) {
                local_search::in_insert(rng, child);
            }
        }

        fn adjust_chance(&mut self, chance: f64) {
            self.chance = chance;
        }
    }

}

pub mod cross_route {
    use super::*;

    pub struct CrossRouteSwapMutation {
        chance: f64,
    }
    impl CrossRouteSwapMutation {
        pub fn new(chance: f64) -> Self {
            assert!(0.0 <= chance  && chance <= 1.0);
            Self {chance}
        }
    }
    impl Mutation for CrossRouteSwapMutation {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
            if rng.gen_bool(self.chance as _) {
                local_search::cross_swap(rng, child);
            }
        }

        fn adjust_chance(&mut self, chance: f64) {
            self.chance = chance;
        }
    }

    pub struct CrossRouteInsertMutation {
        chance: f64,
    }
    impl CrossRouteInsertMutation {
        pub fn new(chance: f64) -> Self {
            assert!(0.0 <= chance  && chance <= 1.0);
            Self {chance}
        }
    }
    impl Mutation for CrossRouteInsertMutation {
        fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
            if rng.gen_bool(self.chance as _) {
                local_search::cross_insert(rng, child);
            }
        }

        fn adjust_chance(&mut self, chance: f64) {
            self.chance = chance;
        }
    }

}

pub struct InversionMutation {
    chance: f64,
}
impl InversionMutation {
    pub fn new(chance: f64) -> Self {
        assert!(0.0 <= chance  && chance <= 1.0);
        Self {chance}
    }
}
impl Mutation for InversionMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {

        if rng.gen_bool(self.chance as _) {
            let between = Uniform::from(0..child.len());
            let t = between.sample(rng);
            let t2 = between.sample(rng);
            //println!("{:?}", &child.genes[min(t, t2)..max(t, t2)]);
             let _ = &child.genes[min(t, t2)..max(t, t2)].reverse();
            //println!("{:?}", &child.genes[min(t, t2)..max(t, t2)]);

        }
    }

    fn adjust_chance(&mut self, chance: f64) {
        self.chance = chance;
    }
}

pub struct ScrambleMutation {
    chance: f64,
}
impl ScrambleMutation {
    pub fn new(chance: f64) -> Self {
        assert!(0.0 <= chance  && chance <= 1.0);
        Self {chance}
    }
}
impl Mutation for ScrambleMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        let between = Uniform::from(0..child.len());

        if rng.gen_bool(self.chance as _) {

            let t = between.sample(rng);
            let t2 = between.sample(rng);

            //println!("{:?}", &child.genes[min(t, t2)..max(t, t2)]);
            let _ = &child.genes[min(t, t2)..max(t, t2)].shuffle(rng);
            //println!("{:?}", &child.genes[min(t, t2)..max(t, t2)]);

        }
    }

    fn adjust_chance(&mut self, chance: f64) {
        self.chance = chance;
    }
}

pub struct SwapMutation {
    chance: f64,
}
impl SwapMutation {
    pub fn new(chance: f64) -> Self {
        assert!(0.0 <= chance  && chance <= 1.0);
        Self {chance}
    }
}
impl Mutation for SwapMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        let mut pos1 = 0u16;
        for gene in child.iter_mut() {
            if rng.gen_bool(self.chance as _) {
                pos1 = *gene;
                todo!()
            }
        }
    }

    fn adjust_chance(&mut self, chance: f64) {
        self.chance = chance;
    }
}

pub struct MutationHolder {
    mutations: Vec<Box<dyn Mutation + 'static>>,
}

impl MutationHolder {
    pub(crate) fn new() -> Self {
        Self {mutations: Vec::new()}
    }
    pub fn register(&mut self, data: impl Mutation + 'static) {
        self.mutations.push(Box::new(data));
    }
    pub fn mutations(&self) -> &Vec<Box<dyn Mutation + 'static>>
    {
        &self.mutations
    }

    pub fn adjust_chances(&mut self, chance: f64)
    {
        for m in self.mutations.iter_mut()
        {
            m.adjust_chance(chance);
        }
    }
    pub fn len(&self) -> usize
    {
        self.mutations.len()
    }


}

impl Index<usize> for MutationHolder {
    type Output = Box<dyn Mutation + 'static>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mutations[index]
    }
}

