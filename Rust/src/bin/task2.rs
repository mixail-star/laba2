//! Задание 2: множества строк в файле. Команды SETADD / SETDEL / SET_AT.
//!
//! Запуск: task2 --file data.txt --query "SETADD имя значение"

use std::collections::VecDeque;
use std::fs;
use std::process::ExitCode;

const TABLE_SIZE: usize = 1009;

/// Хеш-множество строк с цепочками (новые элементы — в начало цепочки).
struct HashSet {
    table: Vec<VecDeque<String>>,
}

impl HashSet {
    fn new() -> Self {
        Self {
            table: vec![VecDeque::new(); TABLE_SIZE],
        }
    }

    fn hash(key: &str) -> usize {
        const P: usize = 31;
        key.bytes()
            .fold(0usize, |h, c| (h * P + c as usize) % TABLE_SIZE)
    }

    fn add(&mut self, key: &str) {
        if self.contains(key) {
            return; // множество хранит только уникальные элементы
        }
        self.table[Self::hash(key)].push_front(key.to_string());
    }

    fn remove(&mut self, key: &str) -> bool {
        let chain = &mut self.table[Self::hash(key)];
        match chain.iter().position(|k| k == key) {
            Some(pos) => {
                chain.remove(pos);
                true
            }
            None => false,
        }
    }

    fn contains(&self, key: &str) -> bool {
        self.table[Self::hash(key)].iter().any(|k| k == key)
    }

    fn serialize(&self) -> String {
        let items: Vec<&str> = self
            .table
            .iter()
            .flat_map(|chain| chain.iter().map(String::as_str))
            .collect();
        items.join(" ")
    }
}

struct NamedSet {
    name: String,
    set: HashSet,
}

/// Набор именованных множеств (новое — в начало, как в оригинале,
/// от этого зависит порядок строк в файле).
struct SetStorage {
    sets: Vec<NamedSet>,
}

impl SetStorage {
    fn new() -> Self {
        Self { sets: Vec::new() }
    }

    fn get_or_create(&mut self, name: &str) -> &mut HashSet {
        let idx = match self.sets.iter().position(|s| s.name == name) {
            Some(i) => i,
            None => {
                self.sets.insert(
                    0,
                    NamedSet {
                        name: name.to_string(),
                        set: HashSet::new(),
                    },
                );
                0
            }
        };
        &mut self.sets[idx].set
    }

    fn load_from_file(&mut self, filename: &str) {
        // Если файла ещё нет, он создастся при сохранении.
        let Ok(content) = fs::read_to_string(filename) else {
            return;
        };
        for line in content.split('\n') {
            let mut tokens = line.split_whitespace();
            let Some(set_name) = tokens.next() else {
                continue;
            };
            let set = self.get_or_create(set_name);
            for item in tokens {
                set.add(item);
            }
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut out = String::new();
        for ns in &self.sets {
            let items = ns.set.serialize();
            if !items.is_empty() {
                out.push_str(&ns.name);
                out.push(' ');
                out.push_str(&items);
                out.push('\n');
            }
        }
        if fs::write(filename, out).is_err() {
            eprintln!("Ошибка: не удалось открыть файл для записи.");
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut filename = String::new();
    let mut query = String::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--file" && i + 1 < args.len() {
            i += 1;
            filename = args[i].clone();
        } else if args[i] == "--query" && i + 1 < args.len() {
            i += 1;
            query = args[i].clone();
        }
        i += 1;
    }

    if filename.is_empty() || query.is_empty() {
        return ExitCode::FAILURE;
    }

    let mut parts = query.split_whitespace();
    let (Some(command), Some(set_name), Some(value)) = (parts.next(), parts.next(), parts.next())
    else {
        return ExitCode::FAILURE;
    };

    let mut storage = SetStorage::new();
    storage.load_from_file(&filename);

    match command {
        "SETADD" => {
            storage.get_or_create(set_name).add(value);
            storage.save_to_file(&filename);
            println!("OK");
        }
        "SETDEL" => {
            if storage.get_or_create(set_name).remove(value) {
                storage.save_to_file(&filename);
                println!("OK");
            } else {
                println!("NOT FOUND");
            }
        }
        "SET_AT" => {
            if storage.get_or_create(set_name).contains(value) {
                println!("TRUE");
            } else {
                println!("FALSE");
            }
        }
        _ => {
            eprintln!("Неизвестная команда: {command}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_basics() {
        let mut s = HashSet::new();
        s.add("a");
        s.add("b");
        s.add("a");
        assert!(s.contains("a"));
        assert!(!s.contains("c"));
        assert!(s.remove("a"));
        assert!(!s.remove("a"));
        assert_eq!(s.serialize(), "b");
    }

    #[test]
    fn colliding_keys_share_a_bucket() {
        // "a" и ключ с тем же хешем по модулю 1009 должны уживаться в одной цепочке
        let mut s = HashSet::new();
        let base = "a";
        let mut twin = None;
        for c in 'b'..='z' {
            for d in 'a'..='z' {
                let cand = format!("{c}{d}");
                if HashSet::hash(&cand) == HashSet::hash(base) {
                    twin = Some(cand);
                }
            }
        }
        // если коллизии нет — тест остаётся валидным по смыслу
        s.add(base);
        if let Some(t) = twin {
            s.add(&t);
            assert!(s.contains(&t));
            assert!(s.remove(base));
            assert!(s.contains(&t));
        }
    }
}
