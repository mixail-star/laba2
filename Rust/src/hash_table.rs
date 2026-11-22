//! Хеш-таблица с открытой адресацией (HashTable.h / HashTable.cpp).
//!
//! Линейное пробирование или двойное хеширование, ленивое удаление
//! (`Deleted`), рехеш при коэффициенте заполнения >= 0.7.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbingMethod {
    Linear,
    DoubleHashing,
}

enum Entry<K, V> {
    Empty,
    Occupied { key: K, value: V },
    Deleted,
}

/// Замена `std::hash<K>{}(key)`. `DefaultHasher::new()` детерминирован,
/// поэтому результаты воспроизводимы между запусками.
pub fn hash_of<K: Hash + ?Sized>(key: &K) -> u64 {
    let mut h = DefaultHasher::new();
    key.hash(&mut h);
    h.finish()
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn next_prime(mut n: usize) -> usize {
    while !is_prime(n) {
        n += 1;
    }
    n
}

fn empty_table<K, V>(capacity: usize) -> Vec<Entry<K, V>> {
    std::iter::repeat_with(|| Entry::Empty)
        .take(capacity)
        .collect()
}

/// Что лежит в ячейке относительно искомого ключа (чтобы не держать
/// заимствование таблицы во время записи).
enum Slot {
    SameKey,
    OtherKey,
    Deleted,
    Empty,
}

pub struct HashTable<K, V> {
    table: Vec<Entry<K, V>>,
    capacity: usize,
    current_size: usize,
    total_collisions: usize,
    method: ProbingMethod,
}

impl<K: Hash + Eq, V> HashTable<K, V> {
    pub fn new(initial_capacity: usize, method: ProbingMethod) -> Self {
        let capacity = next_prime(initial_capacity);
        Self {
            table: empty_table(capacity),
            capacity,
            current_size: 0,
            total_collisions: 0,
            method,
        }
    }

    fn hash1(&self, key: &K) -> usize {
        (hash_of(key) % self.capacity as u64) as usize
    }

    fn hash2(&self, key: &K) -> usize {
        // Не должна возвращать 0, поэтому берём простое число и вычитаем остаток.
        let prime: u64 = if self.capacity > 7 { 7 } else { 3 };
        (prime - hash_of(key) % prime) as usize
    }

    fn get_index(&self, key: &K, attempt: usize) -> usize {
        let h1 = self.hash1(key);
        match self.method {
            ProbingMethod::Linear => (h1 + attempt) % self.capacity,
            ProbingMethod::DoubleHashing => (h1 + attempt * self.hash2(key)) % self.capacity,
        }
    }

    fn rehash(&mut self) {
        self.capacity = next_prime(self.capacity * 2);
        let old = std::mem::replace(&mut self.table, empty_table(self.capacity));
        self.current_size = 0;

        for entry in old {
            if let Entry::Occupied { key, value } = entry {
                self.insert(key, value);
            }
        }
    }

    /// Вставка или обновление значения. `false` — если свободной ячейки нет.
    pub fn insert(&mut self, key: K, value: V) -> bool {
        if self.load_factor() >= 0.7 {
            self.rehash();
        }

        let mut first_deleted: Option<usize> = None;

        for attempt in 0..self.capacity {
            let idx = self.get_index(&key, attempt);

            let slot = match &self.table[idx] {
                Entry::Occupied { key: k, .. } if *k == key => Slot::SameKey,
                Entry::Occupied { .. } => Slot::OtherKey,
                Entry::Deleted => Slot::Deleted,
                Entry::Empty => Slot::Empty,
            };

            match slot {
                Slot::SameKey => {
                    if let Entry::Occupied { value: v, .. } = &mut self.table[idx] {
                        *v = value; // обновление значения
                    }
                    return true;
                }
                Slot::OtherKey => self.total_collisions += 1,
                Slot::Deleted => {
                    first_deleted.get_or_insert(idx);
                }
                Slot::Empty => {
                    let target = first_deleted.unwrap_or(idx);
                    self.table[target] = Entry::Occupied { key, value };
                    self.current_size += 1;
                    return true;
                }
            }
        }

        if let Some(target) = first_deleted {
            self.table[target] = Entry::Occupied { key, value };
            self.current_size += 1;
            return true;
        }

        false
    }

    pub fn find(&self, key: &K) -> Option<&V> {
        for attempt in 0..self.capacity {
            match &self.table[self.get_index(key, attempt)] {
                Entry::Empty => return None,
                Entry::Occupied { key: k, value } if k == key => return Some(value),
                _ => {}
            }
        }
        None
    }

    pub fn remove(&mut self, key: &K) -> bool {
        for attempt in 0..self.capacity {
            let idx = self.get_index(key, attempt);
            match &self.table[idx] {
                Entry::Empty => return false,
                Entry::Occupied { key: k, .. } if k == key => {
                    self.table[idx] = Entry::Deleted;
                    self.current_size -= 1;
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub fn size(&self) -> usize {
        self.current_size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn collisions(&self) -> usize {
        self.total_collisions
    }

    pub fn load_factor(&self) -> f64 {
        self.current_size as f64 / self.capacity as f64
    }

    pub fn clear(&mut self) {
        self.table = empty_table(self.capacity);
        self.current_size = 0;
        self.total_collisions = 0;
    }
}

impl<K: Hash + Eq, V> Default for HashTable<K, V> {
    /// Как в оригинале: ёмкость 101, линейное пробирование.
    fn default() -> Self {
        Self::new(101, ProbingMethod::Linear)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn both() -> [ProbingMethod; 2] {
        [ProbingMethod::Linear, ProbingMethod::DoubleHashing]
    }

    #[test]
    fn insert_find_update_remove() {
        for m in both() {
            let mut t: HashTable<i32, i32> = HashTable::new(11, m);
            assert!(t.insert(1, 10));
            assert!(t.insert(2, 20));
            assert!(t.insert(1, 11)); // обновление
            assert_eq!(t.size(), 2);
            assert_eq!(t.find(&1), Some(&11));
            assert_eq!(t.find(&3), None);
            assert!(t.remove(&1));
            assert!(!t.remove(&1));
            assert_eq!(t.find(&1), None);
            assert_eq!(t.find(&2), Some(&20));
            assert_eq!(t.size(), 1);
        }
    }

    #[test]
    fn rehash_keeps_all_keys() {
        for m in both() {
            let mut t: HashTable<String, usize> = HashTable::new(5, m);
            let start_cap = t.capacity();
            for i in 0..1000 {
                assert!(t.insert(format!("k{i}"), i));
            }
            assert!(t.capacity() > start_cap);
            assert!(t.load_factor() < 0.7);
            for i in 0..1000 {
                assert_eq!(t.find(&format!("k{i}")), Some(&i));
            }
            assert_eq!(t.size(), 1000);
        }
    }

    #[test]
    fn deleted_slots_are_reused_and_do_not_break_probing() {
        let mut t: HashTable<i32, i32> = HashTable::new(11, ProbingMethod::Linear);
        for k in 0..6 {
            t.insert(k, k);
        }
        for k in 0..3 {
            assert!(t.remove(&k));
        }
        for k in 3..6 {
            assert_eq!(t.find(&k), Some(&k));
        }
        for k in 100..103 {
            assert!(t.insert(k, k));
        }
        assert_eq!(t.size(), 6);
    }

    #[test]
    fn clear_resets() {
        let mut t: HashTable<i32, i32> = HashTable::default();
        t.insert(1, 1);
        t.clear();
        assert_eq!(t.size(), 0);
        assert_eq!(t.collisions(), 0);
        assert_eq!(t.find(&1), None);
    }
}
