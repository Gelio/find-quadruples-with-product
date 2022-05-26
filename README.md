# Find quadruples with matching product challenge solution

A solution to the following algorithmic challenge:

> Given an array of integers A, count the quadruples of indices i < j < k < l
> for which A\[i\] \* A\[j\] \* A\[k\] = A\[l\]

## Examples

For `A = [1, 2, 3, 6, 2, 6, 1, 24, 5]`, the result is 5:

- 1 \* 2 \* 3 = 6 (twice, for l = 3 or l = 5)
- 1 \* 3 \* 2 = 6
- 2 \* 6 \* 2 = 24
- 2 \* 2 \* 6 = 24

## Solutions

1. Naive, O(n^4)

   Looks through all possible quadruples of indices i < j < k < l and tests the
   equation.

   There is also a paralellized variant of this solution that uses
   [rayon](https://crates.io/crates/rayon) to look through multiple `i` in
   parallel.

2. With pruning, O(n^3)

   Prepares a hashmap whose keys are array values and hashmap values are sets
   of indices on which those array values appear.

   It then loops through i < j < k indices, computes the product, and counts
   the number of indices for which the value is equal to the product. Those
   indices must be greater than k.

   Instead of filtering those sets of indices, it prunes them as the iteration
   progresses. The outermost loop iterates over k. When the iteration starts
   for k, we are no longer interested in indices l <= k, because l must be
   greater than k. Thus, the index k is removed from the hashmap (technically,
   from the set of indices for which the value is equal to A\[k\]) at the
   beginning of the iteration for index k.

3. With division, O(n^2 \* log n)

   Instead of computing A\[i\] \* A\[j\] \* A\[k\], the algorithm precomutes
   the products of A\[i\] \* A\[j\], saving the j indices in a hashmap, similar
   to the hashmap from the pruning solution, but using the subproduct instead
   of just array values.

   Then, the algorithm loops through all k < l indices and computes A\[l\] / A\[k\] = res.
   It can then look at the indices in the hashmap for the found result `res`.
   It filters the indices `j` which are smaller than `k`.

## Benchmark

[Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html)
is used to benchmark the solutions.

To run the benchmark, run:

```sh
cargo bench --bench solution -- --sample-size 10
```

The lower `--sample-size` is to prevent running the naive algorithm many times,
since it is slow and takes a long time in general for large outputs.

Here is the Criterion benchmark result comparing the 3 solutions:

![Benchmark result showing the execution time for the 3 solutions for arrays of
length up to 2000](https://user-images.githubusercontent.com/889383/170463142-f9b4b615-c310-4e48-9c7f-8ece6044f505.png)

The different magnitudes of time complexity of the 4 solutions are clearly
visible. The naive solution scales much worse than the pruning one, and that
one is worse than the division one. An interesting observation is that
paralellizing the naive solution yields similar results to reducing the
time-complexity to O(n^3).

## Running the tests

To run the unit tests, run:

```sh
cargo test
```
