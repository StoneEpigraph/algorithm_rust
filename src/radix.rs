fn radix_sort_helper(nums: &mut [usize]) {
    if nums.len() == 1 {
        return;
    }
    let max_num = match nums.iter().max() {
        Some(&x) => x,
        None => return,
    };
    let radix = nums.len().next_power_of_two();
    let mut digit = 1;
    while digit <= max_num {
        let index_of = |x| x / digit % radix;
        let mut counter = vec![0; radix];
        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }

        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }
        println!("digit: {}, counter: {:?}", digit, counter);
        digit *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn radix_test() {
        let mut nums = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 12, 25, 33, 123, 2222, 1111];
        radix_sort_helper(&mut nums);
        assert_eq!(
            nums,
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 25, 33, 123, 1111, 2222]
        );
    }
}
