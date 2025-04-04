use std::collections::HashMap;

pub fn rotx(input: &str, x: u8) -> String
{
    input.chars().map(|c| shift_char(c, x)).collect()
}

fn shift_char(c: char, x: u8) -> char
{
    match c {
        'a'..='z' => ((c as u8 - b'a' + x) % 26 + b'a') as char,
        'A'..='Z' => ((c as u8 - b'A' + x) % 26 + b'A') as char,
        _ => c,
    }
}

pub fn encrypt(input: &str, shift: u8) -> String
{
    rotx(input, shift % 26)
}

pub fn decrypt(input: &str, shift: u8) -> String
{
    rotx(input, 26 - (shift % 26))
}

pub fn auto_decrypt(input: &str) -> String
{
    let mut possibilities = Vec::new();

    println!("Trying to guess the origin of the input:\n{}", input);

    for i in 0..26 {
        let decrypted = decrypt(input, i);
        possibilities.push(decrypted);
    }

    println!("Guessing the correct shift...");

    let best_guess = assign_probabilities(&possibilities);

    best_guess
}

fn _text_to_1337(input: &str) -> String
{
    input
        .chars()
        .map(|c| match c {
            'a' => '4',
            'e' => '3',
            'l' => '1',
            'o' => '0',
            's' => '5',
            't' => '7',
            _ => c,
        })
        .collect()
}

fn _l337_to_text(input: &str) -> String
{
    input
        .chars()
        .map(|c| match c {
            '4' => 'a',
            '3' => 'e',
            '1' => 'l',
            '0' => 'o',
            '5' => 's',
            '7' => 't',
            _ => c,
        })
        .collect()
}

fn assign_probabilities(possibilities: &[String]) -> String
{
    let mut scores: Vec<(String, f64)> = possibilities
        .iter()
        .map(|text| (text.clone(), score_text(text)))
        .collect();

    // Sort by score descending
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Return the highest probability match
    scores[0].0.clone()
}

fn score_text(text: &str) -> f64
{
    // https://en.wikipedia.org/wiki/Letter_frequency
    let letter_frequencies: HashMap<char, f64> = [
        ('e', 12.7),
        ('t', 9.1),
        ('a', 8.2),
        ('o', 7.5),
        ('i', 7.0),
        ('n', 6.7),
        ('s', 6.3),
        ('h', 6.1),
        ('r', 6.0),
        ('d', 4.3),
        ('l', 4.0),
        ('u', 2.8),
        ('c', 2.8),
        ('m', 2.4),
        ('w', 2.4),
        ('f', 2.2),
        ('g', 2.0),
        ('y', 2.0),
        ('p', 1.9),
        ('b', 1.5),
        ('v', 1.0),
        ('k', 0.8),
        ('j', 0.2),
        ('x', 0.2),
        ('q', 0.1),
        ('z', 0.1),
    ]
    .iter()
    .cloned()
    .collect();

    text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| {
            letter_frequencies
                .get(&c.to_ascii_lowercase())
                .unwrap_or(&0.0)
        })
        .sum()
}
