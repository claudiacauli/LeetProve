use std::collections::HashMap;

/*
Assumptions: 
- 2 <= nums.length <= 10^4
- -10^9 <= nums[i] <= 10^9
- -10^9 <= target <= 10^9
- Each input would have exactly one solution
- You may not use the same element twice
- Answers can be returned in any order
*/
fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(&j) = map.get(&diff) {
            return (j, i);
        }
        map.insert(num, i);
    }
    panic!("Unreachable - No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn valid_two_sum_input() -> impl Strategy<Value = (Vec<i32>, i32)> {
        (-1_000_000_000..=1_000_000_000i32, 2..=10_000usize)
            .prop_filter("target not zero", |(target, _)| *target != 0i32)
            .prop_flat_map(|(target, len)| {
                (-1_000_000_000..=1_000_000_000i32)
                    .prop_filter("different from target", move |&num| num != target)
                    .prop_filter("diff not large", move |&num| (target - num).abs() <= 1_000_000_000)
                    .prop_flat_map(move |num| {
                        (0..len, 0..len)
                            .prop_filter("different indices", |(i, j)| i != j)
                            .prop_map(move |(i, j)| {
                                let mut nums = vec![0; len];
                                nums[i] = num;
                                nums[j] = target - num;
                                (nums, target)
                            })
                    })
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn indices_are_valid((nums, target) in valid_two_sum_input()) {
            let (i, j) = two_sum(&nums, target);
            // Property: returned indices must be in bounds
            prop_assert!(i < nums.len());
            prop_assert!(j < nums.len());
            // Property: indices must be different
            prop_assert_ne!(i, j);
            // Property: sum must equal target
            prop_assert_eq!(nums[i] + nums[j], target);
            
        }
    }
}

// fn main() {
//     println!("{:?}", two_sum(&[-1,i32::MAX], i32::MAX - 1));
// }