use crate::config::Config;
use crate::online::{FractionalStep, Online, Step};
use crate::problem::FractionalSimplifiedSmoothedConvexOptimization;
use crate::result::{Error, Result};
use crate::schedule::FractionalSchedule;
use crate::utils::assert;
use crate::PRECISION;
use nlopt::{Algorithm, Nlopt, Target};

/// Memoryless Algorithm. Special case of Primal Online Balanced Descent.
pub fn memoryless(
    o: &Online<FractionalSimplifiedSmoothedConvexOptimization<'_>>,
    xs: &mut FractionalSchedule,
    _: &mut Vec<()>,
    _: &(),
) -> Result<FractionalStep<()>> {
    assert(o.w == 0, Error::UnsupportedPredictionWindow)?;
    assert(o.p.d == 1, Error::UnsupportedProblemDimension)?;

    let t = xs.t_end() + 1;
    let prev_x = if xs.is_empty() { 0. } else { xs.now()[0] };

    let x = next(o, t, prev_x)?;
    Ok(Step(Config::single(x), None))
}

/// Determines next `x` with a convex optimization.
fn next(
    o: &Online<FractionalSimplifiedSmoothedConvexOptimization<'_>>,
    t: i32,
    prev_x: f64,
) -> Result<f64> {
    let objective_function =
        |xs: &[f64], _: Option<&mut [f64]>, _: &mut ()| -> f64 {
            (o.p.hitting_cost)(t, Config::new(xs.to_vec())).unwrap()
        };
    let mut xs = [0.];
    let mut opt = Nlopt::new(
        Algorithm::Bobyqa,
        1,
        objective_function,
        Target::Minimize,
        (),
    );
    opt.set_lower_bound(0.)?;
    opt.set_upper_bound(o.p.bounds[0])?;
    opt.set_xtol_rel(PRECISION)?;
    opt.add_inequality_constraint(
        |xs: &[f64], _: Option<&mut [f64]>, _: &mut ()| -> f64 {
            (xs[0] - prev_x).abs()
                - (o.p.hitting_cost)(t, Config::new(xs.to_vec())).unwrap() / 2.
        },
        (),
        PRECISION,
    )?;
    opt.optimize(&mut xs)?;
    Ok(xs[0])
}
