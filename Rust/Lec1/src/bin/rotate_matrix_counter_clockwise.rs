fn main() {
    let matrix: [[i32; 4]; 4] = [
        [1, 2, 3, 4],
        [2, 3, 4, 5],
        [3, 4, 5, 6],
        [4, 5, 6, 7],
    ];
    let mut rotated: [[i32; 4]; 4] = [[0; 4]; 4];
    let n = matrix.len();
    for i in 0..n{
        for j in 0..n{
            rotated[n-1-j][i] = matrix[i][j];
        }
    } 
        for row in &rotated{
        for val in row{
            print!("{:2}",val);
        }
         println!();
    }
}
