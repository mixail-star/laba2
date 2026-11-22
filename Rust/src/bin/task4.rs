//! Задание 4: проверка ударений. Словарь слов с одной заглавной буквой-ударением;
//! считаем слова текста с ошибкой.

use labs::array::Array;
use std::io::{self, Read};

fn count_stresses(s: &str) -> usize {
    s.bytes().filter(u8::is_ascii_uppercase).count()
}

/// Аналог `cin >> n; getline(cin, dummy);`: возвращает n и остаток после первой строки.
fn read_count(input: &str) -> Option<(usize, &str)> {
    let trimmed = input.trim_start();
    let end = trimmed
        .find(|c: char| c.is_ascii_whitespace())
        .unwrap_or(trimmed.len());
    // Отрицательное n в оригинале просто даёт пустой словарь.
    let n: i64 = trimmed[..end].parse().ok()?;
    let n = n.max(0) as usize;
    let rest = &trimmed[end..];
    let rest = match rest.find('\n') {
        Some(pos) => &rest[pos + 1..],
        None => "",
    };
    Some((n, rest))
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let Some((n, rest)) = read_count(&input) else {
        return Ok(());
    };

    // Недостающие строки на конце ввода считаются пустыми (как getline на EOF).
    let mut lines = rest.split('\n');

    let mut dict: Array<String> = Array::new();
    let mut dict_lower: Array<String> = Array::new();

    for _ in 0..n {
        let word = lines.next().unwrap_or("");
        dict.push(word.to_string());
        dict_lower.push(word.to_ascii_lowercase());
    }

    let text = lines.next().unwrap_or("");

    let mut errors = 0;

    for word in text.split_whitespace() {
        let lower_w = word.to_ascii_lowercase();
        let mut in_dict = false;
        let mut exact_match = false;

        for i in 0..dict.len() {
            let d_low = dict_lower.get(i).expect("индекс в пределах словаря");
            let d_orig = dict.get(i).expect("индекс в пределах словаря");

            if *d_low == lower_w {
                in_dict = true;
                if d_orig == word {
                    exact_match = true;
                    break;
                }
            }
        }

        if in_dict {
            if !exact_match {
                errors += 1;
            }
        } else if count_stresses(word) != 1 {
            errors += 1;
        }
    }

    println!("{errors}");
    Ok(())
}
