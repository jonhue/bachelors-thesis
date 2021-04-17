use ordered_float::OrderedFloat;
use pathfinding::dijkstra;

use crate::lib::types::{DiscreteHomProblem, DiscreteSchedule, HomProblem};
use crate::lib::utils::ipos;

// Represents a vertice `v_{t, j}` where the `t ~ time` and `j ~ #servers`.
type Vertice = (i32, i32);
// Represents the length (cost) of an edge
type Cost = OrderedFloat<f64>;
// Maps a vertice to all its neighbors with some cost.
type Neighbors = Box<dyn Fn(&Vertice) -> Vec<(Vertice, Cost)>>;

static eps: f64 = 1.;

impl<'a> DiscreteHomProblem<'a> {
    pub fn transform(&self) -> DiscreteHomProblem<'a> {
        let m = (2 as i32).pow((self.m as f64).log(2.).ceil() as u32);
        let f = Box::new(move |t, x| {
            if x <= self.m {
                (self.f)(t, x)
            } else {
                Some(
                    x as f64
                        * ((self.f)(t, self.m).expect("f should be total on its domain")
                            + eps),
                )
            }
        });

        return HomProblem {
            m: m,
            t_end: self.t_end,
            beta: self.beta,
            f: &f,
        };
    }
}

pub fn alg1<'a>(mut p: &DiscreteHomProblem<'a>) -> DiscreteSchedule {
    if (p.m as f64).log(2.) % 1. == 0. {
        p = &p.transform();
    }

    let neighbors = build_neighbors(p);

    let initial_neighbors = select_initial_neighbors(p, neighbors);
    let mut xs = find_schedule(p, initial_neighbors);

    let k_init = (p.m as f64).log(2.).floor() as u32 - 3;
    for k in k_init..0 {
        let next_neighbors = select_next_neighbors(p, &xs, neighbors, k);
        xs = find_schedule(p, next_neighbors);
    }
    return xs;
}

fn build_neighbors<'a>(p: &'a DiscreteHomProblem) -> &'a Neighbors {
    return &Box::new(move |&(t, i): &Vertice| {
        if t == p.t_end {
            vec![((t + 1, 0), OrderedFloat(0.))]
        } else {
            vec![0; p.m as usize]
                .iter()
                .enumerate()
                .map(|(j, _)| {
                    ((t + 1, j as i32), build_cost(p, t, i, j as i32))
                })
                .collect()
        }
    });
}

fn build_cost(p: &DiscreteHomProblem, t: i32, i: i32, j: i32) -> Cost {
    return OrderedFloat(
        p.beta * ipos(i - j) as f64
            + (p.f)(t, i).expect("f should be total on its domain"),
    );
}

fn select_initial_neighbors<'a>(
    p: &DiscreteHomProblem,
    neighbors: &'a Neighbors,
) -> &'a Neighbors {
    let acceptable_successors: Vec<i32> = (0..4).map(|e| e * p.m / 4).collect();
    return select_neighbors(
        neighbors,
        Box::new(move |&(_, j)| acceptable_successors.contains(&j)),
    );
}

fn select_next_neighbors<'a>(
    p: &DiscreteHomProblem,
    xs: &DiscreteSchedule,
    neighbors: &'a Neighbors,
    k: u32,
) -> &'a Neighbors {
    let acceptable_successors: Vec<Vec<i32>> = (1..p.t_end)
        .map(|t| {
            (-2..2)
                .map(|e| xs[t as usize - 1] + e * (2 as i32).pow(k))
                .collect()
        })
        .collect();
    return select_neighbors(
        neighbors,
        Box::new(move |&(t, j)| {
            acceptable_successors[t as usize - 1].contains(&j)
        }),
    );
}

fn select_neighbors<'a>(
    neighbors: &'a Neighbors,
    is_acceptable_successor: Box<dyn Fn(&Vertice) -> bool>,
) -> &'a Neighbors {
    return &Box::new(move |&(t, i): &Vertice| {
        neighbors(&(t, i))
            .iter()
            .map(|&x| x) // TODO: why is this necessary?
            .filter(|&(v, _)| is_acceptable_successor(&v))
            .collect()
    });
}

fn find_schedule(
    p: &DiscreteHomProblem,
    neighbors: &Neighbors,
) -> DiscreteSchedule {
    let result = dijkstra(&(0, 0), neighbors, |&(t, j): &Vertice| {
        t == p.t_end && j == 0
    });
    let (xs, _) = result.expect("there should always be a path");
    return xs.into_iter().map(|(_, j)| j).collect();
}
