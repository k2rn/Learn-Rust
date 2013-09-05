fn print_grid(row: &[bool], c: int) -> () {
    let mut count = 0;
    for row.iter().advance |x| {
        match *x {
            true => print("x"),
            false => print(".")
        }

        count += 1;

        if (count % c == 0) {
            print("\n");
        }
    }
}

fn main() {
    let count = 6;

    let mut grid = ~[false, false, true, true, false, false,
                     false, true, false, false, true, false,
                     true, false, false, false, false, true,
                     true, false, false, false, false, true,
                     false, true, false, false, true, false,
                     false, false, true, true, false, false];

    loop {
        print("\x1B[2J\x1B[H");
        print_grid(grid, count);
        grid = grid.map(|x| !*x);

        let mut  i = 0;
        while (i < 300000000) {
            //Stopgap pause measure
            //TODO: Replace with sleep/timer func
            i += 1;
        }
    }
}