use std::{io, int, float};

fn make_vec(x: &str) -> ~[float] {
    let words = x.word_iter().collect::<~[&str]>();

    words.map(|x| float::from_str(*x).unwrap())
}

fn main() {
    let input = io::stdin().read_lines();

    let count; //Get number of vectors
    match int::from_str(input[0]) {
        Some(x) if x > 0 => count = x as uint,
        _ => fail!("Must have a valid number of lines")
    }

    let vecs: ~[~[float]];

    let vec_lines = input.slice(1, count +1); //Get lines that contain the vectors

    vecs = vec_lines.map(|x| make_vec(*x));

    println(fmt!("%?", vecs));
}