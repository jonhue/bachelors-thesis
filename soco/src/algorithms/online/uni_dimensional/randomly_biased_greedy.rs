use crate::algorithms::online::{FractionalStep, Step};
use crate::config::{Config, FractionalConfig};
use crate::numerics::convex_optimization::find_minimizer;
use crate::problem::{FractionalSmoothedConvexOptimization, Online};
use crate::result::{Failure, Result};
use crate::schedule::FractionalSchedule;
use crate::utils::{assert, sample_uniform};
use noisy_float::prelude::*;
use pyo3::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Memory {
    /// Random number `r` representing bias.
    r: f64,
}
impl Default for Memory {
    fn default() -> Self {
        Memory {
            r: sample_uniform(-1., 1.),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Options {
    /// Scaling factor for norm. `theta >= 1. Defaults to `1`.
    #[pyo3(get, set)]
    pub theta: f64,
}
impl Default for Options {
    fn default() -> Self {
        Options { theta: 1. }
    }
}
#[pymethods]
impl Options {
    #[new]
    fn constructor(theta: f64) -> Self {
        Options { theta }
    }
}

/// Randomly Biased Greedy
pub fn rbg(
    o: Online<FractionalSmoothedConvexOptimization<'_>>,
    t: i32,
    _: &FractionalSchedule,
    m: Memory,
    options: Options,
) -> Result<FractionalStep<Memory>> {
    assert(o.w == 0, Failure::UnsupportedPredictionWindow(o.w))?;
    assert(o.p.d == 1, Failure::UnsupportedProblemDimension(o.p.d))?;

    let x = next(o, t, m.r, options.theta)?;
    Ok(Step(Config::single(x), None))
}

fn next(
    o: Online<FractionalSmoothedConvexOptimization<'_>>,
    t: i32,
    r: f64,
    theta: f64,
) -> Result<f64> {
    let objective = |raw_x: &[f64]| -> N64 {
        let x = Config::new(raw_x.to_vec());
        w(&o, t - 1, theta, x.clone()).unwrap()
            + n64(r) * n64(theta) * (o.p.switching_cost)(x)
    };

    let (x, _) = find_minimizer(objective, &o.p.bounds)?;
    Ok(x[0])
}

fn w(
    o: &Online<FractionalSmoothedConvexOptimization<'_>>,
    t: i32,
    theta: f64,
    x: FractionalConfig,
) -> Result<N64> {
    if t == 0 {
        Ok(n64(theta) * (o.p.switching_cost)(x))
    } else {
        let f = |raw_y: &[f64]| -> N64 {
            let y = Config::new(raw_y.to_vec());
            w(o, t - 1, theta, y.clone()).unwrap()
                + o.p.hit_cost(t, y.clone())
                + n64(theta) * (o.p.switching_cost)(x.clone() - y)
        };

        let (_, opt) = find_minimizer(f, &o.p.bounds)?;
        Ok(opt)
    }
}
