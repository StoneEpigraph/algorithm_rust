#[derive(Debug, PartialEq)]
pub enum HeapType {
    Min,
    Max,
}

pub trait IHeap<T: Ord + Copy> {
    fn new(heap_type: HeapType) -> Self;
    fn build(vec: Vec<T>, heap_type: HeapType) -> Self;
    fn fast_build(vec: Vec<T>, heap_type: HeapType) -> Self;
    fn get_last_parent(&self) -> Option<usize>;
    fn left_child(index: usize) -> usize;
    fn right_child(index: usize) -> usize;
    fn father(index: usize) -> Option<usize>;
    fn get_most(&self, index: usize) -> Option<usize>;
    fn is_empty(&self) -> bool;
    fn is_leaf(&self, index: usize) -> bool;
    fn cmp(&self, k1: usize, k2: usize) -> bool;
    fn insert(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn sink_down(&mut self, index: usize);
    fn swim_up(&mut self, index: usize);
}
#[derive(Debug, PartialEq)]
pub struct Heap<T: Ord + Copy> {
    pub arr: Vec<T>,
    pub heap_type: HeapType
}

impl<T: Ord + Copy> IHeap<T> for Heap<T> {

    fn new(heap_type: HeapType) -> Self {
        Self {
            arr: vec![],
            heap_type,
        }
    }
    fn build(mut vec: Vec<T>, heap_type: HeapType) -> Self {
        let mut heap = Heap::new(heap_type);
        while let Some(value) = vec.pop() {
            heap.insert(value);
        }
        heap
    }

    fn fast_build(vec: Vec<T>, heap_type: HeapType) -> Self {
        let mut heap = Heap::new(heap_type);
        heap.arr = vec;
        match heap.get_last_parent() {
            None => heap,
            Some(father) => {
                for i in (0..=father).rev() {
                    heap.sink_down(i);
                }
                heap
            },
        }
    }

    fn get_last_parent(&self) -> Option<usize> {
        if self.arr.len() == 0 {
            None
        } else {
            Self::father(self.arr.len() - 1)
        }
    }

    fn left_child(index: usize) -> usize {
        index * 2 + 1
    }

    fn right_child(index: usize) -> usize {
        index * 2 + 2
    }

    fn father(index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / 2)
        }
    }

    fn get_most(&self, index: usize) -> Option<usize> {
        if self.is_leaf(index) {
            None
        } else if self.arr.get(Self::right_child(index)).is_none() {
            Some(Self::left_child(index))
        } else {
            match self.cmp(Self::left_child(index), Self::right_child(index)) {
                true => Some(Self::right_child(index)),
                false => Some(Self::left_child(index)),
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.arr.len() == 0
    }

    fn is_leaf(&self, index: usize) -> bool {
        self.arr.get(Self::left_child(index)).is_none()
    }

    fn cmp(&self, k1: usize, k2: usize) -> bool {
        match self.heap_type {
            HeapType::Min => {
                self.arr[k1] > self.arr[k2]
            }
            HeapType::Max => {
                self.arr[k2] > self.arr[k1]
            }
        }
    }

    fn insert(&mut self, value: T) {
        self.arr.push(value);
        let last_index = self.arr.len() - 1;
        self.swim_up(last_index);
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.arr.len() == 1 {
            let most = self.arr.pop().unwrap();
            Some(most)
        } else {
            let most = self.arr[0];
            self.arr[0] = self.arr.pop().unwrap();
            self.sink_down(0);
            Some(most)
        }
    }

    fn sink_down(&mut self, index: usize) {
        match self.get_most(index) {
            None => {}
            Some(most_index) => {
                if self.cmp(index, most_index) {
                    self.arr.swap(index, most_index);
                    self.sink_down(most_index);
                }
            }
        }
    }
    fn swim_up(&mut self, index: usize) {
        match Self::father(index) {
            Some(father) => {
                if self.cmp(father, index) {
                    self.arr.swap(father, index);
                    self.swim_up(father);
                }
            }
            _ => {}
        }
    }
}


#[test]
fn  test_heap() {
    let mut h = Heap::new(HeapType::Max);
    h.insert(1);
    h.insert(12);
    h.insert(5);
    h.insert(7);
    h.insert(3);
    h.insert(4);
    println!("{:?}", h);
}

#[test]
fn test_heap_pop() {
    let mut h = Heap::new(HeapType::Min);
    h.insert(1);
    h.insert(12);
    h.insert(5);
    h.insert(7);
    h.insert(3);
    h.insert(4);
    println!("{:?}", h.pop());
    println!("{:?}", h.pop());
    println!("{:?}", h.pop());
    println!("{:?}", h.pop());
    println!("{:?}", h);
}

#[test]
fn test_empty_heap() {
    let mut heap:Heap<i32> = Heap::new(HeapType::Max);
    assert_eq!(heap.pop(), None)
}

#[test]
fn test_build_and_fast_build() {
    // let heap1 = Heap::build(vec![3,2,1,4,5,6,7], HeapType::Max);
    let heap1 = Heap::build(vec![1,12,5,7,3,4], HeapType::Max);
    
    // let heap2 = Heap::fast_build(vec![3,2,1,4,5,6,7], HeapType::Max);
    let heap2 = Heap::build(vec![1,12,5,7,3,4], HeapType::Max);
    println!("heap1: {:?}", heap1);
    println!("heap1: {:?}", heap2);
    assert_eq!(heap1, heap2);

}