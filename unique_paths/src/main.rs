fn main() {
    println!("Enter two numbers separated by a comma to find the number of unique paths in a grid.");
    let mut input = String::new();
    std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    
    let parts = input.split(",").collect::<Vec<&str>>();
    if parts.len() >= 2 {
        let m = parts[0].trim().parse::<i32>().expect("Failed to parse m");
        let n = parts[1].trim().parse::<i32>().expect("Failed to parse n");
        unique_paths(m, n);
    } else {
        println!("Please enter two numbers separated by a comma.");
    }
}

fn unique_paths(m: i32, n: i32) {
    let mut dp = vec![vec![0; n as usize]; m as usize];
    for i in 0..m as usize {
        dp[i][0] = 1;
    }
    for i in 0..n as usize {
        dp[0][i] = 1;
    }
    for i in 1..m as usize {
        for j in 1..n as usize {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    println!("{}", dp[m as usize - 1][n as usize - 1]);
    // let mut ans:i32 = 1;
    // for i in 0..m {
    //     ans += &n-1-i;
    //     if i > 0 {
    //         ans += &n - i;
    //     }
    //     println!("{}", &n-1-i);
    // }
    // println!("{}", ans+1);
}
