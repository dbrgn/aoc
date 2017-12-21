//! Solve AoC 20 by using brute force simulation.
extern crate rayon;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader, Result as IoResult};
use std::process::exit;

use rayon::prelude::*;
use regex::Regex;


#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3 { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    pub fn manhattan_distance(&self) -> u64 {
        self.pos.x.abs() as u64 +
        self.pos.y.abs() as u64 +
        self.pos.z.abs() as u64
    }
}

struct Simulation {
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn len(&self) -> usize {
        self.particles.len()
    }

    pub fn step(&mut self) {
        self.particles.iter_mut()
                      .for_each(|p| {
            p.vel.x += p.acc.x;
            p.vel.y += p.acc.y;
            p.vel.z += p.acc.z;
            p.pos.x += p.vel.x;
            p.pos.y += p.vel.y;
            p.pos.z += p.vel.z;
        });
    }

    /// Return the index of the particle closest to the origin point.
    pub fn get_closest(&self) -> (usize, Particle) {
        let (i, p) = self.particles
            .iter()
            .enumerate()
            .min_by(|&(_, first), &(_, second)| {
                first.manhattan_distance().cmp(&second.manhattan_distance())
            })
            .unwrap();
        (i, p.to_owned())
    }
}

/// Parse the input file, return a `Simulation` instance.
fn get_data() -> IoResult<Simulation> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let coord_re = r"<(-?\d+),(-?\d+),(-?\d+)>";
    let re = Regex::new(&format!(r"p={0}, v={0}, a={0}", coord_re))
        .expect("Could not compile regex");

    let mut particles = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line?;
        let caps = re.captures(&line).unwrap();
        let pos = Vec3::new(
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse().unwrap(),
            caps.get(3).unwrap().as_str().parse().unwrap(),
        );
        let vel = Vec3::new(
            caps.get(4).unwrap().as_str().parse().unwrap(),
            caps.get(5).unwrap().as_str().parse().unwrap(),
            caps.get(6).unwrap().as_str().parse().unwrap(),
        );
        let acc = Vec3::new(
            caps.get(7).unwrap().as_str().parse().unwrap(),
            caps.get(8).unwrap().as_str().parse().unwrap(),
            caps.get(9).unwrap().as_str().parse().unwrap(),
        );
        particles.push(Particle { pos, vel, acc });
    }

    Ok(Simulation { particles })
}

fn main() {
    println!("=== AoC 20 ===\n");

    let mut simulation = get_data().expect("Could not initialize simulation");
    println!("Loaded {} particles.\n", simulation.len());

    let mut closest = simulation.get_closest();
    println!("Closest particle: {:?}", closest);

    let mut i = 0;
    let mut unchanged_since = 0;
    loop {
        if unchanged_since > 1_000_000 {
            println!("Closest particle hasn't changed since 1'000'000 simulation steps!");
            println!("Solution: Particle {}", closest.0);
            exit(0);
        }
        simulation.step();
        i += 1;
        let new_closest = simulation.get_closest();
        if new_closest.0 != closest.0 {
            println!("Closest particle changed after {} steps: {} (distance {})",
                i,
                new_closest.0,
                new_closest.1.manhattan_distance(),
            );
            closest = new_closest;
            unchanged_since = 0;
        } else {
            unchanged_since += 1;
        }
    }
}
