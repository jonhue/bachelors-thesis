use crate::algorithms::online::{FractionalStep, Step};
use crate::config::{Config, FractionalConfig};
use crate::cost::CostFn;
use crate::norm::NormFn;
use crate::numerics::convex_optimization::{
    find_unbounded_minimizer, Constraint,
};
use crate::problem::{FractionalSmoothedConvexOptimization, Online};
use crate::result::{Failure, Result};
use crate::schedule::FractionalSchedule;
use crate::utils::assert;
use finitediff::FiniteDiff;
use noisy_float::prelude::*;
use std::sync::Arc;

pub struct Options<'a> {
    /// Determines the l-level set used in each step by the algorithm.
    pub l: f64,
    /// Mirror map chosen based on the used norm.
    pub mirror_map: NormFn<'a, f64>,
}

/// Online Balanced Descent (meta algorithm)
pub fn obd(
    o: &Online<FractionalSmoothedConvexOptimization>,
    xs: &mut FractionalSchedule,
    _: &mut Vec<()>,
    options: &Options,
) -> Result<FractionalStep<()>> {
    assert(o.w == 0, Failure::UnsupportedPredictionWindow(o.w))?;

    let t = xs.t_end() + 1;
    let prev_x = xs.now_with_default(Config::repeat(0., o.p.d));

    let x = bregman_projection(
        &options.mirror_map,
        &o.p.hitting_cost,
        t,
        options.l,
        &prev_x,
    )?;
    Ok(Step(x, None))
}

/// Bregman projection of `x` onto a convex `l`-sublevel set `K` of `f`.
///
/// `mirror_map` must be `m`-strongly convex and `M`-Lipschitz smooth for the norm function with fixed `m` and `M`.
fn bregman_projection(
    mirror_map: &NormFn<'_, f64>,
    f: &CostFn<'_, FractionalConfig>,
    t: i32,
    l: f64,
    x: &FractionalConfig,
) -> Result<FractionalConfig> {
    let objective = |y: &[f64]| -> N64 {
        bregman_divergence(mirror_map, Config::new(y.to_vec()), x.clone())
    };
    // `l`-sublevel set of `f`
    let constraint = Constraint {
        data: (),
        g: Arc::new(|y, _| f.call_certain(t, Config::new(y.to_vec())) - n64(l)),
    };

    let (y, _) = find_unbounded_minimizer(objective, x.d(), vec![constraint])?;
    Ok(Config::new(y))
}

/// Bregman divergence between `x` and `y`.
fn bregman_divergence(
    mirror_map: &NormFn<'_, f64>,
    x: FractionalConfig,
    y: FractionalConfig,
) -> N64 {
    let m = |x: &Vec<f64>| mirror_map(Config::new(x.clone())).raw();
    let mx = mirror_map(x.clone());
    let my = mirror_map(y.clone());
    let grad = Config::new(y.to_vec().central_diff(&m));
    mx - my - grad * (x - y)
}
