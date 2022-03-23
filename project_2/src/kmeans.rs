use rand::{RngCore};
use rand::prelude::IteratorRandom;
//TODO: gjør ferdig dette axel
///Unfinished kMeans implementation for clustering

fn fast_euclid(a_0: &u16, b_0: & u16, a_1: &u16, b_1: &u16) -> f32 {
    ((a_0-b_0).pow(2) as f32 + (a_1-b_1).pow(2) as f32).sqrt()
}

pub fn point_mean(points: &Vec<(u16, u16)>) -> (f32, f32) {
    let x_sum: f32 = points.iter().map(|(a, b)| a).sum::<u16>() as f32 / points.len() as f32;
    let y_sum: f32 = points.iter().map(|(a, b)| b).sum::<u16>() as f32 / points.len() as f32;

    (x_sum, y_sum)
}


pub fn kmeans(points: &[(u16, u16)], k: usize, rng: &mut dyn RngCore) -> Vec<usize>
{
    //TODO normalize points
    let mut centroids = points.iter().choose_multiple(rng, k);
    let mut cluster_assignments = vec![0; points.len()];

    loop {
        for (i, (x, y)) in points.iter().enumerate() {
            let mut init_dist = f32::MAX;
            for (j,(c_x, c_y)) in centroids.iter().enumerate() {
                let dist = fast_euclid(c_x, x, c_y, y);
                if dist < init_dist
                {
                    init_dist = dist;
                    cluster_assignments[i] = j;
                }
            }
        }

        let mut new_cent = vec![(0f32, 0f32); k];
        for cent in 0..centroids.len() {
            let mut q: Vec<(u16, u16)> = vec![];
            for c in cluster_assignments.iter() {
                if *c == cent {
                    q.push(points[*c])
                }
            }
            new_cent.push(point_mean(&q))

        }
        /*
        if new_cent == centroids {
            break
        } else {
            todo!()
            //centroids = new_cent;
        }
        */

    }

    cluster_assignments
}