enum MatrixTypes {
    Zero((u16, u16)),
    Identity(u16),
    Scalar(u16, i128),
}
#[derive(Debug)]
struct Matrix {
    elements: Vec<Vec<i128>>,
}

//Matrix Intitialisation
impl Matrix {
    fn default() -> Self {
        Self { elements: vec![] }
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

    fn get_order(&self) -> (u16, u16) {
        (self.elements.len() as u16, self.elements[0].len() as u16)
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
            MatrixTypes::Zero(t) => {
                for k in 0..t.0 {
                    matrix.elements.push(vec![]);
                    for _l in 0..t.1 {
                        matrix.elements[(k) as usize].push(0);
                    }
                }
            }

            MatrixTypes::Identity(i) => {
                for k in 0..i {
                    matrix.elements.push(vec![]);
                    for l in 0..i {
                        if k == l {
                            matrix.elements[k as usize].push(1);
                        } else {
                            matrix.elements[k as usize].push(0);
                        }
                    }
                }
            }

            MatrixTypes::Scalar(i, j) => {
                for k in 0..i {
                    matrix.elements.push(vec![]);
                    for l in 0..i {
                        if k == l {
                            matrix.elements[k as usize].push(j);
                        } else {
                            matrix.elements[k as usize].push(0);
                        }
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
        determinant✅
        transposev✅
        minor✅
        cofactor✅
        adjoint
        inverse
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
    fn addition(matrix1: &Matrix, matrix2: &Matrix) -> Result<Matrix, String> {
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
    fn subtraction(matrix1: &Matrix, matrix2: &Matrix) -> Result<Matrix, String> {
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

    fn minor(&self) -> Matrix {
        let mut minor_matrix = Matrix::default();

        for i in 0..self.get_num_row() {
            minor_matrix.elements.push(vec![]);
            for j in 0..self.get_num_col() {
                let mut matrix = Matrix::default();
                let mut row = 0;
                for k in 0..self.get_num_row() {
                    if k == i {
                        continue;
                    }
                    matrix.elements.push(vec![]);
                    for l in 0..self.get_num_col() {
                        if l == j {
                            continue;
                        }
                        matrix.elements[row].push(self.elements[k][l]);
                    }

                    row += 1;
                }
                minor_matrix.elements[i].push(matrix.determinant());
            }
        }

        return minor_matrix;
    }

    fn cofactor(&self) -> Matrix {
        let minor = self.minor();
        let mut cofactor = Matrix::new_preset(MatrixTypes::Zero(self.get_order()));
        for i in 0..minor.get_num_row() {
            for j in 0..minor.get_num_col() {
                cofactor.elements[i][j] =
                    (-1 as i128).pow((i + j).try_into().unwrap()) * minor.elements[i][j];
            }
        }
        cofactor
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

    //Returns the transpose of the matrix
    fn transpose(&self) -> Matrix {
        let mut matrix = Matrix::new_preset(MatrixTypes::Zero(self.get_order()));

        for i in 0..self.get_num_row() {
            for j in 0..self.get_num_col() {
                matrix.elements[j][i] = self.elements[i][j];
            }
        }
        println!("{} {}", matrix.get_num_row(), self.get_num_col());
        return matrix;
    }
}

fn main() {
    let matrix = Matrix::construct(vec![vec![9, 7, 8], vec![0, 3, 4], vec![4, 5, 1]]);
    matrix.display();
    let cofactor = matrix.cofactor();
    cofactor.display();
}
