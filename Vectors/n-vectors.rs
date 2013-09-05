use std::{io, int, float};

fn make_vec(x: &str) -> ~[float] {
    let words = x.word_iter().collect::<~[&str]>();

    words.slice(1, words.len()).map(|x| float::from_str(*x).unwrap())
}

fn print_vec_floats(vec: &[float]) -> () {
    for vec.iter().advance |x| {
        print(fmt!("%.5f ", *x));
    }

    print("\n");
}

fn dot(vec1: &[float], vec2: &[float]) -> float {
    if (vec1.len() != vec2.len()) {
        fail!("Doc prouct vectors must be the same length")
    }

    let mut dot: float = 0.0;
    let mut count = 0;

    while count < vec1.len() {
        //I couldn't figure out how to do this with map and zip
        dot += vec1[count] * vec2[count];
        count += 1;
    }

    dot
}

fn length(vec: &[float]) -> float {
    float::sqrt(dot(vec, vec))
}

fn normalize(vec: &[float]) -> ~[float] {
    let len = length(vec);
    vec.map(|x| *x / len)
}

fn main() {
    let input = io::stdin().read_lines();

    let num_vecs; //Number of vectors
    match int::from_str(input[0]) {
        Some(x) if x > 0 => num_vecs = x as uint,
        _ => fail!("Must have a valid number of lines")
    }

    let vecs: ~[~[float]];

    let vec_lines = input.slice(1, num_vecs + 1); //Get lines that contain the vectors

    vecs = vec_lines.map(|x| make_vec(*x));

    let ops_start = num_vecs + 1;
    let num_ops; //Number of calculations
    match int::from_str(input[ops_start]) {
        Some(x) if x > 0 => num_ops = x as uint,
        _ => fail!("Must have a valid number of lines")
    }

    let op_lines = input.slice(ops_start + 1, ops_start + num_ops + 1);

    for op_lines.iter().advance |line| {
        let op = line.word_iter().collect::<~[&str]>();

        match op {
            ["l", a] => println(fmt!("%.5f", length(vecs[int::from_str(a).unwrap()]))),
            ["n", a] => print_vec_floats(normalize((vecs[int::from_str(a).unwrap()]))),
            ["d", a, b] => println(fmt!("%.5f", dot(vecs[int::from_str(a).unwrap()],
                                                    vecs[int::from_str(b).unwrap()]))),
            _ => fail!("Invalid operation")
        }
    }
}