Jason Aguirre and Eric Shaw

TA help hours from Connor Gray

Implemented:
- from_row_major
- from_col_major
- blank_slate
- iter_row_major
- iter_col_major
- get
- sudoku checker

Design Checklist:

Functions:
iter_row_major -> iterator
This function will allow us to iterate row-major, which refers to visiting every element of 
the first row in order by column, followed by every element of the second row, and so forth.

iter_column_major -> iterator
This function will allow us to iterate column-major, which refers to visiting every element 
of the first (left-most) column, followed by every element of the second column, and so forth

from_row_major -> 2array
This constructor will take data in row major form and construct a 2array

from_col_major -> 2array
This constructor will take data in column major form and create a 2array

blank_slate -> 2array
This constructor will construct a width*height 2array with each element being val

Get -> some<T>
This function allows access to an element by a pair of coordinates

Representation:

Our 2array will be represented as a 1 dimensional vector.

In order to iterate by row major we can iterate through the vector as normal, 
from left to right, top to bottom.

In order to iterate by column major we can utilize the map, step_by, 
and enumerate functions. We can relate an index in the input vector to 
our 2array by following this formula: index = (number of columns * row) + column; 
which means that for every column major iteration we can increment row by one, 
which should then correspond to the value of input[index]. 
We will want to increment the column by one once row == number of rows - 1.

Time:
This assignment took about 20 hours to complete
