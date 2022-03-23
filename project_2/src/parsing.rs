use std::collections::HashMap;
use std::fs;
use serde::{Deserialize};


#[derive(Debug, Clone, Deserialize)]
pub struct TrainData {
    pub instance_name: String,
    pub nbr_nurses: i32,
    pub capacity_nurse: i32,
    pub benchmark: f32,
    pub depot: Depot,
    pub patients: HashMap<String, Patient>,
    pub travel_times: Vec<Vec<f64>>

}
#[derive(Debug, Clone, Deserialize)]
pub struct Depot {
    pub return_time: f64,
    x_coord: i32,
    y_coord: i32
}
#[derive(Debug, Clone, Deserialize)]
pub struct Patient {
    pub care_time: f64,
    pub(crate) demand: i32,
    pub end_time: f64,
    pub start_time: f64,
    pub x_coord: i32,
    pub y_coord: i32,
}
//TODO bruke bedre datatyper enn i32/f32 på alt


pub fn parse_json(filepath: &str) -> TrainData {
    let data = fs::read_to_string(filepath).expect("Unable to read file");
    let parsed: TrainData = serde_json::from_str(&data).expect("Unable to parse file");
    parsed
}

pub fn write_solution_to_file(solution: &String) {
    fs::write("solution.txt", solution).expect("Error writing to file")
}

pub fn pretty_print_solution_to_file(solution: &String) {
    fs::write("solution_detail.txt", solution).expect("Error writing to file")
}