use crate::algorithms::graph_search::Path;
use crate::algorithms::offline::multi_dimensional::optimal_graph_search::optimal_graph_search;
use crate::config::Config;
use crate::online::Online;
use crate::online::OnlineSolution;
use crate::problem::IntegralSmoothedLoadOptimization;
use crate::result::{Error, Result};
use crate::schedule::IntegralSchedule;
use crate::utils::{assert, total_bound};
use std::cmp::max;

/// Lane distribution at some time `t`.
#[derive(Clone)]
pub struct Memory {
    pub lanes: Lanes,
    pub horizons: Horizons,
}

/// Maps each lane to the dimension it is "handled by" at some time `t`.
/// If value is `0`, there the lane is not "active".
pub type Lanes = Vec<i32>;

/// Maps each lane to a finite time horizon it stays "active" for unless replaced by another dimension.
pub type Horizons = Vec<i32>;

/// Deterministic Algorithm
pub fn deterministic<'a>(
    o: &'a Online<IntegralSmoothedLoadOptimization>,
    xs: &IntegralSchedule,
    ms: &Vec<Memory>,
) -> Result<OnlineSolution<i32, Memory>> {
    assert(o.w == 0, Error::UnsupportedPredictionWindow)?;

    let t = xs.t_end() + 1;
    let bound = total_bound(&o.p.bounds) as usize;
    let optimal_lanes = find_optimal_lanes(&o.p, bound)?;
    let Memory {
        lanes: prev_lanes,
        mut horizons,
    } = if ms.is_empty() {
        Memory {
            lanes: vec![0; bound],
            horizons: vec![0; bound],
        }
    } else {
        ms[ms.len() - 1].clone()
    };

    let mut lanes = vec![0; bound];
    for k in 0..bound {
        if prev_lanes[k] < optimal_lanes[k] || t >= horizons[k] {
            lanes[k] = optimal_lanes[k];
            horizons[k] = t + next_time_horizon(
                &o.p.hitting_cost,
                &o.p.switching_cost,
                lanes[k],
            );
        } else {
            lanes[k] = prev_lanes[k];
            horizons[k] = max(
                horizons[k],
                t + next_time_horizon(
                    &o.p.hitting_cost,
                    &o.p.switching_cost,
                    lanes[k],
                ),
            );
        }
    }

    let config = collect_config(o.p.d, &lanes);
    Ok(OnlineSolution(config, Memory { lanes, horizons }))
}

fn next_time_horizon(
    hitting_cost: &Vec<f64>,
    switching_cost: &Vec<f64>,
    k: i32,
) -> i32 {
    if k == 0 {
        0
    } else {
        (switching_cost[k as usize - 1] / hitting_cost[k as usize - 1]).floor()
            as i32
    }
}

fn collect_config(d: i32, lanes: &Lanes) -> Config<i32> {
    let mut config = vec![0; d as usize];
    for i in 0..lanes.len() {
        config[lanes[i] as usize] += 1;
    }
    Config::new(config)
}

fn build_lanes(x: &Config<i32>, d: i32, bound: usize) -> Lanes {
    let mut lanes = vec![0; bound];
    for (k, lane) in lanes.iter_mut().enumerate() {
        if k as i32 <= active_lanes(x, 1, d) {
            for j in 1..=d {
                if active_lanes(x, j, d) >= k as i32 {
                    *lane = j;
                } else {
                    continue;
                }
            }
        }
    }
    lanes
}

/// Sums step across dimension from `from` to `to`.
fn active_lanes(x: &Config<i32>, from: i32, to: i32) -> i32 {
    let mut result = 0;
    for k in from..=to {
        result += x[k as usize];
    }
    result
}

fn find_optimal_lanes(
    p: &IntegralSmoothedLoadOptimization,
    bound: usize,
) -> Result<Lanes> {
    let Path(xs, _) = optimal_graph_search(&p.to_sco())?;
    Ok(build_lanes(&xs.now(), p.d, bound))
}
