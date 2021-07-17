mod make_pow_of_2 {
    use soco::algorithms::offline::uni_dimensional::optimal_graph_search::make_pow_of_2;
    use soco::config::Config;
    use soco::cost::{CostFn, SingleCostFn};
    use soco::problem::SimplifiedSmoothedConvexOptimization;
    use soco::verifiers::VerifiableProblem;

    #[test]
    fn _1() {
        let t_end = 1_000;
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end,
            bounds: vec![103],
            switching_cost: vec![1.],
            hitting_cost: CostFn::stretch(
                1,
                t_end,
                SingleCostFn::certain(|_, _| 1.),
            ),
        };
        p.verify().unwrap();
        let transformed_p = make_pow_of_2(p.clone()).unwrap();
        transformed_p.verify().unwrap();

        assert_eq!(transformed_p.t_end, p.t_end);
        assert_eq!(transformed_p.bounds[0], 128);
        assert_abs_diff_eq!(
            transformed_p.switching_cost[0],
            p.switching_cost[0]
        );

        for t in 1..=transformed_p.t_end {
            for j in 0..=transformed_p.bounds[0] {
                assert_abs_diff_eq!(
                    transformed_p.hit_cost(t, Config::single(j)),
                    if j <= p.bounds[0] {
                        1.
                    } else {
                        j as f64 * (1. + std::f64::EPSILON)
                    }
                );
            }
        }
    }
}

mod optimal_graph_search {
    use crate::factories::{penalize_zero, random};
    use soco::algorithms::offline::{
        uni_dimensional::optimal_graph_search::{
            make_pow_of_2, optimal_graph_search, Options,
        },
        OfflineAlgorithm, OfflineAlgorithmWithDefaultOptions,
    };
    use soco::config::Config;
    use soco::objective::Objective;
    use soco::problem::SimplifiedSmoothedConvexOptimization;
    use soco::schedule::Schedule;
    use soco::verifiers::VerifiableProblem;

    #[test]
    fn _1() {
        let t_end = 2;
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end,
            bounds: vec![2],
            switching_cost: vec![1.],
            hitting_cost: penalize_zero(t_end),
        };
        p.verify().unwrap();

        let result = optimal_graph_search
            .solve_with_default_options(p.clone(), false)
            .unwrap();
        result.xs.verify(p.t_end, &p.bounds).unwrap();
        let inv_result = optimal_graph_search
            .solve_with_default_options(p.clone(), true)
            .unwrap();
        inv_result.xs.verify(p.t_end, &p.bounds).unwrap();

        assert_eq!(result.xs, inv_result.xs);
        assert_abs_diff_eq!(result.cost, inv_result.cost);
        assert_eq!(
            result.xs,
            Schedule::new(vec![Config::single(1), Config::single(1)])
        );
        assert_abs_diff_eq!(result.cost, 1.);
        assert_abs_diff_eq!(
            result.cost,
            p.objective_function(&result.xs).unwrap()
        );
    }

    #[test]
    fn _2() {
        let t_end = 100;
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end,
            bounds: vec![8],
            switching_cost: vec![1.],
            hitting_cost: random(t_end),
        };
        p.verify().unwrap();

        let result = optimal_graph_search
            .solve_with_default_options(p.clone(), false)
            .unwrap();
        result.xs.verify(p.t_end, &p.bounds).unwrap();
        let inv_result = optimal_graph_search
            .solve_with_default_options(p.clone(), true)
            .unwrap();
        inv_result.xs.verify(p.t_end, &p.bounds).unwrap();

        assert_eq!(result.xs, inv_result.xs);
        assert_abs_diff_eq!(result.cost, inv_result.cost);
        assert_abs_diff_eq!(
            result.cost,
            p.objective_function(&result.xs).unwrap()
        );
    }

    #[test]
    fn _3() {
        let t_end = 1_000;
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end,
            bounds: vec![9],
            switching_cost: vec![1.],
            hitting_cost: random(t_end),
        };
        p.verify().unwrap();

        let transformed_p = make_pow_of_2(p).unwrap();
        let result = optimal_graph_search
            .solve_with_default_options(transformed_p.clone(), false)
            .unwrap();
        result
            .xs
            .verify(transformed_p.t_end, &transformed_p.bounds)
            .unwrap();
        let inv_result = optimal_graph_search
            .solve_with_default_options(transformed_p.clone(), true)
            .unwrap();
        inv_result
            .xs
            .verify(transformed_p.t_end, &transformed_p.bounds)
            .unwrap();

        assert_eq!(result.xs, inv_result.xs);
        assert_abs_diff_eq!(result.cost, inv_result.cost);
        assert_abs_diff_eq!(
            result.cost,
            transformed_p.objective_function(&result.xs).unwrap()
        );
    }

    #[test]
    fn _4() {
        let t_end = 2;
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end,
            bounds: vec![2],
            switching_cost: vec![1.],
            hitting_cost: penalize_zero(t_end),
        };
        p.verify().unwrap();

        let result = optimal_graph_search
            .solve(p.clone(), Options { x_start: 2 }, false)
            .unwrap();
        result.xs.verify(p.t_end, &p.bounds).unwrap();

        assert_eq!(
            result.xs,
            Schedule::new(vec![Config::single(1), Config::single(1)])
        );
        assert_abs_diff_eq!(result.cost, 0.);
        assert_abs_diff_eq!(
            result.cost,
            p.objective_function_with_default(
                &result.xs,
                &Config::single(2),
                false
            )
            .unwrap()
        );
    }
}
