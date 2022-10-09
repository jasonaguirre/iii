use std::collections::{HashMap, HashSet};
use std::env;
use array2::Array2;
use csc411_image::{Read, GrayImage};

fn main() {
    let gray_board;
    if env::args().len() == 2 {
        let input = env::args().nth(1);
        gray_board = GrayImage::read(input.as_deref()).unwrap();
    }
    else{
        //get standard input
        assert!(env::args().len() < 2, "Too many arguments!");
        gray_board = GrayImage::read(None).unwrap();
    }

    let board = Array2::from_row_major(
        gray_board.width as usize,
        gray_board.height as usize,
        gray_board.pixels);

    if gray_board.width != 9 || gray_board.height != 9{
        std::process::exit(1);
    }

    valid_sudoku(board);
    std::process::exit(0);

}

fn valid_sudoku(board: Array2<csc411_image::Gray>) {
    //{row/col/square : set of values}
    let mut row_set: HashMap<_, HashSet<_>> = HashMap::new();
    let mut col_set: HashMap<_, HashSet<_>> = HashMap::new();
    let mut squares: HashMap<_, HashSet<_>> = HashMap::new();

    for i in 0..board.height {
        for j in 0..board.width {

            //check columns for duplicates
            match col_set.get_mut(&j) {
                Some(n) => {
                    //if number in set of this column, exit with 1
                    if n.insert(board.get(i, j).value) == false ||
                        board.get(i, j).value < 1 ||
                        board.get(i, j).value > 9 {

                        std::process::exit(1);
                    };
                }
                None => {
                    let mut values = HashSet::new();
                    values.insert(board.get(i, j).value);
                    col_set.insert((&j).to_owned(), values);
                }
            }

            //check rows for duplicates
            match row_set.get_mut(&i) {
                Some(n) => {
                    if n.insert(board.get(i, j).value) == false {
                        std::process::exit(1);
                    };
                }
                None => {
                    let mut values = HashSet::new();
                    values.insert(board.get(i, j).value);
                    row_set.insert((&i).to_owned(), values);
                }
            }

            //check squares for duplicates
            //tuple of i/3 and j/3 will give a unique identifier for each square
            match squares.get_mut(&(i/3, j/3)) {
                Some(n) => {
                    if n.insert(board.get(i, j).value) == false {
                        std::process::exit(1);
                    };
                }
                None => {
                    let mut values = HashSet::new();
                    values.insert(board.get(i, j).value);
                    squares.insert((&(i/3, j/3)).to_owned(), values);
                }
            }
        }
    }
}
