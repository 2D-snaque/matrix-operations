enum MatrixTypes {
    Zero(u16),
    Identity(u16),
    Scalar(u16, i128),
}

struct Matrix {
    elements: Vec<Vec<i128>>,
}

//Matrix Intitialisation
impl Matrix {
    fn default() -> Self {
        Self {
            elements: vec![vec![]],
        }
    }

    //Initializes a new matrix with the input as the elements
    fn construct(elements: Vec<Vec<i128>>) -> Self {
        Self { elements }
    }

    //returns number of rows in the matrix
    fn get_num_row(&self) -> usize {
        self.elements.len()
    }

    //returns number of columns in the matrix
    fn get_num_col(&self) -> usize {
        self.elements[0].len()
    }

    //Displays the matrix in a readable manner
    /* TODO: implement the Diplay trait for Matrix
     */

    fn display(&self) {
        for i in &self.elements {
            for k in i {
                print!(" {} ", k)
            }
            println!()
        }
        println!()
    }

    //Constructs a new matrix depending on the Type of Matrix required
    fn new_preset(matrix_type: MatrixTypes) -> Self {
        let mut matrix = Matrix::default();

        match matrix_type {
            MatrixTypes::Zero(i) => {
                for k in 0..i {
                    for _l in 0..i {
                        matrix.elements[(k) as usize].push(0);
                    }
                    if k < i - 1 {
                        matrix.elements.push(vec![]);
                    }
                }
                println!("{:?}", matrix.elements);
            }

            MatrixTypes::Identity(i) => {
                for k in 0..i {
                    for l in 0..i {
                        if k == l {
                            matrix.elements[k as usize].push(1);
                        } else {
                            matrix.elements[k as usize].push(0);
                        }
                    }
                    if k < i - 1 {
                        matrix.elements.push(vec![]);
                    }
                }
            }

            MatrixTypes::Scalar(i, j) => {
                for k in 0..i {
                    for l in 0..i {
                        if k == l {
                            matrix.elements[k as usize].push(j);
                        } else {
                            matrix.elements[k as usize].push(0);
                        }
                    }
                    if k < i - 1 {
                        matrix.elements.push(vec![]);
                    }
                }
            }
        }
        return matrix;
    }
}

// Matrix Operations
impl Matrix {
    /* TODO: multiplication ✅
             addition✅
             subtraction✅
             inverse
             determinant
    */

    //Multiplication of two matrices (not commutative)
    fn multiplication(matrix1: &Matrix, matrix2: &Matrix) -> Result<Matrix, String> {
        let mut matrix: Matrix = Matrix::default();

        if matrix1.get_num_col() != matrix2.get_num_row() {
            return Err(String::from("Wrong Order"));
        }
        for i in 0..matrix1.get_num_row() {
            matrix.elements.push(vec![]);
            for j in 0..matrix1.get_num_col() {
                let mut ele: i128 = 0;
                for k in 0..matrix2.get_num_col() {
                    ele += matrix1.elements[i][j] * matrix2.elements[k][i];
                }

                matrix.elements[i].push(ele);
            }
        }
        return Ok(matrix);
    }

    //Addition of two matrices
    fn summation(matrix1: &Matrix, matrix2: &Matrix) -> Result<Matrix, String> {
        let mut matrix = Matrix::default();

        if (matrix1.get_num_row() != matrix2.get_num_row())
            || (matrix1.get_num_col() != matrix2.get_num_col())
        {
            return Err(String::from("Wrong Order"));
        }

        for i in 0..matrix1.get_num_row() {
            matrix.elements.push(vec![]);
            for j in 0..matrix1.get_num_col() {
                matrix.elements[i].push(0);
                matrix.elements[i][j] = matrix1.elements[i][j] + matrix2.elements[i][j];
            }
        }

        return Ok(matrix);
    }
    //Subtraction of two matrices (not commutative)
    fn deduction(matrix1: &Matrix, matrix2: &Matrix) -> Result<Matrix, String> {
        let mut matrix = Matrix::default();

        if (matrix1.get_num_row() != matrix2.get_num_row())
            || (matrix1.get_num_col() != matrix2.get_num_col())
        {
            return Err(String::from("Wrong Order"));
        }

        for i in 0..matrix1.get_num_row() {
            matrix.elements.push(vec![]);
            for j in 0..matrix1.get_num_col() {
                matrix.elements[i].push(0);
                matrix.elements[i][j] = matrix1.elements[i][j] - matrix2.elements[i][j];
            }
        }

        return Ok(matrix);
    }

    fn matrix2x2(&self) -> i128 {
        self.elements[0][0] * self.elements[1][1] - self.elements[0][1] * self.elements[1][0]
    }
    //Determinant of a matrix
    fn determinant(&self) -> i128 {
        let mut list_deter = vec![];
        let mut determinant: i128 = 0;
        if self.get_num_row() <= 2 {
            if self.get_num_row() == 1 {
                return self.elements[0][0];
            }
            return self.matrix2x2();
        }
        for j in 0..self.get_num_col() {
            let mut matrix = Matrix::default();
            for i in 1..self.get_num_row() {
                for l in 0..self.get_num_col() {
                    if l == j {
                        continue;
                    }
                    matrix.elements[i - 1].push(self.elements[i][l]);
                }
                if matrix.get_num_row() == self.get_num_row() - 1 {
                    break;
                }
                matrix.elements.push(vec![]);
            }
            list_deter.push(matrix)
        }
        if list_deter[0].get_num_col() == 2 {
            for i in 0..self.get_num_row() {
                determinant +=
                    (-1 as i128).pow(i as u32) * self.elements[0][i] * list_deter[i].matrix2x2();
            }
        } else {
            for i in 0..self.get_num_row() {
                determinant +=
                    (-1 as i128).pow(i as u32) * self.elements[0][i] * list_deter[i].determinant();
            }
        }
        return determinant;
    }
}

fn main() {
    let matrix = Matrix::construct(vec![vec![9, 7, 8], vec![0, 3, 1], vec![4, 5, 6]]);
    matrix.display();
    eprintln!("{}", matrix.determinant())
}
