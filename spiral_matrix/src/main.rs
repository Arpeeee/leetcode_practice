// Title: Spiral Matrix
fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut matrix2 = vec![vec![7],vec![9],vec![6]];
    let ans = spiral_matrix(matrix);
    println!("{:?}", ans);
    // println!("{:?}", matrix);
    // println!("{:?}", matrix.iter().any(|sub_vec| sub_vec.is_empty()));
}

pub fn spiral_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix1 = matrix;
        let mut ans: Vec<i32> = Vec::new();
        let mut direction: i32 = 0;
        loop {
            if matrix1.iter().all(|sub_vec| sub_vec.is_empty()) {
                break;
            }
            
            match direction {
                0 => {
                    ans.extend(matrix1.remove(0));
                    direction = 1;
                },
                1 => {
                    ans.extend(matrix1.iter_mut().map(|row| row.pop().unwrap()));
                    direction = 2;
                    println!("{:?}", matrix1);
                    println!("{}", matrix1[0].is_empty());
                },
                2 => {
                    // matrix1.last();
                    ans.extend(matrix1.pop().unwrap().into_iter().rev());
                    direction = 3;
                },
                3 => {
                    ans.extend(matrix1.iter_mut().rev().map(|row| row.remove(0)));
                    direction = 0;
                },
                _ => {
                    println!("Error");
                }
            }
            
        }
        return ans;
}