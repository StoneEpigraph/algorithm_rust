use std::fmt::Debug;

struct Bucket<H, T> {
    hasher: H,
    values: Vec<T>,
}

impl<H, T> Bucket<H, T> {
    fn new(hasher: H, values: T) -> Bucket<H, T> {
        Bucket {
            hasher: hasher,
            values: vec![values],
        }
    }
}

fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
where
    H: Ord,
    T: Ord + Clone + Debug,
    F: Fn(&T) -> H,
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();
    for val in nums.iter() {
        let hasher = hasher(&val);

        match buckets.binary_search_by(|bct| bct.hasher.cmp(&hasher)) {
            Ok(idx) => buckets[idx].values.push(val.clone()),
            Err(idx) => buckets.insert(idx, Bucket::new(hasher, val.clone())),
        }
    }

    let ret = buckets
        .into_iter()
        .flat_map(|mut bucket| {
            bucket.values.sort();
            bucket.values
        })
        .collect::<Vec<T>>();

    nums.clone_from_slice(&ret);
}

fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {
        return;
    }

    let max_bkt_num = nums.iter().max().unwrap() + 1;
    let mut counter = vec![0; max_bkt_num];
    for &v in nums.iter() {
        counter[v] += 1;
        // println!("counter: {:?}, v: {}", counter, v)
    }

    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bucket_sort() {
        let mut nums = vec![3, 2, 1, 5, 4, 6, 7, 8, 9, 10];
        bucket_sort(&mut nums, |x| x / 3);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut nums = vec![3, 2, 1, 10, 9, 8, 7, 6, 5, 4];
        bucket_sort(&mut nums, |x| x / 3);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        bucket_sort(&mut nums, |x| x / 3);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    #[test]
    fn test_counting_sort() {
        let mut nums = vec![3, 2, 1, 5, 4, 6, 7, 8, 9, 10];
        counting_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut nums = vec![3, 2, 1, 10, 9, 8, 7, 6, 5, 4];
        counting_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        counting_sort(&mut nums);
        assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
