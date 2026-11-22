//! Задание 3: «близость» геномов — сколько двухбуквенных пар первого генома
//! встречается среди пар второго.

use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tokens = input.split_whitespace();
    let (Some(genome1), Some(genome2)) = (tokens.next(), tokens.next()) else {
        return Ok(());
    };

    let second_pairs: HashSet<&[u8]> = genome2.as_bytes().windows(2).collect();

    let closeness = genome1
        .as_bytes()
        .windows(2)
        .filter(|pair| second_pairs.contains(pair))
        .count();

    println!("{closeness}");
    Ok(())
}
