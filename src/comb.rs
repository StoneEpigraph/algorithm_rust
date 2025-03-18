
// 使用Rust写一个梳排序算法
fn comb_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mut gap = n as f64 / 3.0;
    while gap < 1.0 {
        gap /= 1.3245;
        
        for i in (1..n).step_by(gap as usize) {
            let mut j = i + (gap as usize);
            
            while j < n {
                let temp = arr[j];
                
                while j > i && arr[j - (gap as usize)] > temp {
                    arr[j] = arr[j - (gap as usize)];
                    j -= gap as usize;
                }
                arr[j] = temp;
                j += gap as usize;
            }
            println!("{:?}", arr);
        }
    }
    insertion_sort(arr);
}

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb::comb_sort;

    #[test]
    fn test_comb_sort() {

        let mut arr = [40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        comb_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10,  11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40]);
    }
}