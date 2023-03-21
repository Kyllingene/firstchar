use std::env;
use std::io;

fn main() {
    let args = env::args();

    let text = if args.len() >= 2 {
        args.skip(1).collect::<Vec<String>>().join(" ")
    } else {
        io::stdin().lines().filter_map(|l| l.ok()).collect::<Vec<String>>().join("\n")
    };

    let mut short = String::new();
    let mut chs = text.chars().peekable();

    if let Some(ch) = chs.next() {
        short.push(ch);
    }

    while let Some(ch) = chs.next() {
        if ch.is_ascii_punctuation() || ch == '\n' || ch == '\t' {
            short.push(ch);

            if let Some(ch) = chs.peek() {
                if !(ch.is_ascii_punctuation() || *ch == '\n' || *ch == '\t') {
                    short.push(*ch);
                }
            }
        } else if ch == ' ' {
            if let Some(ch) = chs.peek() {
                if !(ch.is_ascii_punctuation() || *ch == '\n' || *ch == '\t') {
                    short.push(*ch);
                }
            }
        }
    }

    println!("{short}");
}
