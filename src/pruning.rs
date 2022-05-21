use std::collections::{HashMap, HashSet};

type ValueIndexMap = HashMap<i64, HashSet<usize>>;

pub fn solve(arr: &[i64]) -> usize {
    let mut value_index_map: ValueIndexMap = HashMap::new();
    for (index, value) in arr.into_iter().enumerate() {
        value_index_map.entry(*value).or_default().insert(index);
    }

    let skip_index = |index: usize, value_index_map: &mut ValueIndexMap| {
        let removed = value_index_map
            .get_mut(&arr[index])
            .expect("value index map was constructed incorrectly")
            .remove(&index);

        assert!(
            removed,
            "index {} was removed from the array too quickly",
            index
        );
    };

    let mut result = 0;

    // NOTE: k must be at least 2, because for i < j < k
    skip_index(0, &mut value_index_map);
    skip_index(1, &mut value_index_map);

    for k in 2..arr.len() {
        skip_index(k, &mut value_index_map);

        for j in 0..k {
            for i in 0..j {
                let product = arr[i] * arr[j] * arr[k];

                result += value_index_map
                    .get(&product)
                    .map(|x| x)
                    .map_or(0, |indices| indices.len());
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
