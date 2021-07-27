#[cfg(test)]
mod randomized {
    use crate::factories::int_parabola;
    use crate::init;
    use soco::algorithms::online::uni_dimensional::randomized::randomized;
    use soco::problem::{Online, SimplifiedSmoothedConvexOptimization};

    #[test]
    fn _1() {
        init();

        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end: 1,
            bounds: vec![2],
            switching_cost: vec![1.],
            hitting_cost: int_parabola(),
        };
        let mut o = Online { p, w: 0 };
        o.verify().unwrap();

        let result = o.stream(&randomized, |_, _| false, ()).unwrap();
        result.0.verify(o.p.t_end, &o.p.bounds).unwrap();
    }

    #[test]
    fn _2() {
        init();

        let p = SimplifiedSmoothedConvexOptimization {
            d: 1,
            t_end: 1,
            bounds: vec![2],
            switching_cost: vec![1.],
            hitting_cost: int_parabola(),
        };
        let mut o = Online { p, w: 0 };
        o.verify().unwrap();

        let t_end = 2;
        let result = o.offline_stream(&randomized, t_end, ()).unwrap();
        result.0.verify(t_end, &o.p.bounds).unwrap();
    }
}
