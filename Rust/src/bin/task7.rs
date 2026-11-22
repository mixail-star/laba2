//! Задание 7: LRU-кэш на хеш-таблице и списке порядка использования.
//!
//! Ввод: `cap Q`, затем Q команд `SET x y` / `GET x`. Результаты GET печатаются
//! через пробел в одной строке (-1, если ключа нет).

use labs::hash_table::{HashTable, ProbingMethod};
use labs::llist::List;
use std::io::{self, Read, Write};

struct LruCache {
    capacity: i32,
    current_size: i32,
    cache_map: HashTable<i32, i32>,
    /// Ключи от самого свежего (голова) к самому старому (хвост).
    order_list: List<i32>,
}

impl LruCache {
    fn new(capacity: i32) -> Self {
        let map_capacity = capacity.max(0) as usize * 2;
        Self {
            capacity,
            current_size: 0,
            cache_map: HashTable::new(map_capacity, ProbingMethod::Linear),
            order_list: List::new(),
        }
    }

    fn touch(&mut self, key: i32) {
        self.order_list.pop_value(&key);
        self.order_list.push_head(key);
    }

    fn get(&mut self, x: i32) -> i32 {
        let Some(&val) = self.cache_map.find(&x) else {
            return -1;
        };
        self.touch(x);
        val
    }

    fn set(&mut self, x: i32, y: i32) {
        if self.capacity <= 0 {
            return;
        }

        if self.cache_map.find(&x).is_some() {
            self.cache_map.insert(x, y);
            self.touch(x);
            return;
        }

        if self.current_size == self.capacity {
            if let Some(oldest) = self.order_list.pop_tail() {
                self.cache_map.remove(&oldest);
            }
            self.current_size -= 1;
        }

        self.cache_map.insert(x, y);
        self.order_list.push_head(x);
        self.current_size += 1;
    }
}

fn next_int<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Option<i32> {
    tokens.next()?.parse().ok()
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut tokens = input.split_whitespace();

    let (Some(cap), Some(q)) = (next_int(&mut tokens), next_int(&mut tokens)) else {
        return Ok(());
    };

    let mut cache = LruCache::new(cap);
    let mut results = String::new();

    for _ in 0..q {
        let Some(kind) = tokens.next() else { break };
        match kind {
            "SET" => {
                let (Some(x), Some(y)) = (next_int(&mut tokens), next_int(&mut tokens)) else {
                    break;
                };
                cache.set(x, y);
            }
            "GET" => {
                let Some(x) = next_int(&mut tokens) else { break };
                results.push_str(&cache.get(x).to_string());
                results.push(' ');
            }
            _ => {}
        }
    }

    writeln!(io::stdout().lock(), "{results}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evicts_least_recently_used() {
        let mut c = LruCache::new(2);
        c.set(1, 10);
        c.set(2, 20);
        assert_eq!(c.get(1), 10); // теперь самый старый — 2
        c.set(3, 30); // вытесняет 2
        assert_eq!(c.get(2), -1);
        assert_eq!(c.get(1), 10);
        assert_eq!(c.get(3), 30);
    }

    #[test]
    fn set_existing_updates_value_and_recency() {
        let mut c = LruCache::new(2);
        c.set(1, 10);
        c.set(2, 20);
        c.set(1, 11); // обновили 1, самый старый — 2
        c.set(3, 30);
        assert_eq!(c.get(2), -1);
        assert_eq!(c.get(1), 11);
    }

    #[test]
    fn zero_capacity_stores_nothing() {
        let mut c = LruCache::new(0);
        c.set(1, 1);
        assert_eq!(c.get(1), -1);
    }
}
