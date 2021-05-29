# List of Algorithms

| Name                                                                                                           | Variant | Multi-Dimensional? | Integral? | Online? | Approximation/Competitiveness* | Complexity** | Notes |
| -------------------------------------------------------------------------------------------------------------- | ------- | ------------------ | --------- | ------- | ------------------------------ | ------------ | ----- |
| [Backward-Recurrent Capacity Provisioning](offline/uni_dimensional/capacity_provisioning.rs) [1]               | 2       | ❌                 | ❌        | ❌      | optimal                        |              |
| [Forward-Recurrent Capacity Provisioning](offline/uni_dimensional/capacity_provisioning.rs) [*]                | 2       | ❌                 | ✅        | ❌      | optimal                        |              |
| [Graph-Based Optimal Algorithm](offline/uni_dimensional/optimal_graph_search.rs) [4]                           | 2       | ❌                 | ✅        | ❌      | optimal                        | O(T log m)   |
| [Graph-Based Optimal Algorithm](offline/multi_dimensional/optimal_graph_search.rs) [8]                         | 2       | ✅                 | ✅        | ❌      | optimal                        |              |
| [Graph-Based Approximation Algorithm](offline/multi_dimensional/approx_graph_search.rs) [8]                    | 2       | ✅                 | ✅        | ❌      | (2𝛾 - 1)-approximation         |              | 𝛾 > 0 |
| [Fractional Lazy Capacity Provisioing](online/uni_dimensional/lazy_capacity_provisioing/fractional.rs) [1]     | 2       | ❌                 | ❌        | ✅      | 3-competitive                  |              |
| [Integral Lazy Capacity Provisioing](online/uni_dimensional/lazy_capacity_provisioing/integral.rs) [4]         | 2       | ❌                 | ✅        | ✅      | 3-competitive                  |              |
| [Memoryless Algorithm](online/uni_dimensional/memoryless.rs) [2]                                               | 2       | ❌                 | ❌        | ✅      | 3-competitive                  |              |
| [Probabilistic Algorithm](online/uni_dimensional/probabilistic.rs) [2]                                         | 2       | ❌                 | ❌        | ✅      | 2-competitive                  |              |
| [Randomized Integral Relaxation](online/uni_dimensional/randomized.rs) [4]                                     | 2       | ❌                 | ✅        | ✅      | 2-competitive                  |              |
| [Randomly Biased Greedy](online/uni_dimensional/randomly_biased_greedy.rs) [3]                                 | 1       | ❌                 | ❌        | ✅      | 2-competitive                  |              |
| [Lazy Budgeting for SLO](online/multi_dimensional/lazy_budgeting/smoothed_load_optimization.rs) [7]            | 3       | ✅                 | ✅        | ✅      | 2d-competitive                 |              |
| [Randomized Lazy Budgeting for SLO](online/multi_dimensional/lazy_budgeting/smoothed_load_optimization.rs) [7] | 3       | ✅                 | ✅        | ✅      | (e / (e - 1))d-competitive     |              |
| [Lazy Budgeting for SBLO](online/multi_dimensional/lazy_budgeting/smoothed_balanced_load_optimization.rs) [8]  | 4       | ✅                 | ✅        | ✅      | (2d + 1 + ε)-competitive       |              | ε > 0 |
| [Online Balanced Descent (meta algorithm)](online/multi_dimensional/online_balanced_descent/meta.rs) [5]       | 1       | ✅                 | ❌        | ✅      |                                |              | Ω(m^{-2/3})-competitive for m-strongly convex hitting costs and l2-squared switching costs |
| [Primal Online Balanced Descent](online/multi_dimensional/online_balanced_descent/primal.rs) [5]               | 1       | ✅                 | ❌        | ✅      | 3+O(1/𝛼)-competitive           |              | given competitiveness is for the l2-norm and locally 𝛼-polyhedral hitting costs, O(sqrt(d))-competitive for the l1-norm; mirror map must be m-strongly convex and M-Lipschitz smooth in the switching cost norm
| [Dual Online Balanced Descent](online/multi_dimensional/online_balanced_descent/dual.rs) [5]                   | 1       | ✅                 | ❌        | ✅      |                                |              | mirror map must be m-strongly convex and M-Lipschitz smooth in the switching cost norm |
| [Greedy Online Balanced Descent](online/multi_dimensional/online_balanced_descent/greedy.rs) [6]               | 1       | ✅                 | ❌        | ✅      | O(1/sqrt(m))-competitive       |              | for m-quasiconvex hitting costs and l2-squared switching costs |
| [Regularized Online Balanced Descent](online/multi_dimensional/online_balanced_descent/regularized.rs) [6]     | 1       | ✅                 | ❌        | ✅      | O(1/sqrt(m))-competitive       |              | for m-strongly convex and differentiable hitting costs and switching costs modeled as the Bregman divergence where the potential function is 𝛼-strongly convex, 𝛽-strongly smooth, differentiable, and its Fenchel Conjugate is differentiable; Ω(1/m)-competitive for m-quasiconvex hitting costs and l2-squared switching costs |

\* If online, the competitive ratio describes how much worse the algorithm performs compared to an optimal offline algorithm in the worst case.

\*\* If online, complexity is with respect to one iteration of the algorithm.

### Problem Variants

1. Smoothed Convex Optimization
2. Simplified Smoothed Convex Optimization
3. Smoothed Load Optimization
4. Smoothed Balanced-Load Optimization

Above order is from most general to most specific.

### Optimal Competitiveness

| Variant | Multi-Dimensional? | Integral? | Deterministic? | Memoryless? | Optimal Competitiveness              | Notes |
| ------- | ------------------ | --------- | -------------- | ----------- | ------------------------------------ | ----- |
| 1       | ❌                 | ❌        | ✅             | ✅          | 3-competitive                        |
| 1       | ❌                 | ❌        | ✅             | ❌          | 2-competitive                        |
| 1       | ❌                 | ✅        | ✅             | ❌          | 3-competitive                        |
| 1       | ❌                 | ✅        | ❌             | ❌          | 2-competitive                        |
| 2, 3    | ✅                 | ✅        | ✅             | ❌          | 2d-competitive                       |
| 1       | ✅                 | ❌        | ❌             | ❌          | O(1/sqrt(m))-competitive as m to 0^+ | for m-strongly convex hitting costs and l2-squared switching costs |

### References

1. Minghong Lin and Adam Wierman and Lachlan L. H. Andrew and Eno Thereska. _Dynamic right-sizing for power-proportional data centers_. 2011.
2. Nikhil Bansal and Anupam Gupta and Ravishankar Krishnaswamy and Kirk Pruhs and Kevin Schewior and Cliff Stein. _A 2-Competitive Algorithm For Online Convex Optimization With Switching Costs_. 2015.
3. Nikhil Bansal and Anupam Gupta and Ravishankar Krishnaswamy and Kirk Pruhs and Kevin Schewior and Cliff Stein. _A 2-Competitive Algorithm For Online Convex Optimization With Switching Costs_. 2015.
4. Susanne Albers and Jens Quedenfeld. _Optimal Algorithms for Right-Sizing Data Centers_. 2018.
5. Niangjun Chen and Gautam Goel and Adam Wierman. _Smoothed Online Convex Optimization in High Dimensions via Online Balanced Descent_. 2018.
6. Gautam Goel and Yiheng Lin and Haoyuan Sun and Adam Wierman. _Beyond Online Balanced Descent: An Optimal Algorithm for Smoothed Online Optimization_. 2019.
7. Susanne Albers and Jens Quedenfeld. _Algorithms for Energy Conservation in Heterogeneous Data Centers_. 2021.
8. Susanne Albers and Jens Quedenfeld. _Algorithms for Right-Sizing Heterogeneous Data Centers_. 2021.
