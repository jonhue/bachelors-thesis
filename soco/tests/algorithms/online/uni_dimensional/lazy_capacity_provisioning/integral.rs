mod lcp {
    use std::sync::Arc;
    use soco::algorithms::offline::multi_dimensional::approx_graph_search::{Options as ApproxOptions};
    use soco::algorithms::online::uni_dimensional::lazy_capacity_provisioning::integral::{Options, lcp};
    use soco::online::Online;
    use soco::problem::SimplifiedSmoothedConvexOptimization;
    use soco::config::Config;
    use soco::schedule::Schedule;

    #[test]
    fn _1() {
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end: 1,
            bounds: vec![5],
            switching_cost: vec![1.],
            hitting_cost: Arc::new(|t, j| {
                Some(t as f64 * (if j[0] == 0 { 1. } else { 0. }))
            }),
        };
        let mut o = Online { p, w: 0 };
        o.verify().unwrap();

        let result = o
            .stream(
                lcp,
                |_, _, _| false,
                &Options {
                    optimize_reference_time: true,
                    use_approx: None,
                },
            )
            .unwrap();
        result.0.verify(o.p.t_end, &o.p.bounds).unwrap();

        assert_eq!(result.0, Schedule::new(vec![Config::single(0)]));
    }

    #[test]
    fn _2() {
        let approx_options = ApproxOptions { gamma: None };
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end: 1,
            bounds: vec![5],
            switching_cost: vec![1.],
            hitting_cost: Arc::new(|t, j| {
                Some(t as f64 * (if j[0] == 0 { 1. } else { 0. }))
            }),
        };
        let mut o = Online { p, w: 0 };
        o.verify().unwrap();

        let result = o
            .stream(
                lcp,
                |_, _, _| false,
                &Options {
                    optimize_reference_time: true,
                    use_approx: Some(&approx_options),
                },
            )
            .unwrap();
        result.0.verify(o.p.t_end, &o.p.bounds).unwrap();

        assert_eq!(result.0, Schedule::new(vec![Config::single(0)]));
    }

    #[test]
    fn _3() {
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end: 1,
            bounds: vec![5],
            switching_cost: vec![1.],
            hitting_cost: Arc::new(|t, j| {
                Some(t as f64 * (if j[0] == 0 { 1. } else { 0. }))
            }),
        };
        let mut o = Online { p, w: 0 };
        o.verify().unwrap();

        let t_end = 2;
        let result = o
            .offline_stream(
                lcp,
                t_end,
                &Options {
                    optimize_reference_time: true,
                    use_approx: None,
                },
            )
            .unwrap();
        result.0.verify(t_end, &o.p.bounds).unwrap();

        assert_eq!(
            result.0,
            Schedule::new(vec![Config::single(0), Config::single(1)])
        );
    }

    #[test]
    fn _4() {
        let approx_options = ApproxOptions { gamma: None };
        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end: 1,
            bounds: vec![5],
            switching_cost: vec![1.],
            hitting_cost: Arc::new(|t, j| {
                Some(t as f64 * (if j[0] == 0 { 1. } else { 0. }))
            }),
        };
        let mut o = Online { p, w: 0 };
        o.verify().unwrap();

        let t_end = 2;
        let result = o
            .offline_stream(
                lcp,
                t_end,
                &Options {
                    optimize_reference_time: true,
                    use_approx: Some(&approx_options),
                },
            )
            .unwrap();
        result.0.verify(t_end, &o.p.bounds).unwrap();

        assert_eq!(
            result.0,
            Schedule::new(vec![Config::single(0), Config::single(1)])
        );
    }
}
