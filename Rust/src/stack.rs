//! Стек на односвязном списке (stack.h / stack.cpp).

use std::fmt::Display;

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { top: None }
    }

    pub fn push(&mut self, val: T) {
        let node = Box::new(Node {
            val,
            next: self.top.take(),
        });
        self.top = Some(node);
    }

    /// Аналог `stack_pop` + флага `ok`: `None`, если стек пуст.
    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            node.val
        })
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T: Display> Stack<T> {
    /// Аналог `stack_get`: элементы от вершины вниз, каждый с пробелом впереди.
    pub fn render(&self) -> String {
        if self.top.is_none() {
            return "Нет такой структуры".to_string();
        }
        let mut out = String::new();
        let mut cur = self.top.as_deref();
        while let Some(node) = cur {
            out.push(' ');
            out.push_str(&node.val.to_string());
            cur = node.next.as_deref();
        }
        out
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Итеративный Drop, чтобы длинный стек не переполнил стек вызовов.
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifo_order() {
        let mut s = Stack::new();
        s.push("a".to_string());
        s.push("b".to_string());
        assert_eq!(s.render(), " b a");
        assert_eq!(s.pop().as_deref(), Some("b"));
        assert_eq!(s.pop().as_deref(), Some("a"));
        assert_eq!(s.pop(), None);
        assert_eq!(s.render(), "Нет такой структуры");
    }

    #[test]
    fn long_stack_drops_without_overflow() {
        let mut s = Stack::new();
        for i in 0..1_000_000 {
            s.push(i);
        }
    }
}
