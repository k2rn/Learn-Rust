use std::{int, str, io};

fn check_upper(s1: &str, s2: &str) -> bool {
    s1.to_ascii().to_upper() == s2.to_ascii().to_owned()
}

fn check_reverse(s1: &str, s2: &str) -> bool {
    let mut rev_str = ~"";

    for c in s1.rev_iter() {
        rev_str = rev_str + str::from_char(c);
    }
    
    rev_str == s2.to_owned()
}

fn main() {
    let input = io::stdin().read_lines();

    let count;
    match int::from_str(input[0]) {
        Some(x) if x > 0 => count = x as uint,
        _ => fail!("Must have a valid number of lines")
    }

    let lines = input.slice(1, count+1);

    for line in lines.iter() {
        let words = line.word_iter().collect::<~[&str]>();

        match words[0] {
            "0" if check_reverse(words[1], words[2]) => println("Good test data"),
            "1" if check_upper(words[1], words[2]) => println("Good test data"),
            _ => println("Mismatch! Bad test data")
        }
    }
}