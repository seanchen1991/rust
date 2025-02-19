use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

/// A view onto a finite range of an infinitely long sequence of T.
///
/// The Ts are indexed 0..infinity. A RingBuffer begins as a view of elements
/// 0..0 (i.e. nothing). The user of the RingBuffer advances its left and right
/// position independently, although only in the positive direction, and only
/// with left <= right at all times.
///
/// Holding a RingBuffer whose view is elements left..right gives the ability to
/// use Index and IndexMut to access elements i in the infinitely long queue for
/// which left <= i < right.
pub struct RingBuffer<T> {
    data: VecDeque<T>,
    // Abstract index of data[0] in the infinitely sized queue.
    offset: usize,
}

impl<T> RingBuffer<T> {
    pub fn new() -> Self {
        RingBuffer { data: VecDeque::new(), offset: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, value: T) -> usize {
        let index = self.offset + self.data.len();
        self.data.push_back(value);
        index
    }

    pub fn advance_left(&mut self) {
        self.data.pop_front().unwrap();
        self.offset += 1;
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn index_of_first(&self) -> usize {
        self.offset
    }

    pub fn first(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }

    pub fn last(&self) -> Option<&T> {
        self.data.back()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index.checked_sub(self.offset).unwrap()]
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index.checked_sub(self.offset).unwrap()]
    }
}
