use crate::individual::chromosome::Chromosome;
use crate::parsing::TrainData;

pub trait FitnessFunction {
    fn calculate_fitness(&self, chromosome: &Chromosome) -> (f32, f32);
}

pub struct TrainDataFitness {
    t_data: TrainData,
}

impl TrainDataFitness {
    pub fn new(t_data: TrainData) -> Self
    {
        Self { t_data }
    }
}
/*
impl FitnessFunction for TrainDataFitness {
    fn calculate_fitness(&self, chromosome: &Chromosome) -> (f32, f32) {
        let mut start: usize = 0;
        let mut cumulative_breaks: i32 = 0;
        let mut cumulative_travel_time: f32 = 0.0f32;
        let travel_matrix: &Vec<Vec<f32>> = &t_data.travel_times;

        let nurse_capacity = t_data.capacity_nurse;
        let depot_return_time = t_data.depot.return_time;

        // Reset for each nurse (every 0 in chromosome)
        let mut nurse_load: i32 = 0;
        let mut current_time: f32 = 0.0f32;

        for gene in chromosome.iter()
        {

            /// Recalculate time based on travel
            let travel_time = travel_matrix[start][*gene as usize];
            current_time += travel_time;
            cumulative_travel_time += travel_time;

            /// if current gene is 0, we start a new nurse route
            /// reset time and nurse load, and move to first patient
            if *gene == 0 {
                ///Move nurse back to depot and check if back on time
                if current_time >= depot_return_time {
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
                cumulative_breaks += 1;
                current_time = current_patient.start_time
            }
            // Care for patient, and add demand to nurse load
            current_time += current_patient.care_time;
            nurse_load += current_patient.demand;

            // If nurse is finished after end_time, add a constraint break
            if current_time > current_patient.end_time {
                cumulative_breaks += 1
            }

            // Check if current nurse over capacity
            // currently adds one break for every patient attended while over capacity. Maybe change?
            if nurse_load > nurse_capacity {
                cumulative_breaks += 1;
            }

            // Move to next patient
            start = *gene as usize;
        }
        (cumulative_travel_time, cumulative_breaks as f32)
    }
}
*/

