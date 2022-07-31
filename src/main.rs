mod allsum;
mod rowsum;

fn main() {
    // Rowsum
    // Read the matrix from a CSV file given as input
    let m = rowsum::Matrix::new("sample.csv").unwrap();

    // Compute the sum between given coordinates:
    // (startx, starty) -> (endx, endy)
    let s = m.sum(1, 1, 3, 2);
    println!("[RowSum method] Sum: {}", s.unwrap());

    println!("---------------------");

    // Allsum
    // Read the matrix from a CSV file given as input
    let m = allsum::Matrix::new("sample.csv").unwrap();

    // Compute the sum between given coordinates:
    // (startx, starty) -> (endx, endy)
    let s = m.sum(1, 1, 3, 2);
    println!("[Allsum method] Sum: {}", s.unwrap())
}
