use crate::{
    algorithms::offline::{
        multi_dimensional::{
            approx_graph_search::{
                approx_graph_search, Options as ApproxGraphSearchOptions,
            },
            convex_optimization::co,
            optimal_graph_search::optimal_graph_search,
        },
        uni_dimensional::{
            capacity_provisioning::brcp,
            optimal_graph_search::{
                optimal_graph_search as optimal_graph_search_1d,
                Options as OptimalGraphSearch1dOptions,
            },
        },
    },
    model::data_center::model::{DataCenterModel, DataCenterOfflineInput},
    streaming::offline,
};
use pyo3::prelude::*;

/// Backward-Recurrent Capacity Provisioning
#[pyfunction]
#[pyo3(name = "brcp")]
fn brcp_py(
    model: DataCenterModel,
    input: DataCenterOfflineInput,
    inverted: bool,
) -> PyResult<Vec<Vec<f64>>> {
    Ok(offline::solve(&model, &brcp, (), input, inverted)
        .unwrap()
        .to_vec())
}

/// Graph-Based Optimal Algorithm (uni-dimensional)
#[pyfunction]
#[pyo3(name = "optimal_graph_search_1d")]
fn optimal_graph_search_1d_py(
    model: DataCenterModel,
    input: DataCenterOfflineInput,
    options: OptimalGraphSearch1dOptions,
    inverted: bool,
) -> PyResult<(Vec<Vec<i32>>, f64)> {
    let result = offline::solve(
        &model,
        &optimal_graph_search_1d,
        options,
        input,
        inverted,
    )
    .unwrap();
    Ok((result.xs.to_vec(), result.cost))
}

/// Graph-Based Optimal Algorithm
#[pyfunction]
#[pyo3(name = "optimal_graph_search")]
fn optimal_graph_search_py(
    model: DataCenterModel,
    input: DataCenterOfflineInput,
    inverted: bool,
) -> PyResult<(Vec<Vec<i32>>, f64)> {
    let result =
        offline::solve(&model, &optimal_graph_search, (), input, inverted)
            .unwrap();
    Ok((result.xs.to_vec(), result.cost))
}

/// Graph-Based Polynomial-Time Approximation Scheme
#[pyfunction]
#[pyo3(name = "approx_graph_search")]
fn approx_graph_search_py(
    model: DataCenterModel,
    input: DataCenterOfflineInput,
    options: ApproxGraphSearchOptions,
    inverted: bool,
) -> PyResult<(Vec<Vec<i32>>, f64)> {
    let result =
        offline::solve(&model, &approx_graph_search, options, input, inverted)
            .unwrap();
    Ok((result.xs.to_vec(), result.cost))
}

/// Convex Optimization
#[pyfunction]
#[pyo3(name = "co")]
fn co_py(
    model: DataCenterModel,
    input: DataCenterOfflineInput,
    inverted: bool,
) -> PyResult<Vec<Vec<f64>>> {
    Ok(offline::solve(&model, &co, (), input, inverted)
        .unwrap()
        .to_vec())
}

pub fn submodule(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(brcp_py, m)?)?;

    m.add_function(wrap_pyfunction!(optimal_graph_search_1d_py, m)?)?;
    m.add_class::<OptimalGraphSearch1dOptions>()?;

    m.add_function(wrap_pyfunction!(optimal_graph_search_py, m)?)?;

    m.add_function(wrap_pyfunction!(approx_graph_search_py, m)?)?;
    m.add_class::<ApproxGraphSearchOptions>()?;

    m.add_function(wrap_pyfunction!(co_py, m)?)?;

    Ok(())
}
