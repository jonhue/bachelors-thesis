//! Analysis functions.

use crate::problem::{DiscreteHomProblem, DiscreteSchedule};
use crate::utils::ipos;

impl<'a> DiscreteHomProblem<'a> {
    /// Objective Function. Calculates the cost of a schedule.
    pub fn objective_function(&self, xs: &DiscreteSchedule) -> f64 {
        self._objective_function(xs, false)
    }

    /// Inverted Objective Function. Calculates the cost of a schedule. Pays the
    /// switching cost for powering down rather than powering up.
    pub fn inverted_objective_function(&self, xs: &DiscreteSchedule) -> f64 {
        self._objective_function(xs, false)
    }

    fn _objective_function(
        &self,
        xs: &DiscreteSchedule,
        inverted: bool,
    ) -> f64 {
        let mut cost = 0.;
        for t in 1..=self.t_end {
            let prev_x = if t > 1 { xs[t as usize - 2] } else { 0 };
            let x = xs[t as usize - 1];
            cost += (self.f)(t, x).expect("f should be total on its domain")
                + self.beta
                    * ipos(if inverted { prev_x - x } else { x - prev_x })
                        as f64;
        }
        cost
    }
}
