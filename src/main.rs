use operations::operations as funcs;
fn main() {
    let matrix = funcs::Matrix::construct(vec![
        vec![1, 2, 6, 7, 76],
        vec![6, 3, 6, 0, 4],
        vec![4, 6, 7, 4, 2],
        vec![12, 4, 8, 4, 10],
        vec![4, 69, 420, 56, 9],
    ]);
    matrix.display();
}
