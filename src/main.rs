pub mod solution;

include!("./solution.rs");

fn main() {
    let test1 = vec![2, 6, 4, 1];
    let test2: Vec<i32> = vec![1,2,34,3,4,5,7,23,12];
    let result = Solution::three_consecutive_odds(test1);
    println!("Result test 1: {}", result);

    let result = Solution::three_consecutive_odds(test2);
    println!("Result test 2: {}", result);
}
