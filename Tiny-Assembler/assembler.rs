use std::{io, uint};

struct Instruction {
	op: ~str, //The opcode
	vals: ~[uint], //The hex value of the argument
	address: ~[bool], //Whether or not the value is an address
}

impl Instruction {
	fn new(instr: &[&str]) -> Instruction {
		let name = instr[0].to_ascii().to_upper().to_str_ascii();
		let mut v: ~[uint] = ~[];
		let mut a: ~[bool] = ~[];

		for instr.slice(1, instr.len()).iter().advance |arg| {
			if arg.starts_with("[") {
				v.push(uint::from_str(arg.trim_chars(& &['[', ']'])).unwrap());
				a.push(true);
			}

			else {
				v.push(uint::from_str(*arg).unwrap());
				a.push(false);
			}
		}

		Instruction{op: name, vals: v, address: a}
	}
}

fn get_offset(a: &[bool]) -> uint {
	match a {
		[false] => 1,

		[true, false] => 1,
		[false, true] => 2,
		[false, false] => 3,

		[false, true, true] => 1,
		[true, true, false] => 2,
		[false, true, false] => 3,

		_ => 0

	}
}

fn main() {
	let input = io::stdin().read_lines();

	for input.iter().advance |line| {
		let words = line.word_iter().collect::<~[&str]>();
		let instr = Instruction::new(words);

		let opcode: uint = match instr.op {
			~"AND"    => 0x00,
			~"OR"     => 0x02,
			~"XOR"    => 0x04,
			~"NOT"    => 0x06,
			~"MOV"    => 0x07,
			~"RANDOM" => 0x09,
			~"ADD"    => 0x0a,
			~"SUB"    => 0x0c,
			~"JMP"    => 0x0e,
			~"JZ"     => 0x10,
			~"JEQ"    => 0x14,
			~"JLS"    => 0x18,
			~"JGT"    => 0x1c,
			~"HALT"   => 0xff,
			~"APRINT" => 0x20,
			~"DPRINT" => 0x22,
			_         => fail!("Invalid instruction")
		} + get_offset(instr.address);

		print(fmt!("0x%.2X ", opcode));

		for instr.vals.iter().advance |val| {
			print(fmt!("0x%.2X ", *val));
		}

		print("\n");
	}
}