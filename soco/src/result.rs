//! Result types.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Failure {
    #[error("Bisecting failed with message: {0}")]
    Bisection(String),
    #[error("Integrating failed with message: {0}")]
    Integration(String),
    #[error("A verifier determined an invalidity: {0}")]
    Invalid(String),
    #[error("The interval from {from} to {to} is invalid.")]
    InvalidInterval { from: f64, to: f64 },
    #[error("The given matrix must be invertible to compute the Mahalanobis distance.")]
    MatrixMustBeInvertible,
    // #[error("NLopt returned with an error.")]
    // NlOpt(nlopt::FailState),
    #[error("When solving an online problem, the time horizon `T` should equal the current time slot plus the prediction window. But instead we have `T = {t_end}`, `t = {t}`, and `w = {w}`.")]
    OnlineInsufficientInformation { t_end: i32, t: i32, w: i32 },
    #[error("When solving an online problem from a given time slot, the property `t_end` (current time slot) must always be one time slot ahead of the length of the obtained schedule (number of previous time slots). Yet, the number of previous time slots is {previous_time_slots} and the current time slot is {current_time_slot}.")]
    OnlineInconsistentCurrentTimeSlot {
        previous_time_slots: i32,
        current_time_slot: i32,
    },
    #[error("When solving an online problem from a given time slot, the accumulated memory up to this time slot must be provided. Yet, the number of previous time slots is {previous_time_slots} and the memory consists of {memory_entries} entries.")]
    OnlineOutOfDateMemory {
        previous_time_slots: i32,
        memory_entries: i32,
    },
    #[error("This algorithm expects a problem instance which is the relaxed problem of an integral problem. In particular, the bounds shouldn't be fractional.")]
    MustBeRelaxedProblem,
    #[error("This algorithm does not support inverted movement costs. Set `inverted = false`.")]
    UnsupportedInvertedCost,
    #[error("This algorithm does not support `L`-constrained movement. Set `l = None`.")]
    UnsupportedLConstrainedMovement,
    #[error("This online algorithm does not support a prediction window. Set `w = 0` (was {0}).")]
    UnsupportedPredictionWindow(i32),
    #[error("This online algorithm does not support multi-dimensional problems. Set `d = 1` (was {0}).")]
    UnsupportedProblemDimension(i32),
}

// impl From<nlopt::FailState> for Failure {
//     fn from(error: nlopt::FailState) -> Self {
//         Failure::NlOpt(error)
//     }
// }

pub type Result<T> = std::result::Result<T, Failure>;
