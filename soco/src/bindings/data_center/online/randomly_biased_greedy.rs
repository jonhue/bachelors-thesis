use super::{Response, StepResponse};
use crate::{
    algorithms::online::uni_dimensional::randomly_biased_greedy::{
        rbg, Memory, Options,
    },
    model::data_center::model::{
        DataCenterModel, DataCenterOfflineInput, DataCenterOnlineInput,
    },
    problem::FractionalSimplifiedSmoothedConvexOptimization,
    streaming::online,
};
use pyo3::prelude::*;

/// Starts backend in a new thread.
#[pyfunction]
#[allow(clippy::type_complexity)]
fn start(
    addr: String,
    model: DataCenterModel,
    input: DataCenterOfflineInput,
    w: i32,
    options: Options,
) -> PyResult<Response<f64, Memory>> {
    let ((xs, cost), (int_xs, int_cost), m) = online::start(
        addr.parse().unwrap(),
        model,
        &rbg,
        options,
        w,
        input,
        None,
    )
    .unwrap();
    Ok(((xs.to_vec(), cost), (int_xs.to_vec(), int_cost), m))
}

/// Executes next iteration of the algorithm.
#[pyfunction]
fn next(
    addr: String,
    input: DataCenterOnlineInput,
) -> PyResult<StepResponse<f64, Memory>> {
    let ((x, cost), (int_x, int_cost), m) = online::next::<
        f64,
        FractionalSimplifiedSmoothedConvexOptimization,
        Memory,
        DataCenterOnlineInput,
    >(addr.parse().unwrap(), input);
    Ok(((x.to_vec(), cost), (int_x.to_vec(), int_cost), m))
}

/// Memoryless Algorithm
pub fn submodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start, m)?)?;
    m.add_function(wrap_pyfunction!(next, m)?)?;

    m.add_class::<Options>()?;

    Ok(())
}
