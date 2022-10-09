pub struct Array2 <T>{
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T:Clone> Array2<T> {
    pub fn from_row_major(width: usize, height: usize, data: Vec<T>) -> Self {
        //!This constructor will take data in row major form and construct a 2array
        Array2 {
            width,
            height,
            data,
        }
    }

    pub fn from_col_major(width: usize, height: usize, data: Vec<T>) -> Self {
        //!This constructor will take data in column major form and construct a 2array
        let mut data_col_major = vec![];
        for i in 0..height{
            for j in 0..width{
                data_col_major.push(data[i * width + j].clone())
            }
        }
        Array2 {
            width,
            height,
            data: data_col_major,
        }
    }

    pub fn blank_slate(width: usize, height: usize, val: T) -> Self {
        //!This constructor will construct a width*height 2array with each element being val
        let data = vec![val; width * height];
        Array2 {
            width,
            height,
            data,
        }
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item=&T> {
        //!This function will allow us to iterate row-major, which refers to visiting every element of the first row in order by column, followed by every element of the second row, and so forth.
        self.data.iter()
    }

    pub fn iter_col_major(&self) -> impl Iterator<Item=&T> {
        //!This function will allow us to iterate column-major, which refers to visiting every element of the first (left-most) column, followed by every element of the second column, and so forth
        (0..self.width).map(|x|self.data.iter().skip(x).step_by(self.width)).flatten()
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        //!This function allows access to an element in a 2array by a pair of coordinates
        self.data[row * self.width + col].clone()
     }
}