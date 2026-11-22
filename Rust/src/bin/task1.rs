//! Задание 1: проверка XML-подобной строки стеком; поиск замены одного символа,
//! после которой строка становится корректной.

use labs::stack::Stack;
use std::io::{self, BufRead, Write};

/// Корректна ли строка вида `<a><b></b></a>` (теги только из букв a–z, пробелы между тегами).
fn is_valid_xml(s: &[u8]) -> bool {
    let n = s.len();
    let mut stack: Stack<&[u8]> = Stack::new();
    let mut i = 0;

    while i < n {
        while i < n && s[i] == b' ' {
            i += 1;
        }
        if i >= n {
            break;
        }

        if s[i] != b'<' {
            return false;
        }
        i += 1;
        if i >= n {
            return false;
        }

        let mut is_closing = false;
        if s[i] == b'/' {
            is_closing = true;
            i += 1;
        }

        let start = i;
        while i < n && s[i].is_ascii_lowercase() {
            i += 1;
        }
        let tag = &s[start..i];

        if tag.is_empty() || i >= n || s[i] != b'>' {
            return false;
        }
        i += 1;

        if !is_closing {
            stack.push(tag);
        } else {
            match stack.pop() {
                Some(top) if top == tag => {}
                _ => return false,
            }
        }
    }

    stack.is_empty()
}

fn main() -> io::Result<()> {
    let mut line = Vec::new();
    if io::stdin().lock().read_until(b'\n', &mut line)? == 0 {
        return Ok(());
    }
    if line.last() == Some(&b'\n') {
        line.pop();
    }
    if line.last() == Some(&b'\r') {
        line.pop();
    }
    let mut s = line;

    let mut alphabet: Vec<u8> = b"<>/".to_vec();
    alphabet.extend(b'a'..=b'z');

    let mut out = io::stdout().lock();

    // В исходном .cpp условие цикла было `i >= s.length()`, и тело не выполнялось ни разу.
    // Приложенный бинарник `task1` ведёт себя как обход с конца строки
    // (`i = len - 1; i >= 0; --i`) — сверено на ~1900 входах, портирован этот вариант.
    for i in (0..s.len()).rev() {
        let original = s[i];
        if original == b' ' {
            continue;
        }

        for &c in &alphabet {
            if c == original {
                continue;
            }
            s[i] = c;
            if is_valid_xml(&s) {
                out.write_all(&s)?;
                out.write_all(b"\n")?;
                return Ok(());
            }
        }
        s[i] = original;
    }

    writeln!(out, "Incorrect")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_and_invalid() {
        assert!(is_valid_xml(b"<a></a>"));
        assert!(is_valid_xml(b"<a><b></b></a>"));
        assert!(is_valid_xml(b" <a> </a> "));
        assert!(is_valid_xml(b""));
        assert!(!is_valid_xml(b"<a></b>"));
        assert!(!is_valid_xml(b"<a>"));
        assert!(!is_valid_xml(b"</a>"));
        assert!(!is_valid_xml(b"<>"));
        assert!(!is_valid_xml(b"<A></A>"));
        assert!(!is_valid_xml(b"<a>x</a>"));
    }
}
