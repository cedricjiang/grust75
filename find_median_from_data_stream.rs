use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    small_max_heap: BinaryHeap<i32>,
    large_min_heap: BinaryHeap<Reverse<i32>>,
    even: bool,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            small_max_heap: BinaryHeap::new(),
            large_min_heap: BinaryHeap::new(),
            even: true,
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.even {
            self.large_min_heap.push(Reverse(num));
            if let Some(Reverse(min)) = self.large_min_heap.pop() {
                self.small_max_heap.push(min);
            }
        } else {
            self.small_max_heap.push(num);
            if let Some(max) = self.small_max_heap.pop() {
                self.large_min_heap.push(Reverse(max));
            }
        }

        self.even = !self.even;
    }

    fn find_median(&self) -> f64 {
        if self.even {
            (*self.small_max_heap.peek().unwrap() + self.large_min_heap.peek().unwrap().0) as f64
                / 2.0
        } else {
            *self.small_max_heap.peek().unwrap() as f64
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */