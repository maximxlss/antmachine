mod vector;
pub use vector::Vector;

/// Functions to generate waves from some continuously rising value
pub mod gen {
    /// Generate a saw wave with given phase, length and amplitude value at state.
    /// If abs is true, output ranges from 0 to amplitude.
    /// If abs is false, output ranges from -amplitude/2 to amplitude/2
    pub fn saw(state: f32, phase: f32, length: f32, amplitude: f32, abs: bool) -> f32 {
        let mut state = state;
        state += phase;
        state %= length;
        state /= length;
        state = f32::abs(state - 0.5) * 2.;
        if abs {
            state *= amplitude;
        } else {
            state -= 0.5;
            state *= amplitude / 2.
        }
        state
    }
}

pub mod ants {
    use core::f64;
    use core::ops::Add;
    use std::f64::consts::PI;
    //use std::sync::{Arc, Mutex};
    use rayon::prelude::*;
    use super::vector::{Vector, angle_diff};
    use parking_lot::RwLock;

    static MOVE: f64 = 0.0333;

    #[derive(Clone, Copy)]
    pub struct Ant {
        pub pos: Vector,
        pub dir: Vector
    }

    impl Default for Ant {
        fn default() -> Self {
            Ant {
                pos: Vector {x: rand::random(), y: rand::random()},
                dir: Vector::from_angle((rand::random::<f64>() - 0.4) * 2. * PI)
            }
        }
    }

    impl Ant {
        pub fn new() -> Ant{
            Ant::default()
        }

        pub fn evolve(&mut self, pheromones: &[Pheromone]) {
            let self_angle = self.dir.angle();
            if self_angle > PI {
                self.dir = self.dir.rotated(-2. * PI)
            } else if self_angle < PI {
                self.dir = self.dir.rotated(2. * PI)
            }

            let (mean_angle, mut total_weight)= { // weighted mean
                let mut mean = Vector::new();
                let mut total_weight = 0.;
                for p in pheromones.iter() {
                    if p.pos == self.pos {
                        continue;
                    }
                    let to_p = p.pos - self.pos;
                    let mut angle_diff = angle_diff(self_angle, to_p.angle()).abs();
                    if angle_diff == 0. {
                        angle_diff = 1.
                    }
                    let weight = p.pow * ((1. / angle_diff) * 2.);
                    total_weight += weight * to_p.length();
                    mean = mean + to_p.mul_by_float(weight);
                }
                (mean.angle(), total_weight)
            };

            if total_weight == 0. {
                total_weight = 1.
            }

            if mean_angle != 0. {
                self.dir = self.dir.rotated(angle_diff(self_angle, mean_angle) / 40.);
            }
            let noise: f64 = (rand::random::<f64>() - 0.5)/4.;
            self.dir = self.dir.rotated(noise);
            self.pos = self.pos + self.dir.mul_by_float(MOVE/(total_weight.sqrt()/10.));
            if self.pos.x > 1. || self.pos.x < 0. || self.pos.y > 1. || self.pos.y < 0. {
                self.dir = self.dir.rotated(PI);
                self.pos = self.pos + self.dir.mul_by_float(MOVE/(total_weight.sqrt()/10.));
            }
        }
    }

    #[derive(Clone, Copy, Default)]
    pub struct Pheromone {
        pub pos: crate::Vector,
        pub pow: f64
    }

    impl Add for Pheromone {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            Self {
                pos: self.pos + other.pos,
                pow: self.pow + other.pow
            }
        }
    }

    impl Pheromone {
        pub fn new() -> Pheromone{
            Pheromone::default()
        }

        pub fn evolve(&mut self) {
            self.pow -= 0.1
        }
    }

    #[derive(Clone, Default)]
    pub struct World {
        pub ants: Vec<Ant>,
        pub pheromones: Vec<Pheromone>
    }

    impl World {
        pub fn new(num_ants: usize) -> World {
            World {
                ants: {
                    let mut vec = Vec::with_capacity(num_ants);
                    for _ in 0..num_ants {
                        vec.push(Ant::default());
                    }
                    vec
                },
                pheromones: Vec::new()
            }
        }

        pub fn evolve(&mut self) {
            for p in &mut self.pheromones {
                p.evolve()
            }
            self.pheromones.retain(|x| x.pow > 0.);
            for a in &mut self.ants {
                self.pheromones.push(Pheromone {pos: a.pos, pow: 1.});
                a.evolve(&self.pheromones)
            }
        }

        pub fn evolve_threaded(&mut self, threads: usize) {
            let mut ants_per_thread = self.ants.len() / threads;
            let mut pheromones_per_thread = self.pheromones.len() / threads;
            if pheromones_per_thread == 0 {
                pheromones_per_thread = 1
            }
            if ants_per_thread == 0 {
                ants_per_thread = 1
            }
            self.pheromones.as_mut_slice().par_chunks_mut(pheromones_per_thread).for_each(|pheromones| {
                for p in pheromones {
                    p.evolve();
                }
            });
            self.pheromones.retain(|x| x.pow > 0.);
            let pheromones = RwLock::new(&mut self.pheromones);
            self.ants.as_mut_slice().par_chunks_mut(ants_per_thread).for_each(|ants| {
                for a in ants {
                    pheromones.write().push(Pheromone {pos: a.pos, pow: 1.});
                    let pheromones = pheromones.read();
                    a.evolve(&pheromones);
                }
            });
        }
    }
}
