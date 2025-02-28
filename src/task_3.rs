fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {

    let mut res: [[i32; 3]; 3] = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            res[i][j] = matrix[j][i];
        }
    }

    res
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

pub fn demonstrate() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("--- Task 3 ---");
    println!("Матрица: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("Транспонированная матрица: {:#?}", transposed);
    println!();
}