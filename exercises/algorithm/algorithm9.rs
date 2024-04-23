/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DON

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + std::fmt::Display,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + std::fmt::Display,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // omg it already has an element
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
// don't need to use recursive!
    pub fn add(&mut self, value: T) {
        //TODO
        self.count+=1;
        self.items.push(value);
        self.heapify_up(self.count);
        self.print_items();
        // let mut curid = self.count;
        // todo!();
        // while !(self.comparator)(&self.items[self.parent_idx(curid)], &self.items[curid]) {
        //     self.items.swap(curid/2, curid);
        //     curid = self.parent_idx(curid);
        // }
    }

    fn heapify_up(&mut self, idx: usize) {
        if idx > self.count {
            return;
        }
        let mut curid = idx;
        while self.parent_idx(curid) > 0 {
            let parent_idx = self.parent_idx(curid);
            if !(self.comparator)(&self.items[parent_idx], &self.items[curid]) {
                self.items.swap(curid, parent_idx);
            }
            curid = parent_idx;
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        if idx > self.count {
            return;
        }
        let mut curid = idx;
        while self.children_present(curid) {
            let scid = self.smallest_child_idx(curid);
            if !(self.comparator)(&self.items[curid], &self.items[scid]) {
                self.items.swap(curid, scid);
            }
            curid = scid;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let lcid = self.left_child_idx(idx);
        let rcid = self.right_child_idx(idx);
        if rcid > self.count {
            lcid
        } else if (self.comparator)(&self.items[lcid], &self.items[rcid]) {
            lcid
        } else {
            rcid
        }
    }

    fn print_items(&self) {
        for i in self.items.iter() {
            print!("{} ", i);
        }
        println!();
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::fmt::Display,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone + std::fmt::Display,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.is_empty() {
            return None;
        }

        //dbg
        self.print_items();

        let res = self.items[1].clone();

        self.items.swap(1, self.count);
        self.items.pop();
        self.count-=1;
        self.heapify_down(1);
        return Some(res);
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Display,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Display,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}