// fn main() {
//     let series = vec![1, 1, 2, 3, 5, 8, 13, 21];
//     let mut ans: i32;
//     let mut iter = series.iter().enumerate();
//     let mut hleft = iter.next();
//     let mut hright = iter.next_back();
//     while let (Some((i, h1)),Some((j, h2))) = (hleft, hright) {
//         ans = ans.max(h1.min(h2)*(j-i) as i32);
//         if h1 < h2 {
//             hleft = iter.next();
//         } else {
//             hright = iter.next_back();
//         }
//     }
// }

fn main() {
    let s = String::from("hello");
 
    let len = calculate_length(&s);
 
    println!("The length of '{s}' is {len}.");
}
 
fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of a String.
    length
}