use std::error::Error;
use std::f64::consts;

use mpi::traits::*;
use mpi::Threading;
use rand::prelude::*;
use rand_distr::{Exp1, StandardNormal};
use rand_xoshiro::*;

mod constants;
mod field;
mod geometry;
mod particle;
mod nonlinear_compton;
mod special_functions;
mod output;

use constants::*;
use field::*;
use geometry::*;
use particle::*;
use output::*;

fn collide<F: Field, R: Rng>(field: &F, incident: Particle, rng: &mut R) -> Shower {
    let mut primary = incident;
    let mut secondaries: Vec<Particle> = Vec::new();
    let dt = field.max_timestep().unwrap_or(1.0);

    while field.contains(primary.position()) {
        let (r, u) = field.push(
            primary.position(), 
            primary.normalized_momentum(),
            primary.charge_to_mass_ratio(),
            dt
        );
        
        if let Some(k) = field.radiate(r, u, dt, rng) {
            let photon = Particle::create(Species::Photon, r)
                .with_normalized_momentum(k);
            secondaries.push(photon);
        }

        primary.with_position(r);
        primary.with_normalized_momentum(u);
    }

    Shower {
        primary,
        secondaries,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (universe, _) = mpi::initialize_with_threading(Threading::Funneled).unwrap();
    let world = universe.world();
    let id = world.rank();
    let numtasks = world.size();

    let a0 = 100.0;
    let wavelength = 0.8e-6;
    let waist = 4.0e-6;
    let duration = 30.0e-15;
    let pol = Polarization::Linear;
    let num: i32 = 100_000;
    let gamma = 1000.0;
    let sigma = 1.0;
    let radius = 1.0e-6;
    let focusing = true;

    let ospec = "angle_x:angle_y,p^-:p_perp";
    let ospec: Vec<DistributionFunction> = ospec
        .split(',')
        .map(|s| s.parse::<DistributionFunction>().unwrap())
        .collect();

    let mut rng = Xoshiro256StarStar::seed_from_u64(id as u64);

    let primaries: Vec<Particle> = (0..num).into_iter()
        .map(|_i| {
            let z = 2.0 * SPEED_OF_LIGHT * duration;
            let x = radius * rng.sample::<f64,_>(StandardNormal);
            let y = radius * rng.sample::<f64,_>(StandardNormal);
            let r = FourVector::new(-z, x, y, z);
            let u = -(gamma * gamma - 1.0f64).sqrt();
            let u = u + rng.sample::<f64,_>(StandardNormal);
            let u = FourVector::new(0.0, 0.0, 0.0, u).unitize();
            Particle::create(Species::Electron, r)
                .with_normalized_momentum(u)
                .with_optical_depth(rng.sample(Exp1))
        })
        .collect();

    let merge = |(mut p, mut s): (Vec<Particle>, Vec<Particle>), mut sh: Shower| {
        p.push(sh.primary);
        s.append(&mut sh.secondaries);
        (p, s)
    };

    let electrons: Vec<Particle> = Vec::new();
    let photons: Vec<Particle> = Vec::new();

    let (electrons, photons) = if focusing {
        let laser = FocusedLaser::new(a0, wavelength, waist, duration, pol);
        primaries
            .iter()
            .map(|pt| collide(&laser, *pt, &mut rng))
            .fold((electrons, photons), merge)
    } else {
        let laser = PlaneWave::new(a0, wavelength, 4.0, pol);
        primaries
            .iter()
            .map(|pt| collide(&laser, *pt, &mut rng))
            .fold((electrons, photons), merge)
    };

    println!("e.len = {}, ph.len = {}", electrons.len(), photons.len());

    for dstr in &ospec {
        dstr.write(&world, &electrons, "output/electron")?;
        dstr.write(&world, &photons, "output/photon")?;
    }

    Ok(())
}
