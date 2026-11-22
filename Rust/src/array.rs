//! Динамический массив (array.h / array.cpp).
//! Рост ёмкости и сдвиги элементов делает `Vec`.

use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub struct Array<T> {
    data: Vec<T>,
}

impl<T> Array<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
    }

    /// Вставка по индексу; `idx == len` допустим (как в оригинале).
    pub fn push_at(&mut self, idx: usize, val: T) -> bool {
        if idx > self.data.len() {
            return false;
        }
        self.data.insert(idx, val);
        true
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }

    pub fn del(&mut self, idx: usize) -> bool {
        if idx >= self.data.len() {
            return false;
        }
        self.data.remove(idx);
        true
    }

    pub fn set(&mut self, idx: usize, val: T) -> bool {
        match self.data.get_mut(idx) {
            Some(slot) => {
                *slot = val;
                true
            }
            None => false,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T: Display> Array<T> {
    /// Аналог `arr_print`: `[a, b, c]` или сообщение для пустого массива.
    pub fn print(&self) -> String {
        if self.data.is_empty() {
            return "Нет такой структуры".to_string();
        }
        let items: Vec<String> = self.data.iter().map(|x| x.to_string()).collect();
        format!("[{}]", items.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ops() {
        let mut a = Array::new();
        assert_eq!(a.print(), "Нет такой структуры");
        a.push("x".to_string());
        a.push("z".to_string());
        assert!(a.push_at(1, "y".to_string()));
        assert!(!a.push_at(5, "bad".to_string()));
        assert_eq!(a.print(), "[x, y, z]");
        assert!(a.set(0, "w".to_string()));
        assert!(a.del(1));
        assert!(!a.del(9));
        assert_eq!(a.print(), "[w, z]");
        assert_eq!(a.get(1).map(String::as_str), Some("z"));
        assert_eq!(a.get(2), None);
    }
}
