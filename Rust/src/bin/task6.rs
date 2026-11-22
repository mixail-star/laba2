//! Задание 6: хеш-таблица с фильтром Блума перед поиском.

use labs::hash_table::{hash_of, HashTable, ProbingMethod};
use std::cell::Cell;
use std::hash::Hash;
use std::time::Instant;

struct BloomFilter {
    bits: Vec<bool>,
    num_hashes: usize,
}

impl BloomFilter {
    fn new(size: usize, hash_count: usize) -> Self {
        Self {
            bits: vec![false; size],
            num_hashes: hash_count,
        }
    }

    fn nth_hash<K: Hash>(&self, key: &K, i: usize) -> usize {
        let h1 = hash_of(key);
        let mut h2 = (h1 >> 16) ^ h1.wrapping_mul(0x45d9f3b);
        if h2 == 0 {
            h2 = 1;
        }
        (h1.wrapping_add((i as u64).wrapping_mul(h2)) % self.bits.len() as u64) as usize
    }

    fn add<K: Hash>(&mut self, key: &K) {
        for i in 0..self.num_hashes {
            let idx = self.nth_hash(key, i);
            self.bits[idx] = true;
        }
    }

    fn possibly_contains<K: Hash>(&self, key: &K) -> bool {
        (0..self.num_hashes).all(|i| self.bits[self.nth_hash(key, i)])
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        self.bits.fill(false);
    }
}

struct HashTableWithBloom<K, V> {
    table: HashTable<K, V>,
    filter: BloomFilter,
    rejected_by_bloom: Cell<usize>,
}

impl<K: Hash + Eq + Clone, V> HashTableWithBloom<K, V> {
    fn new(
        initial_capacity: usize,
        method: ProbingMethod,
        bloom_bits: usize,
        bloom_hashes: usize,
    ) -> Self {
        Self {
            table: HashTable::new(initial_capacity, method),
            filter: BloomFilter::new(bloom_bits, bloom_hashes),
            rejected_by_bloom: Cell::new(0),
        }
    }

    fn insert(&mut self, key: K, value: V) -> bool {
        self.filter.add(&key);
        self.table.insert(key, value)
    }

    fn find(&self, key: &K) -> Option<&V> {
        if !self.filter.possibly_contains(key) {
            self.rejected_by_bloom.set(self.rejected_by_bloom.get() + 1);
            return None;
        }
        self.table.find(key)
    }

    fn rejected_count(&self) -> usize {
        self.rejected_by_bloom.get()
    }
}

fn main() {
    const N: usize = 50_000;
    let mut smart_table: HashTableWithBloom<String, i32> =
        HashTableWithBloom::new(1009, ProbingMethod::DoubleHashing, N * 10, 4);

    for i in 0..N {
        smart_table.insert(format!("user_{i}"), i as i32);
    }

    if let Some(val) = smart_table.find(&"user_777".to_string()) {
        println!("Ключ 'user_777' найден! Значение: {val}");
    }

    let mut found_false_positives = 0usize;
    let start = Instant::now();

    for i in 0..N {
        if smart_table.find(&format!("guest_{i}")).is_some() {
            found_false_positives += 1;
        }
    }

    let elapsed = start.elapsed();

    println!("\n=== Результаты поиска {N} отсутствующих ключей ===");
    println!("Время выполнения: {:.3} мс", elapsed.as_secs_f64() * 1000.0);
    println!(
        "Отсечено фильтром Блума (без поиска в таблице): {} из {}",
        smart_table.rejected_count(),
        N
    );
    println!("Найдено ложных элементов: {found_false_positives}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_false_negatives() {
        let mut t: HashTableWithBloom<String, i32> =
            HashTableWithBloom::new(101, ProbingMethod::Linear, 10_000, 4);
        for i in 0..500 {
            t.insert(format!("k{i}"), i);
        }
        for i in 0..500 {
            assert_eq!(t.find(&format!("k{i}")), Some(&i));
        }
    }

    #[test]
    fn missing_keys_are_mostly_rejected_by_filter() {
        let mut t: HashTableWithBloom<String, i32> =
            HashTableWithBloom::new(101, ProbingMethod::Linear, 100_000, 4);
        for i in 0..500 {
            t.insert(format!("k{i}"), i);
        }
        let mut found = 0;
        for i in 0..1000 {
            if t.find(&format!("absent{i}")).is_some() {
                found += 1;
            }
        }
        assert_eq!(found, 0); // таблица точная: ложных находок быть не может
        assert!(t.rejected_count() > 900); // фильтр отсёк почти всё
    }
}
