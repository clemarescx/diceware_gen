use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Write},
};

use rand::{thread_rng, Rng};

fn main() {
    let diceware_list =
        File::open("diceware.txt").expect("file 'diceware.txt' should be in root dir");
    let reader = BufReader::new(diceware_list);
    let diceware_words: HashMap<u32, String> = reader
        .lines()
        .map(|l| {
            l.expect("all lines are readable")
                .split_once('\t')
                .map(|(k, w)| {
                    (
                        k.parse::<u32>().expect("key should parse to number"),
                        w.to_string(),
                    )
                })
                .expect("key and word delimited by a tab")
        })
        .collect();

    let word_count = loop {
        print!("number of words in password (255 max): ");
        std::io::stdout()
            .flush()
            .expect("stdout should be flushable");
        let mut buf = String::new();
        let res = std::io::stdin()
            .lock()
            .read_line(&mut buf)
            .map(|_| buf.trim().parse::<u8>());
        if let Ok(Ok(count)) = res {
            break count;
        } else {
            println!("\ncould not parse number, try again")
        }
    };
    let mut rng = thread_rng();
    let mut password_words = Vec::with_capacity(word_count as usize);
    for _ in 0..word_count {
        let die_rolls = (0..5).fold(0, |acc, _| (acc * 10) + rng.gen_range(1..=6));
        let word = diceware_words
            .get(&die_rolls)
            .unwrap_or_else(|| panic!("no word for key {die_rolls}"));
        password_words.push(word.clone());
    }
    let charset_count = diceware_words
        .values()
        .flat_map(|w| w.chars())
        .chain([' '])
        .collect::<HashSet<_>>()
        .len() as f64;

    let pw_string = password_words.join(" ");
    let pw_char_count = pw_string.len() as i32;
    let lowercase_bit_entropy = charset_count.powi(pw_char_count).log2();
    let diceware_word_count = diceware_words.len();
    let diceware_bit_entropy = (diceware_word_count as f64).powi(word_count as i32).log2();

    println!("password: ");
    println!("\t{pw_string}");
    println!("stats: ");
    println!("- password length: {pw_char_count}");
    println!("- word charset bit entropy: {lowercase_bit_entropy:.2} <- log2({charset_count:.0}^{pw_char_count})");
    println!("- diceware list bit entropy: {diceware_bit_entropy:.2} <- log2({diceware_word_count}^{word_count})");
}
