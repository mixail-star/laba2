//! Очередь FIFO (queue.h / queue.cpp) поверх `VecDeque`.

use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.items.push_back(val);
    }

    /// `None`, если очередь пуста (вместо `ok = false`).
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    /// O(1), в оригинале `queue_size` был O(n).
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_order() {
        let mut q = Queue::new();
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.len(), 3);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
        assert_eq!(q.pop(), None);
        assert!(q.is_empty());
    }
}
