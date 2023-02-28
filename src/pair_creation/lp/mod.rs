//! Nonlinear Breit-Wheeler pair creation in LP backgrounds,
//! including dependence on the polarization of the high-energy photon

use std::f64::consts;
#[cfg(test)]
use std::fmt;

use rand::prelude::*;
use num_complex::Complex64;
#[cfg(test)]
use rayon::prelude::*;

use crate::special_functions::*;
#[cfg(test)]
use crate::quadrature::*;

mod theta_bound;
use theta_bound::ThetaBound;

#[cfg(test)]
mod indefinite_integral;

mod tables;

/// Represents the double-differential partial rate, `d^2 W_n / (ds dθ)`,
/// at harmonic order n.
/// Multiply by `ɑ / η` to get `dP_n/(ds dθ dphase)`.
struct DoubleDiffPartialRate {
    n: i32,
    a: f64,
    eta: f64,
    dj: DoubleBessel,
    tb: ThetaBound,
}

impl DoubleDiffPartialRate {
    /// Constructs the double-differential pair-creation spectrum at the given
    /// harmonic order `n`.
    fn new(n: i32, a: f64, eta: f64) -> Self {
        let x_max = (n as f64) * consts::SQRT_2;
        let y_max = (n as f64) * 0.5;
        let dj = DoubleBessel::at_index(n, x_max, y_max);
        let tb = ThetaBound::for_harmonic(n, a, eta);
        Self { n, a, eta, dj, tb }
    }

    fn s_bounds(&self) -> (f64, f64) {
        let s_min = 0.5;
        let s_max = 0.5 + (0.25 - (1.0 + 0.5 * self.a * self.a) / (2.0 * (self.n as f64) * self.eta)).sqrt();
        let s_max = s_max.max(0.5);
        (s_min, s_max)
    }

    fn max_theta(&self, s: f64) -> f64 {
        self.tb.at(s)
    }

    /// Calculates the pair creation rate, differential in s (fractional lightfront momentum transfer)
    /// and theta (azimuthal angle), for a parallel and perpendicularly polarized photon,
    /// returning the results as a single complex number.
    fn at(&mut self, s: f64, theta: f64) -> Complex64 {
        let n = self.n;
        let a = self.a;
        let eta = self.eta;

        let r_n_sqd = 2.0 * (n as f64) * eta * s * (1.0 - s) - (1.0 + 0.5 * a * a);

        let x = if r_n_sqd > 0.0 {
            a * r_n_sqd.sqrt() * theta.cos() / (eta * s * (1.0 - s))
        } else {
            return Complex64 { re: 0.0, im: 0.0 };
        };

        let y = a * a / (8.0 * eta * s * (1.0 - s));

        let j = self.dj.evaluate(x, y); // n-2, n-1, n, n+1, n+2

        let gamma = [j[2], 0.5 * (j[1] + j[3]), 0.25 * (j[0] + 2.0 * j[2] + j[4])];

        let h_s = 0.5 / (s * (1.0 - s)) - 1.0;

        let unpol = gamma[0].powi(2) + a * a * h_s * (gamma[1].powi(2) - gamma[0] * gamma[2]);
        let delta = (gamma[0] * r_n_sqd.sqrt() - gamma[1] * a).powi(2) - (gamma[0] * theta.sin()).powi(2) * r_n_sqd;

        Complex64 {
            re: (unpol - delta) / (2.0 * consts::PI),
            im: (unpol + delta) / (2.0 * consts::PI),
        }
    }

    /// Pair creation rate, differential in s (fractional lightfront momentum transfer)
    /// and theta (azimuthal angle), for a partially polarized photon.
    fn component(&mut self, s: f64, theta: f64, sv1: f64) -> f64 {
        let tmp = self.at(s, theta);
        0.5 * ((1.0 + sv1) * tmp.re + (1.0 - sv1) * tmp.im)
    }

    #[cfg(test)]
    fn integrate_over_theta(&mut self, s: f64, max_theta: f64) -> Complex64 {
        GAUSS_32_NODES.iter()
            .map(|x| 0.5 * (x + 1.0) * max_theta)
            .zip(GAUSS_32_WEIGHTS.iter())
            .map(|(theta, w2)| {
                0.5 * w2 * max_theta * self.at(s, theta)
            })
            .sum()
    }

    #[cfg(test)]
    fn integrate_direct(&mut self) -> Complex64 {
        let n = self.n;
        let a = self.a;
        let eta = self.eta;

        let (s_min, s_max) = self.s_bounds();

        // approx s where rate is maximised
        let s_peak = (0.5 + (0.25 - (1.0 + a * a) / (2.0 * (n as f64) * eta)).sqrt()).min(0.5);

        // integrates from 0 < theta < pi/2 and 0.5 < s < s_max:
        let integral: Complex64 = if s_peak < s_min + 2.0 * (s_max - s_min) / 3.0 {
            let s_peak = s_peak.min(s_min + (s_max - s_min) / 3.0);

            // split domain into two
            let bounds = [
                (s_min, s_peak),
                (s_peak, s_max)
            ];

            bounds.iter()
                .map(|(s0, s1)| -> Complex64 {
                    GAUSS_32_NODES.iter()
                        .map(|x| s0 + 0.5 * (x + 1.0) * (s1 - s0))
                        .zip(GAUSS_32_WEIGHTS.iter())
                        .map(|(s, w)| {
                            let max_theta = self.max_theta(s);
                            let single_diff = self.integrate_over_theta(s, max_theta);
                            0.5 * w * (s1 - s0) * single_diff
                        })
                        .sum()
                })
                .sum()
        } else {
            // split domain into three: s_min to s_max-2d, s_max-2d to s_peak,
            // s_peak to s_max where d = s_max - s_peak
            let delta = s_max - s_peak;
            let bounds: [(f64, f64, &[f64], &[f64]); 3] = [
                (s_min,               s_max - 2.0 * delta, &GAUSS_16_NODES, &GAUSS_16_WEIGHTS),
                (s_max - 2.0 * delta, s_peak,              &GAUSS_32_NODES, &GAUSS_32_WEIGHTS),
                (s_peak,              s_max,               &GAUSS_32_NODES, &GAUSS_32_WEIGHTS),
            ];

            bounds.iter()
                .map(|(s0, s1, nodes, weights)| -> Complex64 {
                    nodes.iter()
                        .map(|x| s0 + 0.5 * (x + 1.0) * (s1 - s0))
                        .zip(weights.iter())
                        .map(|(s, w)| {
                            let max_theta = self.max_theta(s);
                            let single_diff = self.integrate_over_theta(s, max_theta);
                            0.5 * w * (s1 - s0) * single_diff
                        })
                        .sum()
                })
                .sum()
        };

        2.0 * 4.0 * integral
    }

    #[cfg(test)]
    fn integrate_adaptive(&mut self) -> (Complex64, i32) {
        let n = self.n;
        let a = self.a;
        let eta = self.eta;

        let (s_min, s_max) = self.s_bounds();
        // approx s where rate is maximised
        let s_peak = 0.5 + (0.25 - (1.0 + a * a) / (2.0 * (n as f64) * eta)).sqrt();
        let s_peak = if s_peak > s_min + 2.0 * (s_max - s_min) / 3.0 {
            Some(s_peak)
        } else {
            None
        };

        let theta_bound = self.tb.clone();
        let theta_split = Some(theta_bound.at(s_peak.unwrap_or(0.5)));

        let (integral, count) = adaptive::integrate_2d(
            |s, theta| self.at(s, theta),
            s_min, s_peak, s_max,
            0.0, theta_split, consts::FRAC_PI_2,
            |s, theta| theta < theta_bound.at(s),
            1.0e-3, 16,
        );

        (4.0 * 2.0 * integral, count)
    }

    /// Integrates the double-differential partial rate over the entire domain `0 < θ < 2π` and `s_min < s < s_max`,
    /// returning the result for the two polarization components as a single complex number.
    /// Multiply by `ɑ / η` to get `dP_n/dphase`.
    #[cfg(test)]
    fn integrate(&mut self) -> Complex64 {
        if self.max_theta(0.5) < 0.75 * consts::FRAC_PI_2 {
            let (integral, _) = self.integrate_adaptive();
            integral
        } else {
            self.integrate_direct()
        }
    }

    /// Returns the peak value of the double-diff spectrum, padded by a small safety margin.
    fn ceiling(&mut self, sv1: f64) -> f64 {
        let n = self.n;
        let a = self.a;
        let eta = self.eta;
        let (s_min, s_max) = self.s_bounds();

        let centre = 0.5 + (0.25 - (1.0 + a * a) / (2.0 * (n as f64) * eta)).sqrt();
        let (lower, upper) = if centre.is_finite() {
            ((centre - 0.2).max(s_min), 0.5 * (centre + s_max))
        } else {
            (s_min, s_max)
        };

        // println!("\tsearching in {:.4} to {:.4} for {}, {}, {}", lower, upper, a, eta, n);

        let max = (0..50)
            .map(|i| {
                let s = lower + (upper - lower) * (i as f64) / 50.0;
                // double-diff rate always maximised along theta = 0
                self.component(s, 0.0, sv1)
            })
            .reduce(f64::max)
            .unwrap();

        1.1 * max
    }

    /// Samples the double-differential spectrum, returning a pseudorandomly selected
    /// lightfront-momentum fraction `s` and polar angle `theta`, as well as the
    /// number of rejections.
    fn sample<R: Rng>(&mut self, sv1: f64, mut rng: R) -> (f64, f64, i32) {
        let (s_min, s_max) = self.s_bounds();

        let max = self.ceiling(sv1);
        let mut count = 0;

        // Rejection sampling
        let (s, theta) = loop {
            let s = s_min + (s_max - s_min) * rng.gen::<f64>();
            let theta = consts::FRAC_PI_2 * rng.gen::<f64>();
            if theta > self.max_theta(s) {
                continue;
            }
            let z = max * rng.gen::<f64>();
            let f = self.component(s, theta, sv1);
            count += 1;
            if z < f {
                break (s, theta);
            }
        };

        // Fix s, which is [1/2, s_max] at the moment
        let s = match rng.gen_range(0, 2) {
            0 => 1.0 - s,
            1 => s,
            _ => unreachable!(),
        };

        // Fix range of theta, which is [0, pi/2] at the moment
        let quadrant = rng.gen_range(0, 4);
        let theta = match quadrant {
            0 => theta,
            1 => consts::PI - theta,
            2 => consts::PI + theta,
            3 => 2.0 * consts::PI - theta,
            _ => unreachable!(),
        };

        (s, theta, count)
    }
}

/// Represents the cumulative sum over the partial rates
#[cfg(test)]
#[derive(Default)]
struct RateTable {
    cdf: [[f64; 3]; 16],
}

#[cfg(test)]
impl fmt::Display for RateTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for entry in self.cdf.iter().take(15) {
            write!(f, "[{:>18.12e}, {:>18.12e}, {:>18.12e}], ", entry[0], entry[1], entry[2])?;
        }
        write!(f, "[{:>18.12e}, {:>18.12e}, {:>18.12e}]]", self.cdf[15][0], self.cdf[15][1], self.cdf[15][2])
    }
}

#[cfg(test)]
impl RateTable {
    fn total(&self) -> (f64, f64) {
        (self.cdf[15][1], self.cdf[15][2])
    }
}

/// Represents the total pair-creation rate, i.e. [DoubleDiffPartialRate]
/// integrated over all s and θ, then summed over all n.
/// Multiply by `ɑ / η` to get `Σ_n dP_n/dphase`.
pub(super) struct TotalRate {
    a: f64,
    eta: f64,
}

impl TotalRate {
    pub(super) fn new(a: f64, eta: f64) -> Self {
        Self {
            a,
            eta,
        }
    }

    /// Returns the range of harmonics that contribute to the total rate.
    fn sum_limits(&self) -> (i32, i32) {
        let (a, eta) = (self.a, self.eta);
        let n_min = (2.0f64 * (1.0 + 0.5 * a * a) / eta).ceil();
        let range = if a < 1.0 {
            2.0 + (2.0 + 20.0 * a * a) * (-(0.5 * eta).sqrt()).exp()
        } else {
            // if a < 20
            3.0 + 2.8 * a.powf(8.0/3.0) / eta.cbrt() + 0.25 * a.powi(3) * eta.sqrt()
        };

        let test = 0.25 - (1.0 + 0.5 * a * a) / (2.0 * (n_min as f64) * eta);
        if test <= f64::EPSILON {
            ((n_min as i32) + 1, (n_min + 1.0 + range) as i32)
        } else {
            (n_min as i32, (n_min + range) as i32)
        }
    }

    /// Near enough the most probable harmonic order, assuming a > 5.
    // fn modal_n(&self) -> i32 {
    //     let delta = 0.2 * (1.0 - 0.7 * self.eta.log10()) * self.a.powf(1.5);
    //     let (n_min, _) = self.sum_limits();
    //     n_min + (delta as i32)
    // }

    /// Checks if a and eta are small enough such that the rate < exp(-200)
    fn is_negligible(&self) -> bool {
        self.eta.log10() < -1.0 - (self.a.log10() + 2.0).powi(2) / 4.5
    }

    /// Similar to [is_negligible], intended only for use during table generation
    #[cfg(test)]
    fn can_be_skipped(&self) -> bool {
        let eta = 1.2 * self.eta;
        eta.log10() < -1.0 - (self.a.log10() + 2.0).powi(2) / 4.5
    }

    /// Returns the sum, over harmonic index, of the partial nonlinear
    /// Breit-Wheeler rates. Implemented as a table lookup.
    pub(super) fn value(&self, sv1: f64) -> f64 {
        let a = self.a;
        let eta = self.eta;

        if self.is_negligible() {
            0.0
        } else if tables::mid_range::contains(a, eta) {
            tables::mid_range::interpolate(a, eta, sv1)
        } else if tables::contains(a, eta) {
            tables::interpolate(a, eta, sv1)
        } else {
           0.0
        }
    }

    /// Returns a pseudorandomly sampled n (harmonic order), s (lightfront momentum
    /// transfer) and theta (azimuthal angle in the ZMF) for a pair creatione event that
    /// occurs at normalized amplitude a and energy parameter eta.
    pub(super) fn sample<R: Rng>(&self, sv1: f64, rng: &mut R) -> (i32, f64, f64) {
        let a = self.a;
        let eta = self.eta;
        let frac = rng.gen::<f64>();

        let n = if tables::mid_range::contains(a, eta) {
            tables::mid_range::invert(a, eta, sv1, frac)
        } else if tables::contains(a, eta) {
            tables::invert(a, eta, sv1, frac)
        } else {
            panic!("out of bounds at {} {}", a, eta);
        };

        let mut spectrum = DoubleDiffPartialRate::new(n, a, eta);
        let (s, theta, _) = spectrum.sample(sv1, rng);

        (n, s, theta)
    }

    /// Returns the total rate of emission and the cumulative density function, obtained
    /// by summing the partial rates over the relevant range of harmonics.
    #[cfg(test)]
    fn by_summation(&self) -> RateTable {
        let a = self.a;
        let eta = self.eta;
        let (n_min, n_max) = self.sum_limits();

        // run this bit in parallel
        let mut rates: Vec<(i32, Complex64)> = (n_min..=n_max).into_par_iter()
            .map(|n| (n, DoubleDiffPartialRate::new(n, a, eta).integrate()))
            .collect();

        // cumulative sum
        let mut total = Complex64::new(0.0, 0.0);
        for (_, pr) in rates.iter_mut() {
            total = total + *pr;
            *pr = total;
        }

        // Fill nodes of CDF
        let mut cdf = {
            let delta = (n_max - n_min) as f64;
            let u_max = (1.0 + delta).ln();

            let mut cdf = [[0.0; 3]; 16];
            cdf[0][0] = n_min as f64;

            // Split the log-scaled interval uniformly
            let mut u = 0.0;
            let mut du = u_max / 15.0;
            for i in 1..=15 {
                u = u + du;
                let n = (n_min as f64) + u.exp_m1();
                let n = if n.round() <= cdf[i-1][0] {
                    // == means we're duplicating a point
                    // < means u > u_max, either way push n up
                    let n = cdf[i-1][0] + 1.0;
                    // reset u and du
                    u = (n - (n_min as f64) + 1.0).ln();
                    du = (u_max - u) / (15.0 - (i as f64));
                    n
                } else {
                    n.round()
                };
                cdf[i][0] = n;
            }

            cdf
        };

        // Populate the cumulative rate
        for [n, par, perp] in cdf.iter_mut() {
            let index = (*n as usize) - (n_min as usize);
            if index < rates.len() {
                let (k, cumsum) = rates[index];
                assert!(*n as i32 == k);
                *par = cumsum.re;
                *perp = cumsum.im;
            } else {
                let (_, cumsum) = rates.last().unwrap();
                *par = cumsum.re;
                *perp = cumsum.im;
            }
        }

        RateTable { cdf }
    }

    /// Returns the total rate of emission and the cumulative density function, obtained
    /// by integrating the partial rates as a function of n, over the relevant range of harmonics.
    #[cfg(test)]
    fn by_integration(&self) -> RateTable {
        use std::collections::HashMap;
        use indefinite_integral::IndefiniteIntegral;

        let a = self.a;
        let eta = self.eta;

        // the number of harmonics is > 150
        assert!(a >= 5.0);

        let (n_min, n_max) = self.sum_limits();
        let delta = (n_max - n_min) as f64;

        // Transform to integral over u = ln(n - n_min + 1) => dn = exp(u) du
        // Peak in *unweighted* f(u) occurs at u ~= 0.3 * u_max, or at 0.7 * u_max for
        // exp(u) f(u), so split domain into 0..2/3 and 2/3..1.
        let u_max = (1.0 + delta).ln();

        // u, n, w, delta, f
        let mut pts: Vec<(f64, f64, f64, f64, Complex64)> = Vec::with_capacity(30);

        for (x, w) in CLENSHAW_CURTIS_15_NODES_WEIGHTS.iter() {
            let u = (2.0 / 3.0) * u_max * x;
            let n = (n_min as f64) + u.exp_m1();
            pts.push((u, n, *w, 2.0 * u_max / 3.0, Complex64::new(0.0, 0.0)));
        }

        for (x, w) in CLENSHAW_CURTIS_15_NODES_WEIGHTS.iter() {
            let u = (2.0 + x) * u_max / 3.0;
            let n = (n_min as f64) + u.exp_m1();
            pts.push((u, n, *w, u_max / 3.0, Complex64::new(0.0, 0.0)));
        }

        // get a vec of all the distinct harmonic orders we need
        let harmonics: Vec<i32> = {
            let mut harmonics = vec![n_min];
            for i in 1..pts.len() {
                let diff = pts[i].1 - pts[i-1].1;
                if diff < 2.0 {
                    // interpolating
                    let n = pts[i].1;
                    let n = n.floor() as i32;
                    harmonics.push(n);
                    harmonics.push(n+1);
                } else {
                    let n = pts[i].1.round() as i32;
                    harmonics.push(n);
                };
            }
            harmonics.sort_unstable();
            harmonics.dedup();
            harmonics
        };

        // run this bit in parallel!
        let cache: HashMap<i32, Complex64> = harmonics.into_par_iter()
            .map(|n| (n, DoubleDiffPartialRate::new(n, a, eta).integrate()))
            .collect();

        for i in 1..pts.len() {
            let diff = pts[i].1 - pts[i-1].1;
            let pr = if diff < 2.0 {
                // interpolate
                let n = pts[i].1;
                let frac = n.fract();
                let n = n.floor() as i32;
                let pr_1 = cache.get(&n).unwrap();
                let pr_2 = cache.get(&(n+1)).unwrap();
                (1.0 - frac) * pr_1 + frac * pr_2
            } else {
                let n = pts[i].1.round() as i32;
                *(cache.get(&n).unwrap())
            };
            pts[i].4 = pr;
        }

        // Fill nodes of CDF
        let mut cdf = {
            let mut cdf = [[0.0; 3]; 16];
            cdf[0][0] = n_min as f64;

            // Split the log-scaled interval uniformly
            let mut u = 0.0;
            let mut du = u_max / 15.0;
            for i in 1..=15 {
                u = u + du;
                let n = (n_min as f64) + u.exp_m1();
                let n = if n.round() == cdf[i-1][0] {
                    // we're duplicating a point, so push n up
                    let n = cdf[i-1][0] + 1.0;
                    // reset u and du
                    u = (n - (n_min as f64) + 1.0).ln();
                    du = (u_max - u) / (15.0 - (i as f64));
                    n
                } else {
                    n.round()
                };
                cdf[i][0] = n;
            }

            cdf
        };

        // Construct Chebyshev interpolants of the CDF
        let f = cache.get(&n_min).unwrap();
        let f_lower: Vec<Complex64> = pts.iter().take(15).map(|(u, .., f)| f * u.exp()).collect();
        let f_upper: Vec<Complex64> = pts.iter().skip(15).map(|(u, .., f)| f * u.exp()).collect();
        assert!(f_lower.len() == 15);
        assert!(f_upper.len() == 15);
        let cdf_lower = IndefiniteIntegral::new(&f_lower, 0.0, 2.0 * u_max / 3.0, 0.5 * f);
        let cdf_upper = IndefiniteIntegral::new(&f_upper, 2.0 * u_max / 3.0, u_max, cdf_lower.at(2.0 * u_max / 3.0));

        for elem in cdf.iter_mut() {
            let u = (elem[0] - (n_min as f64) + 1.0).ln();
            let f = if u < 2.0 * u_max / 3.0 { cdf_lower.at(u) } else { cdf_upper.at(u) };
            elem[1] = f.re;
            elem[2] = f.im;
        }

        RateTable { cdf }
    }
}

#[cfg(test)]
mod tests {
    // use std::fs::File;
    // use std::io::Write;
    use rand_xoshiro::*;
    // use std::time::Duration;
    // use indicatif::{ProgressBar, ProgressStyle};
    use super::*;

    #[test]
    fn total_rate_lookup() {
        let num: usize = std::env::var("RAYON_NUM_THREADS")
            .map(|s| s.parse().unwrap_or(1))
            .unwrap_or(1);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num)
            .build()
            .unwrap();

        let pts = [
            (5.0, 1.00, 1.873432e-1, 3.331584e-1),
            (5.0, 0.20, 3.697912e-3, 6.947803e-3),
            (5.0, 0.10, 1.062241e-4, 2.040422e-4),
            (5.0, 0.05, 2.018323e-7, 3.957741e-7),
            (5.0, 0.02, 6.568476e-15, 1.298533e-14),
            (7.0, 1.00, 3.034581e-1, 5.330692e-1),
            (7.0, 0.20, 1.175448e-2, 2.195742e-2),
            (7.0, 0.10, 7.440645e-4, 1.422075e-3),
            (7.0, 0.05, 6.604356e-6, 1.285089e-5),
            (7.0, 0.02, 2.037976e-11, 3.996260e-11),
            (7.0, 0.005, 4.610807e-37, 9.102997e-37),
            (12.0, 1.00, 5.917700e-1, 9.984177e-1),
            (12.0, 0.20, 4.872358e-2, 8.798135e-2),
            (12.0, 0.10, 7.082706e-3, 1.328779e-2),
            (12.0, 0.05, 3.257889e-4, 6.139886e-4),
            (12.0, 0.02, 1.188422e-7, 2.308734e-7),
            (12.0, 0.005, 5.472640e-23, 1.084332e-22),
            (15.0, 1.00, 7.535927e-1, 1.262477e0),
            (15.0, 0.2, 7.810475e-2, 1.388705e-1),
            (15.0, 0.1, 1.453620e-2, 2.688097e-2),
            (15.0, 0.05, 1.036233e-3, 1.976234e-3),
            (15.0, 0.02, 1.473862e-6, 2.884801e-6),
            (15.0, 0.005, 5.381016e-19, 1.072959e-18),
        ];

        for (a, eta, target_par, target_perp) in &pts {
            let rate = TotalRate::new(*a, *eta);
            let result_par = rate.value(1.0);
            let result_perp = rate.value(-1.0);
            let (target_par, target_perp) = if *target_par == 0.0 {
                pool.install(|| rate.by_integration().total())
            } else {
                (*target_par, *target_perp)
            };
            let err_par = (target_par - result_par) / target_par;
            let err_perp = (target_perp - result_perp) / target_perp;
            let err_avg = ((target_par + target_perp) - (result_par + result_perp)) / (target_par + target_perp);

            println!(
                "a = {}, eta = {}: target = {:.6e} {:.6e}, err = {:.2}% {:.2}% [{:.2}%]",
                a, eta, target_par, target_perp,
                100.0 * err_par, 100.0 * err_perp, 100.0 * err_avg,
            );

            assert!(err_par.abs() < 2.0e-2);
            assert!(err_perp.abs() < 2.0e-2);
        }

        // a = 1.5

        // let mut file = File::create("output/nbw_lp_interp_err_a1.5.dat").unwrap();

        // let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}): {msg}")
        //     .unwrap();
        // let pb = ProgressBar::new(1000).with_style(style);
        // pb.enable_steady_tick(Duration::from_millis(100));

        // for i in 0..1000 {
        //     let a = 1.5;
        //     let eta = (0.002_f64.ln() + (2_f64.ln() - 0.002_f64.ln()) * (i as f64) / 1000.0).exp();
        //     pb.set_message(format!("eta = {:.3e}", eta));

        //     let rate = TotalRate::new(a, eta);
        //     let target = if rate.is_negligible() {
        //         continue;
        //     } else if a < 5.0 {
        //         pool.install(|| rate.by_summation().total())
        //     } else {
        //         pool.install(|| rate.by_integration().total())
        //     };

        //     pb.inc(1);
        //     let result = (rate.value(1.0), rate.value(-1.0));
        //     let err = ((target.0 - result.0) / target.0, (target.1 - result.1) / target.1);
        //     let avg_res = rate.value(0.0);
        //     let avg_tgt = 0.5 * (target.0 + target.1);
        //     let avg_err = (avg_tgt - avg_res) / avg_tgt;
        //     writeln!(file, "{:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e} {:.6e}", eta, target.0, result.0, err.0, target.1, result.1, err.1, avg_tgt, avg_res, avg_err).unwrap();
        // };

        // Randomly chosen a, eta

        // let mut file = File::create("output/nbw_lp_interp_err.dat").unwrap();
        // let mut rng = Xoshiro256StarStar::seed_from_u64(0);

        // let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}): {msg}")
        //     .unwrap();
        // let pb = ProgressBar::new(1000).with_style(style);
        // pb.enable_steady_tick(Duration::from_millis(100));

        // for _i in 0..1000 {
        //     let a = (0.1_f64.ln() + (20_f64.ln() - 0.1_f64.ln()) * rng.gen::<f64>()).exp();
        //     let eta = (0.002_f64.ln() + (2_f64.ln() - 0.002_f64.ln()) * rng.gen::<f64>()).exp();
        //     pb.set_message(format!("a = {:.3}, eta = {:.3e}", a, eta));

        //     let rate = TotalRate::new(a, eta);
        //     let target = if rate.is_negligible() {
        //         continue;
        //     } else if a < 5.0 {
        //         pool.install(|| rate.by_summation().total())
        //     } else {
        //         pool.install(|| rate.by_integration().total())
        //     };

        //     pb.inc(1);
        //     let result = (rate.value(1.0), rate.value(-1.0));
        //     let err = ((target.0 - result.0) / target.0, (target.1 - result.1) / target.1);
        //     writeln!(file, "{:.6e} {:.6e} {:.6e} {:.6e}", a, eta, err.0, err.1).unwrap();
        // };
    }

    #[test]
    fn summation_over_n() {
        let num: usize = std::env::var("RAYON_NUM_THREADS")
            .map(|s| s.parse().unwrap_or(1))
            .unwrap_or(1);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num)
            .build()
            .unwrap();

        let pts = [
            (5.0, 1.00, 1.857911e-1, 3.340399e-1),
            (5.0, 0.20, 3.682913e-3, 6.941696e-3),
            (5.0, 0.10, 1.061032e-4, 2.039585e-4),
            (5.0, 0.05, 2.028183e-7, 3.957505e-7),
            (5.0, 0.02, 6.552820e-15, 1.296773e-14),
            (7.0, 1.00, 3.041498e-1, 5.322159e-1),
            (7.0, 0.20, 1.183898e-2, 2.196409e-2),
            (7.0, 0.10, 7.449889e-4, 1.421329e-3),
            (7.0, 0.05, 6.612238e-6, 1.283869e-5),
            (7.0, 0.02, 2.029134e-11, 3.999135e-11),
        ];

        for (a, eta, target_par, target_perp) in &pts {
            let rate = TotalRate::new(*a, *eta);
            let rate = pool.install(|| rate.by_integration());
            let result = rate.total();
            let err_par = (target_par - result.0) / target_par;
            let err_perp = (target_perp - result.1) / target_perp;

            println!(
                "a = {}, eta = {}: target = {:.6e} {:.6e}, err = {:.2}% {:.2}%",
                a, eta, target_par, target_perp,
                100.0 * err_par, 100.0 * err_perp,
            );

            assert!(err_par.abs() < 1.0e-2);
            assert!(err_perp.abs() < 1.0e-2);
        }
    }

    #[test]
    fn integration_over_s_and_theta() {
        let pts = [
            ( 1.0,  0.1,    31, 1.646791418e-14, 3.406531716e-14), // 4000 x 4000
            ( 1.0,  0.1,    32, 9.869816939e-15, 2.119908242e-14),
            ( 1.0,  0.1,    35, 2.007846619e-15, 4.332723390e-15),
            ( 1.0,  0.1,    40, 1.602465895e-16, 3.581264569e-16),
            ( 3.0,  0.1,   111, 5.592163938e-8,  1.151441043e-7 ),
            ( 3.0,  0.1,   120, 4.744736678e-8,  9.584197406e-8 ),
            ( 3.0,  0.1,   140, 1.858852224e-8,  3.681258499e-8 ),
            ( 3.0,  0.1,   180, 3.747475034e-9,  6.653802050e-9 ),
            ( 3.0,  0.1,   220, 5.434722501e-10, 9.707658165e-10),
            (10.0,  0.1,  1021, 3.015321515e-6,  6.605732700e-6 ),
            (10.0,  0.1,  1100, 5.622463945e-6,  1.141983272e-5 ),
            (10.0,  0.1,  1500, 2.905881799e-6,  5.310687728e-6 ),
            (10.0,  0.1,  2000, 1.276555879e-6,  2.275444046e-6 ),
            (10.0,  0.1,  3000, 1.152856168e-7,  1.763929509e-7 ),
            ( 3.0, 0.01,  1111, 6.502873066e-44, 1.297185557e-43),
            ( 3.0, 0.01,  1150, 1.353161811e-44, 2.650952082e-44),
            ( 3.0, 0.01,  1200, 2.770977903e-45, 5.408247680e-45),
            ( 3.0, 0.01,  1350, 3.683763308e-47, 7.471773284e-47),
            (10.0, 0.01, 10201, 3.916663585e-18, 7.926212737e-18), // 2000 x 2000
            (10.0, 0.01, 10500, 5.200283830e-18, 1.032182681e-17),
            (10.0, 0.01, 11000, 2.577948532e-18, 5.065941188e-18),
            (10.0, 0.01, 13000, 2.906692165e-19, 5.686049535e-19),
            (10.0, 0.01, 16000, 1.320700399e-20, 2.559667394e-20),
            ( 1.0,  1.0,     4, 1.844141257e-3,  3.624303172e-3 ), // 4000 x 4000
            ( 1.0,  1.0,     7, 1.893728405e-4,  4.011983775e-4 ),
            ( 1.0,  1.0,    10, 1.907658419e-5,  3.025342852e-5 ),
            ( 3.0,  1.0,    12, 4.061597515e-3,  1.039506344e-2 ),
            ( 3.0,  1.0,    20, 3.768847872e-3,  6.120133328e-3 ),
            ( 3.0,  1.0,    40, 5.291178227e-4,  7.507287556e-4 ),
            ( 3.0,  1.0,    70, 2.833403160e-5,  3.407155252e-5 ),
            (10.0,  1.0,   103, 8.469204247e-4,  2.643715690e-3 ),
            (10.0,  1.0,   200, 1.610808750e-3,  2.824524221e-3 ),
            (10.0,  1.0,   300, 1.098327860e-3,  1.786228092e-3 ),
            (10.0,  1.0,   800, 6.801400124e-5,  7.778644702e-5 ),
            (10.0,  1.0,  1600, 4.337896871e-6,  4.612765156e-6 ),
            (20.0, 0.05,  8041, 2.765187304e-7,  6.089363125e-7 ), // 2000 x 2000
            (20.0, 0.05,  8500, 7.370834923e-7,  1.502157365e-6 ),
            (20.0, 0.05,  9000, 6.622163806e-7,  1.311737081e-6 ),
            (20.0, 0.05, 10000, 5.320547861e-7,  1.018684755e-6 ),
            (20.0, 0.05, 20000, 5.504167879e-8,  9.344886335e-8 ),
        ];

        let mut avg_error: [f64; 2] = [0.0, 0.0];

        for (a, eta, n, re, im) in &pts {
            let target = Complex64::new(*re, *im);

            let mut spectrum = DoubleDiffPartialRate::new(*n, *a, *eta);
            let result = spectrum.integrate();

            let err = Complex64::new(
                (target.re - result.re) / target.re,
                (target.im - result.im) / target.im,
            );

            avg_error[0] += err.re + err.im;
            avg_error[1] += err.norm_sqr();

            let indicator = if spectrum.max_theta(0.5) < 0.75 * consts::FRAC_PI_2 {
                "adaptive"
            } else {
                "non-adaptive"
            };

            println!(
                "a = {:>3}, eta = {:>4}, n = {:>5} => target = {:>20.3e}, got {:>20.3e} with {:+.2}% err with {} integration",
                a, eta, n, target, result, 100.0 * err, indicator,
            );
        }

        let len = 2.0 * (pts.len() as f64);
        let avg_error = [avg_error[0] / len, (avg_error[1] / len - (avg_error[0] / len).powi(2)).sqrt()];
        println!("RMS error = {:.2}% ± {:.2}%", 100.0 * avg_error[0], 100.0 * avg_error[1]);

        assert!(avg_error[0].abs() < 0.01);
    }

    #[test]
    fn ceiling_at_fixed_n() {
        let mut rng = Xoshiro256StarStar::seed_from_u64(0);

        for _i in 0..10 {
            let a = (0.1_f64.ln() + (20_f64.ln() - 0.1_f64.ln()) * rng.gen::<f64>()).exp();
            let eta = (0.01_f64.ln() + (1_f64.ln() - 0.01_f64.ln()) * rng.gen::<f64>()).exp();

            let total_rate = TotalRate::new(a, eta);

            if total_rate.is_negligible() {
                continue;
            }

            let sv1 = -1.0 + 2.0 * rng.gen::<f64>();
            let (n_min, n_max) = total_rate.sum_limits();
            let step = ((n_max - n_min) / 10).max(2);

            println!("a = {:>9.3e}, eta = {:>9.3e}, S_1 = {:+.3}:", a, eta, sv1);

            for n in (n_min..n_max).step_by(step as usize) {
                let mut spectrum = DoubleDiffPartialRate::new(n, a, eta);

                let target = {
                    let s_min = 0.5;
                    let s_max = 0.5 + (0.25 - (1.0 + 0.5 * a * a) / (2.0 * (n as f64) * eta)).sqrt();
                    (0..500)
                        .map(|i| {
                            let s = s_min + (s_max - s_min) * (i as f64) / 500.0;
                            spectrum.component(s, 0.0, sv1)
                        })
                        .reduce(f64::max)
                        .unwrap()
                };

                let max = spectrum.ceiling(sv1);
                let err = (target - max) / target;

                println!(
                    "\tn = {:>6} => max = {:>9.3e}, predicted = {:>9.3e}, err = {:.2}%",
                    n, target, max, 100.0 * err,
                );

                assert!(err < 0.0);
            }
        }
    }
}

#[cfg(test)]
mod table_generation {
    use std::fs::File;
    use std::io::Write;
    use std::time::Duration;
    use indicatif::{HumanDuration, ProgressBar, ProgressState, ProgressStyle};
    use super::*;

    fn smoothed_eta(s: &ProgressState, w: &mut dyn fmt::Write) {
        match (s.pos(), s.len()) {
            (0, _) => write!(w, "-").unwrap(),
            (pos, Some(len)) => write!(
                w,
                "{:#}",
                HumanDuration(Duration::from_millis(
                    (s.elapsed().as_millis() * (len as u128 - pos as u128) / (pos as u128))
                        as u64
                ))
            )
            .unwrap(),
            _ => write!(w, "-").unwrap(),
        }
    }

    #[test]
    #[ignore]
    fn mid_range() {
        let num: usize = std::env::var("RAYON_NUM_THREADS")
            .map(|s| s.parse().unwrap_or(1))
            .unwrap_or(1);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num)
            .build()
            .unwrap();

        const LN_MIN_A: f64 = -7.0 * consts::LN_10 / 5.0; // ~0.04
        const LN_MAX_ETA_PRIME: f64 = consts::LN_2; // 2.0
        const A_DENSITY: usize = 20; // points per order of magnitude
        const ETA_PRIME_DENSITY: usize = 4; // points per harmonic step
        const N_COLS: usize = 36; // points in a0
        const N_ROWS: usize = 157; // points in eta_prime

        let mut pts = vec![];
        for i in 0..N_ROWS {
            for j in 0..N_COLS {
                // eta' = 2 density / (i + 1)
                let eta_prime = LN_MAX_ETA_PRIME.exp() / (1.0 + (i as f64) / (ETA_PRIME_DENSITY as f64));
                let a = LN_MIN_A.exp() * 10_f64.powf((j as f64) / (A_DENSITY as f64));
                pts.push((i, j, a, eta_prime));
            }
        }

        println!("Generating pair-creation rate tables (LP) for mid-range a...");

        let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({smoothed_eta}): {msg}")
            .unwrap()
            .with_key("smoothed_eta", smoothed_eta);
        let pb = ProgressBar::new((N_COLS * N_ROWS) as u64).with_style(style);
        pb.enable_steady_tick(Duration::from_millis(100));

        // 10 threads, takes about 5 mins
        let pts: Vec<_> = pts.into_iter()
            .map(|(i, j, a, eta_prime)| {
                pb.set_message(format!("a = {:.3}, eta_prime = {:.3e}", a, eta_prime));

                let eta = (1.0 + 0.5 * a * a) * eta_prime;
                let rate = TotalRate::new(a, eta);
                let table = pool.install(|| rate.by_summation());

                pb.inc(1);

                (i, j, table)
            })
            .collect();

        // Build rate table
        let mut table = [[[0.0; 2]; N_COLS]; N_ROWS];
        for (i, j, rate) in pts.iter() {
            table[*i][*j][0] = rate.total().0;
            table[*i][*j][1] = rate.total().1;
        }

        let path = "output/nbw_mid_a_rate_table.rs";
        let mut file = File::create(&path).unwrap();
        writeln!(file, "pub const LN_MIN_A: f64 = {:.16e};", LN_MIN_A).unwrap();
        writeln!(file, "pub const LN_MAX_ETA_PRIME: f64 = {:.16e};", LN_MAX_ETA_PRIME).unwrap();
        writeln!(file, "pub const LN_MIN_ETA_PRIME: f64 = {:.16e};", LN_MAX_ETA_PRIME - (1.0 + ((N_ROWS - 1) as f64) / (ETA_PRIME_DENSITY as f64)).ln()).unwrap();
        writeln!(file, "pub const LN_A_STEP: f64 = {:.16e};", consts::LN_10 / (A_DENSITY as f64)).unwrap();
        writeln!(file, "pub const ETA_PRIME_DENSITY: f64 = {:.16e};", ETA_PRIME_DENSITY).unwrap();
        writeln!(file, "pub const TABLE: [[[f64; 2]; {}]; {}] = [", N_COLS, N_ROWS).unwrap();
        for row in table.iter() {
            write!(file, "\t[").unwrap();
            for val in row.iter() {
                let par = val[0].ln();
                let perp = val[1].ln();
                if par.is_finite() {
                    write!(file, "[{:>18.12e}", par).unwrap();
                } else {
                    write!(file, "[NEG_INFINITY").unwrap();
                }
                if perp.is_finite() {
                    write!(file, ", {:>18.12e}], ", perp).unwrap();
                } else {
                    write!(file, ", NEG_INFINITY], ").unwrap();
                }
            }
            writeln!(file, "],").unwrap();
        }
        writeln!(file, "];").unwrap();
        println!("Rate data for mid-range a written to {}", path);

        let path = "output/nbw_mid_a_cdf_table.rs";
        let mut file = File::create(&path).unwrap();
        writeln!(file, "pub const N_COLS: usize = {};", N_COLS).unwrap();
        writeln!(file, "pub const LN_MIN_A: f64 = {:.16e};", LN_MIN_A).unwrap();
        writeln!(file, "pub const LN_MAX_ETA_PRIME: f64 = {:.16e};", LN_MAX_ETA_PRIME).unwrap();
        writeln!(file, "pub const LN_A_STEP: f64 = {:.16e};", consts::LN_10 / (A_DENSITY as f64)).unwrap();
        writeln!(file, "pub const TABLE: [[[f64; 3]; 16]; {}] = [", N_COLS * N_ROWS).unwrap();
        for (_, _, rate) in &pts {
            writeln!(file, "\t{},", rate).unwrap();
        }
        writeln!(file, "];").unwrap();
        println!("CDF data for mid-range a written to {}", path);
    }

    #[test]
    #[ignore]
    fn high_range() {
        let num: usize = std::env::var("RAYON_NUM_THREADS")
            .map(|s| s.parse().unwrap_or(1))
            .unwrap_or(1);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num)
            .build()
            .unwrap();

        const LN_MIN_A: f64 = -consts::LN_10; // 0.1
        const A_DENSITY: usize = 10; // 20 points per order of magnitude
        const N_COLS: usize = 2 * A_DENSITY + 4; // + 7; // points in a0, a <= 20

        const LN_MIN_ETA: f64 = consts::LN_2 - 3_f64 * consts::LN_10; // 0.002
        const ETA_DENSITY: usize = 20; // 20 points per order of magnitude
        const N_ROWS: usize = 3 * ETA_DENSITY + 1; // points in eta, eta <= 2

        let mut pts = vec![];
        for i in 0..N_ROWS {
            for j in 0..N_COLS {
                let a = (LN_MIN_A + consts::LN_10 * (j as f64) / (A_DENSITY as f64)).exp();
                let eta = (LN_MIN_ETA + consts::LN_10 * (i as f64) / (ETA_DENSITY as f64)).exp();
                pts.push((i, j, a, eta));
            }
        }

        println!("Generating pair-creation rate tables (LP) for full range of a...");

        let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({smoothed_eta}): {msg}")
            .unwrap()
            .with_key("smoothed_eta", smoothed_eta);
        let pb = ProgressBar::new((N_COLS * N_ROWS) as u64).with_style(style);
        pb.enable_steady_tick(Duration::from_millis(100));

        let pts: Vec<_> = pts.into_iter()
            .map(|(i, j, a, eta)| {
                let rate = TotalRate::new(a, eta);
                let (n_min, n_max) = rate.sum_limits();
                pb.set_message(format!("a = {:.3}, eta = {:.3e}, n = {}..{}", a, eta, n_min, n_max));

                let table = if a > 5.0 {
                    pool.install(|| rate.by_integration())
                } else if !rate.can_be_skipped() {
                    pool.install(|| rate.by_summation())
                } else {
                    Default::default()
                };

                pb.suspend(|| println!(
                    "LP NBW: eta = {:>9.3e}, a = {:>9.3e}, i = {:>3}, j = {:>3} => {:>15.6e} {:>15.6e}",
                    eta, a, i, j, table.total().0.ln(), table.total().1.ln(),
                ));
                pb.inc(1);

                (i, j, table)
            })
            .collect();

        // Build rate table
        let mut table = [[[0.0; 2]; N_COLS]; N_ROWS];
        for (i, j, rate) in pts.iter() {
            table[*i][*j][0] = rate.total().0;
            table[*i][*j][1] = rate.total().1;
        }

        let path = "output/nbw_rate_table.rs";
        let mut file = File::create(&path).unwrap();
        writeln!(file, "use std::f64::NEG_INFINITY;").unwrap();
        writeln!(file, "pub const N_COLS: usize = {};", N_COLS).unwrap();
        writeln!(file, "pub const N_ROWS: usize = {};", N_ROWS).unwrap();
        writeln!(file, "pub const LN_MIN_A: f64 = {:.16e};", LN_MIN_A).unwrap();
        writeln!(file, "pub const LN_MIN_ETA: f64 = {:.16e};", LN_MIN_ETA).unwrap();
        writeln!(file, "pub const LN_A_STEP: f64 = {:.16e};", consts::LN_10 / (A_DENSITY as f64)).unwrap();
        writeln!(file, "pub const LN_ETA_STEP: f64 = {:.16e};", consts::LN_10 / (ETA_DENSITY as f64)).unwrap();
        writeln!(file, "pub const TABLE: [[[f64; 2]; {}]; {}] = [", N_COLS, N_ROWS).unwrap();
        for row in table.iter() {
            write!(file, "\t[").unwrap();
            for val in row.iter() {
                let par = val[0].ln();
                let perp = val[1].ln();
                if par.is_finite() {
                    write!(file, "[{:>18.12e}", par).unwrap();
                } else {
                    write!(file, "[NEG_INFINITY").unwrap();
                }
                if perp.is_finite() {
                    write!(file, ", {:>18.12e}], ", perp).unwrap();
                } else {
                    write!(file, ", NEG_INFINITY], ").unwrap();
                }
            }
            writeln!(file, "],").unwrap();
        }
        writeln!(file, "];").unwrap();
        println!("Rate data written to {}", path);

        let path = "output/nbw_cdf_table.rs";
        let mut file = File::create(&path).unwrap();
        writeln!(file, "pub const N_COLS: usize = {};", N_COLS).unwrap();
        // writeln!(file, "pub const N_ROWS: usize = {};", N_ROWS).unwrap();
        writeln!(file, "pub const MIN: [f64; 2] = [{:.16e}, {:.16e}];", LN_MIN_A, LN_MIN_ETA).unwrap();
        writeln!(file, "pub const STEP: [f64; 2] = [{:.16e}, {:.16e}];", consts::LN_10 / (A_DENSITY as f64), consts::LN_10 / (ETA_DENSITY as f64)).unwrap();
        writeln!(file, "pub const TABLE: [[[f64; 3]; 16]; {}] = [", N_COLS * N_ROWS).unwrap();
        for (_, _, rate) in &pts {
            writeln!(file, "\t{},", rate).unwrap();
        }
        writeln!(file, "];").unwrap();
        println!("CDF data written to {}", path);
    }
}
