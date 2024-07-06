use std::collections::HashSet;
use itertools::Itertools;
fn main() {
    test();
}

// fn longestConsecutive(num: Vec<i32>) -> i32{
//     let num_set: HashSet<i32> = num.into_iter();
//     println!("num_set: {:?}", num_set);
// }

pub fn test() -> i32 {
    let vec: Vec<i32> = vec![100, 4, 200, 1, 3, 2];
    // let vec2 = vec.into_iter().sorted();
    let num_set: HashSet<i32> = vec.into_iter().sorted().collect();
    
    let mut ans = 0;
    
    for &num in &num_set {
        // 找到連續數列的頭
        if !num_set.contains(&(num - 1)) {
            let count= (num..).take_while(|n| num_set.contains(n)).count();
            ans = ans.max(count);
        }
    }
    println!("ans: {}", ans);
    return ans as i32;
}