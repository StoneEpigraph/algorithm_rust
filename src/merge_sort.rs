fn merge<T: Ord + Copy>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut i = 0;  // left
    let mut j = 0;  // right

    let mut result = vec![];

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

fn merge_sort<T: Ord + Copy>(arr: Vec<T>)  -> Vec<T> {
    if arr.len() == 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    let left = arr[0 .. mid].to_vec();
    let right = arr[mid ..].to_vec();

    merge(&merge_sort(left), &merge_sort(right))
}


#[test]
fn test_merge() {
    let left = vec![1,3,5,7,9];
    let right = vec![2,4,6,8,10];
    let result = merge(&left, &right);
    println!("{:?}", result);
}

#[test]
fn test_merge_sort() {
    let arr = vec![1,3,5,7,9,2,4,6,8,10,11,1,1,1,1,2,3,4,23,1,23,1,23,1,23];
    let result = merge_sort(arr);
    println!("{:?}", result);
}