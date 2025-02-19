mod task_1;
mod task_2;
mod task_3;


fn main() {
    // Task 1
    let n = 20;
    println!("--- Task 1 ---");
    println!("fib({n}) = {}", task_1::fib(n));
    println!();

    // Task 2
    let n = 11;
    println!("--- Task 2 ---");
    println!("Length: {}", task_2::collatz_length(n));
    println!();

    // Task 3
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("--- Task 3 ---");
    println!("Matrix: {:#?}", matrix);
    let transposed = task_3::transpose(matrix);
    println!("Transposed matrix: {:#?}", transposed);
    println!();
}
