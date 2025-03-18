
fn _bubble_sort3(arr: &mut [i32]) {
    let mut compare = true;
    let mut len = arr.len() - 1;
    
    while len > 0 && compare {
        compare = false;
        
        for i in 0..len {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                compare = true;
            }
        }
        len -= 1;
    }
}

fn cocktail_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    let mut bubble = true;
    let len = arr.len();
    
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;
            
            for j in i..(len - i - 1) {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    bubble = true;
                }
            }
            
            for j in ((i + 1)..=(len -i -1)).rev() {
                if arr[j] < arr[j - 1] {
                    arr.swap(j - 1, j);
                    bubble = true;
                }
            }
        } else {
            break;
        }
        println!("{:?}", arr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort3() {
        let mut arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        cocktail_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
