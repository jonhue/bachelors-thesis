#[cfg(test)]
mod bcp {
    use crate::factories::inv_e;
    use crate::init;
    use soco::algorithms::offline::{
        uni_dimensional::capacity_provisioning::brcp,
    };
    use soco::algorithms::offline::{OfflineAlgorithm, OfflineOptions, OfflineResult};
    use soco::config::Config;
    use soco::convert::DiscretizableSchedule;
    use soco::problem::SimplifiedSmoothedConvexOptimization;
    use soco::schedule::Schedule;
    use soco::verifiers::VerifiableProblem;

    #[test]
    fn _1() {
        init();

        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end: 2,
            bounds: vec![2.],
            switching_cost: vec![1.],
            hitting_cost: inv_e(),
        };
        p.verify().unwrap();

        let result = brcp
            .solve_with_default_options(p.clone(), OfflineOptions::default())
            .unwrap()
            .xs();
        result.verify(p.t_end, &p.bounds).unwrap();

        assert_eq!(
            result.to_i(),
            Schedule::new(vec![Config::single(2), Config::single(2)])
        );
    }
}
