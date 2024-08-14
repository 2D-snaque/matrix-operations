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

    fn construct(elements: Vec<Vec<i128>>) -> Self {
        Self { elements }
    }

    fn get_num_row(&self) -> usize {
        self.elements.len()
    }

    fn get_num_col(&self) -> usize {
        self.elements[0].len()
    }

    fn display(&self) {
        for i in &self.elements {
            for k in i {
                print!("{} ", k)
            }
            println!()
        }
    }

    fn new_matrix_type(matrix_type: MatrixTypes) -> Self {
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

    //Determinant of a matrix
    fn determinant(&self) -> Matrix {
        let mut matrix = Matrix::default();
        let determinant: i128 = 0;

        for k in 0..self.get_num_row() {
            if k != 0 {
                matrix.elements.push(vec![]);
                for l in 0..self.get_num_col() {
                    if l != 0 {
                        matrix.elements[k].push(self.elements[k][l]);
                    }
                }
            }
        }
        if matrix.get_num_col() != 2 {
            matrix = matrix.determinant();
        };
        matrix
    }
}

fn main() {
    let matrix = Matrix::construct(vec![
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
        vec![3, 3, 3, 3],
        vec![4, 4, 4, 4],
        vec![4, 5, 6, 6],
    ]);
    matrix.determinant().display();
}
