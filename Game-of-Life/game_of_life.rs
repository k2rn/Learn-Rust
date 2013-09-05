fn print_grid(row: &[bool], c: uint) -> () {
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

fn check_alive(grid: &[bool], index: uint, row_len: uint) -> bool {
    let mut live_neighbors = 0;

    if ((index % row_len) != 0 && index >= row_len) {
        //Check status of upper left neighbor, if possible
        if grid[index - row_len - 1] {live_neighbors += 1;}
    }

    if (index >= row_len) {
        //Check status of upper neighbor, if possible
        if grid[index - row_len] {live_neighbors += 1;}
    }

    if ((index % row_len) != row_len-1 && index >= row_len) {
        //Check status of upper right neighbor, if possible
        if grid[index - row_len + 1] {live_neighbors += 1;}
    }

    if ((index % row_len) != row_len-1) {
        //Check status of right neighbor, if possible
        if grid[index + 1] {live_neighbors += 1;}
    }

    if ((index % row_len) != row_len-1 && index < grid.len() - row_len) {
        //Check status of lower right neighbor, if possible
        if grid[index + row_len + 1] {live_neighbors += 1;}
    }

    if (index < grid.len() - row_len) {
        //Check status of lower neighbor, if possible
        if grid[index + row_len] {live_neighbors += 1;}
    }

    if ((index % row_len) != 0 && index < grid.len() - row_len) {
        //Check status of lower left neighbor, if possible
        if grid[index + row_len -1] {live_neighbors += 1;}
    }

    if ((index % row_len) != 0) {
        //Check status of left neighbor, if possible
        if grid[index - 1] {live_neighbors += 1;}
    }

    match grid[index] {
        true if (live_neighbors == 2 || live_neighbors == 3) => true,
        false if (live_neighbors == 3) => true,
        _ => false
    }
}

fn main() {
    let row_len = 6;

    let mut grid = ~[false, false, true, true, false, false,
                     false, true, false, false, true, false,
                     true, false, false, false, false, true,
                     true, false, false, false, false, true,
                     false, true, false, false, true, false,
                     false, false, true, true, false, false];

    loop {
        print("\x1B[2J\x1B[H");

        print_grid(grid, row_len);

        let mut index: uint = 0;
        let mut new_grid: ~[bool] = ~[];

        while index < grid.len() {
            new_grid.push(check_alive(grid, index, row_len));
            index += 1;
        }

        grid = new_grid;

        let mut i = 0;
        while (i < 250000000) {
            //Stopgap pause measure
            //TODO: Replace with sleep/timer func
            i += 1;
        }
    }
}