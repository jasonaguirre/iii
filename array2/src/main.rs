use array2::Array2;
use csc411_image::{Read, GrayImage};

fn main() {
    let data = vec![0,1,2,3,4,5];
    let board = Array2::from_row_major(3,2,data);

    println!("{}",board.get(1,1));
}