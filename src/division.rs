use std::collections::HashMap;

pub fn solve(arr: &[i64]) -> usize {
    // Keys are A[i] * A[j]
    // Values are the j's (the greater indices)
    let mut value_index_map: HashMap<i64, Vec<usize>> = HashMap::new();
    for j in 1..arr.len() {
        for i in 0..j {
            value_index_map.entry(arr[i] * arr[j]).or_default().push(j);
        }
    }

    let mut result = 0;

    for k in 2..arr.len() {
        if arr[k] == 0 {
            // NOTE: A[i] * A[j] * A[k] = 0 for all (i, j), so we accept
            // all l for which A[l] = 0 as valid results
            result += arr.into_iter().skip(k + 1).filter(|x| **x == 0).count();
            continue;
        }

        for l in (k + 1)..arr.len() {
            // SAFETY: arr[k] == 0 is handled above, so this will never panic
            if arr[l] % arr[k] != 0 {
                // NOTE: if the division is not an integer, it will never appear in the
                // value_index_map, so we can skip this case
                continue;
            }

            let division_result = arr[l] / arr[k];

            if let Some(indices) = value_index_map.get(&division_result) {
                let matching_indices = match indices.binary_search(&k) {
                    Ok(some_k_index) => {
                        let mut smaller_than_k_index = some_k_index;
                        // NOTE: since binary_search can return any index if there are multiple
                        // of `k`s in the array, we need to look for the smallest index of `k` and
                        // return it
                        while smaller_than_k_index > 0 && indices[smaller_than_k_index] == k {
                            smaller_than_k_index -= 1;
                        }

                        if smaller_than_k_index == 0 && indices[smaller_than_k_index] == k {
                            // NOTE: if there are no smaller indices than k, return 0
                            0
                        } else {
                            // NOTE: if there are smaller indices than k, count them
                            smaller_than_k_index + 1
                        }
                    }
                    Err(would_be_k_index) => {
                        // NOTE: if there is no `k` in the array and it would be inserted at `i`,
                        // this means there is `i` numbers that are smaller than `k`
                        would_be_k_index
                    }
                };

                result += matching_indices;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use rand::{prelude::SmallRng, Rng, SeedableRng};

    use crate::naive;

    use super::*;

    #[test]
    fn works_small() {
        let arr = [1, 2, 3, 6, 2, 6, 1, 24, 5];
        assert_eq!(solve(&arr), 5);
    }

    #[test]
    fn works_large() {
        let arr = [
            23, -47, 21, 32, -7, -3, 38, -38, 39, 1, 24, 16, -17, -17, 13, 43, -9, 39, -38, -30,
            39, -15, -45, -28, -14, 27, 42, -29, 42, 10, 43, -7, 14, -41, 45, 42, -15, -21, -3,
            -14, -23, 6, -5, 4, -50, 30, -25, 50, 37, -15,
        ];
        assert_eq!(solve(&arr), 6);
    }

    #[test]
    fn results_match_naive() {
        let mut rng = SmallRng::seed_from_u64(1337);
        let arr: Vec<i64> = (0..10).map(|_| rng.gen_range(-50..=50)).collect();
        assert_eq!(solve(&arr), naive::solve(&arr));
    }
}
