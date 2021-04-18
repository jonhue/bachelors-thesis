use crate::problem::types::{
    DiscreteHomProblem, DiscreteSchedule, Online, OnlineSolution,
};
use crate::problem::utils::project;

/// Lower and upper bound at some time t.
type Memory = (i32, i32);

impl<'a> Online<DiscreteHomProblem<'a>> {
    /// Discrete Lazy Capacity Provisioning.
    pub fn lcp(
        &self,
        xs: DiscreteSchedule,
        _ms: &Vec<Memory>,
    ) -> OnlineSolution<i32, Memory> {
        let i = if xs.len() > 0 { xs[xs.len() - 1] } else { 0 };
        let l = self.lower_bound();
        let u = self.upper_bound();
        (project(i, l, u), (l, u))
    }

    fn lower_bound(&self) -> i32 {
        1
    }

    fn upper_bound(&self) -> i32 {
        1
    }
}
