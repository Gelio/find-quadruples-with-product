use rayon::prelude::*;

pub fn solve(arr: &[i64]) -> usize {
    // NOTE: only parallelize the first loop, since that splits the work the most
    arr.into_par_iter()
        .enumerate()
        .map(|(i, &a_i)| {
            let mut subresult = 0;
            for (j, &a_j) in arr.into_iter().enumerate().skip(i + 1) {
                for (k, &a_k) in arr.into_iter().enumerate().skip(j + 1) {
                    let product = a_i * a_j * a_k;
                    for (_l, &a_l) in arr.into_iter().enumerate().skip(k + 1) {
                        if product == a_l {
                            subresult += 1;
                        }
                    }
                }
            }

            subresult
        })
        .sum()
}

#[cfg(test)]
mod tests {
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
}
