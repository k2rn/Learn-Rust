use std::{int, io};

fn main() {
	let input = io::stdin().read_lines();

	let count;
	match int::from_str(input[0]) {
		Some(x) if x > 0 => count = x as uint,
		_ => fail!("Must have a valid number of lines")
	}

	let lines = input.slice(1, count+1);

	for lines.iter().advance |line| {
		println(fmt!("%?", line));
	}
}