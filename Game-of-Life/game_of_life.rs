use std::{io, os, uint};

fn print_grid(row: &[bool], row_len: uint) -> () {
    print("\x1B[2J\x1B[H"); //Clear the screen

    let mut count = 0;
    for row.iter().advance |x| {
        match *x {
            true => print("x"),
            false => print(".")
        }

        count += 1;

        if (count % row_len == 0) {
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

fn build_grid(input: &str, row_len: uint) -> ~[bool] {
    let mut grid: ~[bool] = ~[];

    for input.iter().advance |c| {
        match c {
            '.' => grid.push(false),
            'x' => grid.push(true),
            _ => {}
        }
    }

    if (grid.len() % row_len) != 0 {
        fail!("Input grid must be rectangular");
    }

    grid
}

fn main() {
    let args = os::args();

    if args.len() != 3 {
        println("Usage: ./game_of_life [File] [Row Length]");
        fail!("Invalid number of arguments");
    }
    let input = io::read_whole_file_str(&Path(args[1])).unwrap();

    let row_len = uint::from_str(args[2]).unwrap();

    let mut grid = build_grid(input, row_len);

    let mut generation = 0;

    loop {
        print_grid(grid, row_len);
        println(fmt!("Generation: %d", generation));
        generation += 1;

        let mut index: uint = 0;
        let mut new_grid: ~[bool] = ~[];

        while index < grid.len() {
            //Calculate updated grid
            new_grid.push(check_alive(grid, index, row_len));
            index += 1;
        }

        grid = new_grid;

        let mut i = 0;
        while (i < 220000000) {
            //Stopgap pause measure
            //TODO: Replace with sleep/timer func
            i += 1;
        }
    }
}