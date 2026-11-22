//! Двусвязный список (llist.h / llist.cpp).
//!
//! Реализован на `VecDeque`: наружу виден тот же набор операций
//! (вставка/удаление по значению, до/после элемента, с обоих концов).

use std::collections::VecDeque;
use std::fmt::{self, Display};

/// Результат `pop_value` / `pop_before` / `pop_after`.
/// `Display` выдаёт те же строки, что и оригинал.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopResult {
    /// Элемент удалён ("OK").
    Ok,
    /// Целевой элемент не найден ("NOT_FOUND").
    NotFound,
    /// Целевой найден, но соседа в нужную сторону нет ("FALSE").
    NoNeighbor,
}

impl Display for PopResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PopResult::Ok => "OK",
            PopResult::NotFound => "NOT_FOUND",
            PopResult::NoNeighbor => "FALSE",
        })
    }
}

#[derive(Debug, Default)]
pub struct List<T> {
    items: VecDeque<T>,
}

impl<T: PartialEq> List<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn push_head(&mut self, val: T) {
        self.items.push_front(val);
    }

    pub fn push_tail(&mut self, val: T) {
        self.items.push_back(val);
    }

    /// Вставить `val` перед первым вхождением `target`.
    pub fn push_before(&mut self, val: T, target: &T) -> bool {
        match self.find(target) {
            Some(i) => {
                self.items.insert(i, val);
                true
            }
            None => false,
        }
    }

    /// Вставить `val` после первого вхождения `target`.
    pub fn push_after(&mut self, val: T, target: &T) -> bool {
        match self.find(target) {
            Some(i) => {
                self.items.insert(i + 1, val);
                true
            }
            None => false,
        }
    }

    /// `None` для пустого списка (в оригинале возвращалась строка "Empty").
    pub fn pop_head(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        self.items.pop_back()
    }

    /// Удалить первое вхождение `target`.
    pub fn pop_value(&mut self, target: &T) -> PopResult {
        match self.find(target) {
            Some(i) => {
                self.items.remove(i);
                PopResult::Ok
            }
            None => PopResult::NotFound,
        }
    }

    /// Удалить элемент, стоящий перед первым вхождением `target`.
    pub fn pop_before(&mut self, target: &T) -> PopResult {
        match self.find(target) {
            None => PopResult::NotFound,
            Some(0) => PopResult::NoNeighbor,
            Some(i) => {
                self.items.remove(i - 1);
                PopResult::Ok
            }
        }
    }

    /// Удалить элемент, стоящий после первого вхождения `target`.
    pub fn pop_after(&mut self, target: &T) -> PopResult {
        match self.find(target) {
            None => PopResult::NotFound,
            Some(i) if i + 1 >= self.items.len() => PopResult::NoNeighbor,
            Some(i) => {
                self.items.remove(i + 1);
                PopResult::Ok
            }
        }
    }

    /// Позиция первого вхождения `target` (вместо указателя на узел).
    pub fn find(&self, target: &T) -> Option<usize> {
        self.items.iter().position(|x| x == target)
    }
}

impl<T: PartialEq + Display> List<T> {
    /// Аналог `list_get`: элементы от головы, каждый с пробелом впереди.
    pub fn get(&self) -> String {
        if self.items.is_empty() {
            return "Нет такой структуры".to_string();
        }
        let mut out = String::new();
        for item in &self.items {
            out.push(' ');
            out.push_str(&item.to_string());
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_of(xs: &[&str]) -> List<String> {
        let mut l = List::new();
        for x in xs {
            l.push_tail(x.to_string());
        }
        l
    }

    fn s(x: &str) -> String {
        x.to_string()
    }

    #[test]
    fn push_before_after() {
        let mut l = list_of(&["a", "c"]);
        assert!(l.push_before(s("b"), &s("c")));
        assert!(l.push_after(s("d"), &s("c")));
        assert!(l.push_before(s("z"), &s("a")));
        assert!(!l.push_after(s("q"), &s("nope")));
        assert_eq!(l.get(), " z a b c d");
    }

    #[test]
    fn pop_variants() {
        let mut l = list_of(&["a", "b", "c", "d"]);
        assert_eq!(l.pop_before(&s("a")), PopResult::NoNeighbor);
        assert_eq!(l.pop_after(&s("d")), PopResult::NoNeighbor);
        assert_eq!(l.pop_before(&s("c")), PopResult::Ok); // убрали b
        assert_eq!(l.pop_after(&s("a")), PopResult::Ok); // убрали c
        assert_eq!(l.get(), " a d");
        assert_eq!(l.pop_value(&s("zzz")), PopResult::NotFound);
        assert_eq!(l.pop_value(&s("a")), PopResult::Ok);
        assert_eq!(l.pop_head(), Some(s("d")));
        assert_eq!(l.pop_head(), None);
        assert_eq!(l.pop_tail(), None);
    }

    #[test]
    fn display_strings_match_original() {
        assert_eq!(PopResult::Ok.to_string(), "OK");
        assert_eq!(PopResult::NotFound.to_string(), "NOT_FOUND");
        assert_eq!(PopResult::NoNeighbor.to_string(), "FALSE");
    }
}
