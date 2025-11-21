fn main() {
    let matrix: [[i32; 4]; 4] = [[1, 2, 3, 4], [2, 3, 4, 5], [3, 4, 5, 6], [4, 5, 6, 7]];
    println!("Matrix is: ");
    for row in &matrix {
        for val in row {
            print!("{:2}", val);
        }
        println!();
    }
}
