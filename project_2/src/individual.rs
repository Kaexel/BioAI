pub(crate) mod individual {
    use crate::helper;
    use crate::individual::chromosome::Chromosome;
    use crate::parsing::TrainData;


    pub trait Individual{
        fn create(chromosome: Chromosome, fitness: f64, feasible: bool) -> Self;
        fn chromosome(&self) -> &Chromosome;
        fn fitness(&self) -> f64;
        fn feasible(&self) -> bool;
    }

    #[derive(Clone, Debug)]
    pub struct Route {
        fitness: f64,
        chromosome: Chromosome,
        feasible: bool
        //parents: Option<Chromosome>
    }
    impl Route {
        pub fn new(chromosome: Chromosome, t_data: &TrainData) -> Self
        {
            let (fitness, feasible) = calculate_fitness(&chromosome, t_data);
            Self { fitness, chromosome, feasible: !(feasible > 0.0) }
        }
    }

    impl Individual for Route {
        fn create(chromosome: Chromosome, fitness: f64, feasible: bool) -> Self {
            Self {
                fitness,
                chromosome,
                feasible,
            }
        }

        fn chromosome(&self) -> &Chromosome {
            &self.chromosome
        }

        fn fitness(&self) -> f64 {
            self.fitness
        }

        fn feasible(&self) -> bool {
            self.feasible
        }
    }




    ///Returns a tuple containing (total travel time, adjusted_fitness)
    ///total travel time only calculates time traveled
    ///adjusted fitness includes penalty for constraint breaks
    pub(crate) fn calculate_fitness(chromosome: &Chromosome, t_data: &TrainData) -> (f64, f64) {

        if chromosome.genes.contains(&0u16)
        {
            calculate_fitness_delims(chromosome, t_data)
        }
        else {
            calculate_fitness_no_delims(chromosome, t_data)
        }

    }

    fn calculate_fitness_delims(chromosome: &Chromosome, t_data: &TrainData) -> (f64, f64) {
        let mut start: usize = 0;
        let mut cumulative_breaks: i32 = 0;
        let mut cumulative_travel_time: f64 = 0.0;
        let travel_matrix: &Vec<Vec<f64>> = &t_data.travel_times;

        let mut penalty: f64 = 0.0;

        let nurse_capacity = t_data.capacity_nurse;
        let depot_return_time = t_data.depot.return_time;

        // Reset for each nurse (every 0 in chromosome)
        let mut nurse_load: i32 = 0;
        let mut current_time: f64 = 0.0;

        for gene in chromosome.iter()
        {

            // Recalculate time based on travel
            let travel_time = travel_matrix[start][*gene as usize];

            current_time += travel_time;
            cumulative_travel_time += travel_time;

            // if current gene is 0, we start a new nurse route
            // reset time and nurse load, and move to first patient
            if *gene == 0 {
                if nurse_load > nurse_capacity  {
                    penalty += (nurse_load - nurse_capacity) as f64 * 0.4; //Multiplier?
                    cumulative_breaks += 1;
                }
                //Move nurse back to depot and check if back on time
                if current_time > depot_return_time {
                    penalty += (current_time - depot_return_time) * 0.4; //Multiplier?
                    cumulative_breaks += 1;
                }
                // Reset nurse variables
                nurse_load = 0;
                current_time = 0.0;
                start = 0;
                continue
            }
            // Fitness is sum of travel times
            let current_patient = &t_data.patients[&gene.to_string()];

            // Checks if nurse arrives before time window
            // If so, wait until start of time:_window
            if current_time < current_patient.start_time {
                //penalty += (current_patient.start_time - current_time) * 0.005;// * 15.0; //Multiplier?
                current_time = current_patient.start_time
            }
            // Care for patient, and add demand to nurse load
            current_time += current_patient.care_time;
            nurse_load += current_patient.demand;

            // If nurse is finished after end_time, add a constraint break
            if current_time > current_patient.end_time {
                penalty += (current_time - current_patient.end_time) * 0.4 ;//  * 15.0; //Multiplier?
                cumulative_breaks += 1
            }

            // Move to next patient
            start = *gene as usize;
        }


        //Add return to depot for last nurse
        cumulative_travel_time += travel_matrix[start][0];


        if nurse_load > nurse_capacity  {
            penalty += (nurse_load - nurse_capacity) as f64 * 0.15; //Multiplier?
            cumulative_breaks += 1;
        }
        //Move nurse back to depot and check if back on time
        if current_time >= depot_return_time {
            penalty += current_time - depot_return_time * 0.4; //Multiplier?
            cumulative_breaks += 1;
        }
        //(cumulative_travel_time + 13.0f32 * cumulative_breaks as f32, cumulative_breaks as f32)
        //(cumulative_breaks as f32, cumulative_breaks as f32)
        //(cumulative_travel_time + penalty  as f32, cumulative_breaks as f32)
        (cumulative_travel_time * ( 1.0 + 0.3*cumulative_breaks as f64)  as f64, cumulative_breaks as f64)
        //(cumulative_travel_time + penalty * 20.0 as f64, cumulative_breaks as f64)
    }
    fn calculate_fitness_no_delims(chromosome: &Chromosome, t_data: &TrainData) -> (f64, f64) {

        let nurses = helper::push_forward_insertion(chromosome, t_data);

        let mut start: usize = 0;
        let mut cumulative_breaks: i32 = 0;
        let mut cumulative_travel_time: f64 = 0.0;
        let travel_matrix: &Vec<Vec<f64>> = &t_data.travel_times;

        let mut penalty: f64 = 0.0;

        let nurse_capacity = t_data.capacity_nurse;
        let depot_return_time = t_data.depot.return_time;

        // Reset for each nurse (every 0 in chromosome)
        let mut nurse_load: i32 = 0;
        let mut current_time: f64 = 0.0;

        for route in nurses.iter()
        {
            for gene in route.iter()
            {
                // Recalculate time based on travel
                let travel_time = travel_matrix[start][*gene as usize];

                current_time += travel_time;
                cumulative_travel_time += travel_time;

                // Fitness is sum of travel times
                let current_patient = &t_data.patients[&gene.to_string()];

                // Checks if nurse arrives before time window
                // If so, wait until start of time:_window
                if current_time < current_patient.start_time {
                    //penalty += (current_patient.start_time - current_time) * 0.005;// * 15.0; //Multiplier?
                    current_time = current_patient.start_time
                }
                // Care for patient, and add demand to nurse load
                current_time += current_patient.care_time;
                nurse_load += current_patient.demand;

                // If nurse is finished after end_time, add a constraint break
                if current_time > current_patient.end_time {
                    penalty += (current_time - current_patient.end_time) * 0.3 ;//  * 15.0; //Multiplier?
                    cumulative_breaks += 1
                }

                // Move to next patient
                start = *gene as usize;
            }
            if nurse_load > nurse_capacity  {
                penalty += (nurse_load - nurse_capacity) as f64 * 0.3; //Multiplier?
                cumulative_breaks += 1;
            }
            //Move nurse back to depot and check if back on time
            if current_time > depot_return_time {
                penalty += (current_time - depot_return_time) * 0.3; //Multiplier?
                cumulative_breaks += 1;
            }
            // Reset nurse variables
            nurse_load = 0;
            current_time = 0.0;
            start = 0;
            //Add return to depot for last nurse
            cumulative_travel_time += travel_matrix[start][0];

        }

        (cumulative_travel_time + penalty as f64, cumulative_breaks as f64)
    }
}

pub(crate) mod chromosome {
    use std::iter::FromIterator;
    use std::ops::Index;

    #[derive(Clone, Debug)]
    pub struct Chromosome {
        pub(crate) genes: Vec<u16>
    }

    impl Chromosome {
        pub fn len(&self) -> usize {
            self.genes.len()
        }
        pub fn iter(&self) -> impl Iterator<Item=&u16> {
            self.genes.iter()
        }
        pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut u16> {
            self.genes.iter_mut()
        }
        pub fn format_chromosome(&self) -> String {
            let mut out = String::from("[[");
            for gene in self.iter() {
                if *gene == 0 {
                    if out.ends_with(',') {
                        out.pop();
                    }
                    out.push_str("],[");
                    //out.push(']');
                } else {
                    out += &gene.to_string();
                    out.push(',');
                }
            }
            if out.ends_with(',') || out.ends_with(']'){
                out.pop();
            }
            out.push_str("]]");
            out
        }
    }

    impl Index<usize> for Chromosome {
        type Output = u16;

        fn index(&self, index: usize) -> &Self::Output {
            &self.genes[index]
        }
    }

    impl FromIterator<u16> for Chromosome {
        fn from_iter<T: IntoIterator<Item=u16>>(iter: T) -> Self {
            Self { genes: iter.into_iter().collect() }
        }
    }

    impl IntoIterator for Chromosome {
        type Item = u16;
        type IntoIter = std::vec::IntoIter<u16>;
        //type IntoIter = impl Iterator<Item=u16>;

        fn into_iter(self) -> Self::IntoIter {
            self.genes.into_iter()
        }
    }
}
