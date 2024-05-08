use crate::heap;
use crate::heap::{HeapType, IHeap};

fn heap_sort<T: Ord + Copy>(vec: Vec<T>) -> Vec<T> {
    let mut heap = heap::Heap::build(vec,HeapType::Max);
    let mut res = vec![];
    while let Some(item) = heap.pop() {
        res.push(item);
    }

    res
}

#[test]
fn test_heap_sort() {
    let source = vec![3,2,1,4,5,7];
    let target = heap_sort(source);
    println!("{:?}", target);
    assert_eq!(target, vec![7,5,4,3,2,1])
}